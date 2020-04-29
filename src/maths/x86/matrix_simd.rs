use crate::maths::matrix::*;
use crate::maths::vector::*;

macro_rules! mask {
    ($a:expr, $b:expr, $c:expr, $d:expr) => {
        $a | ($b << 2) | ($c << 4) | ($d << 6)
    }
}

impl Matrix2 {
    #[cfg(all(
        any(target_arch = "x86", target_arch = "x86_64"),
        target_feature = "sse2"
    ))]
    pub fn transpose(self) -> Self {
        #[cfg(target_arch = "x86")]
        use std::arch::x86 as intrin;
        #[cfg(target_arch = "x86_64")]
        use std::arch::x86_64 as intrin;

        let mut s = self;

        let result = unsafe {
            let xmm = intrin::_mm_load_ps(s.0.as_ptr() as *const f32);
            let xmm = intrin::_mm_permute_ps(xmm, mask!(0, 2, 1, 3));
            intrin::_mm_store_ps(s.0.as_mut_ptr() as *mut f32, xmm);
            s
        };

        result
    }
}

impl Matrix3 {
    #[cfg(all(
        any(target_arch = "x86", target_arch = "x86_64"),
        target_feature = "sse2"
    ))]
    pub fn transpose(self) -> Self {
        #[cfg(target_arch = "x86")]
        use std::arch::x86 as intrin;
        #[cfg(target_arch = "x86_64")]
        use std::arch::x86_64 as intrin;

        let mut s = self;
        let result = unsafe {
            // We leave the last one alone(self[2][2]) since its position won't
            // change after transposing the matrix
            let xmm = [
                intrin::_mm_load_ps(&(s.0)[0][0]),
                intrin::_mm_load_ps(&(s.0)[1][1]),
            ];
        
            let s0 = intrin::_mm_shuffle_ps(xmm[0], xmm[1], mask!(1, 2, 2, 3));
            
            let r0 = intrin::_mm_shuffle_ps(xmm[0], s0, mask!(0, 3, 2, 0));
            let r1 = intrin::_mm_shuffle_ps(xmm[1], s0, mask!(0, 1, 3, 1));
            let r1 = intrin::_mm_shuffle_ps(r1, r1, mask!(0, 2, 3, 1));
            
            intrin::_mm_store_ps(&mut s.0[0][0], r0);
            intrin::_mm_store_ps(&mut s.0[1][1], r1);

            s
        };

        result
    }
}

impl Matrix4 {
    #[cfg(all(
        any(target_arch = "x86", target_arch = "x86_64"),
        target_feature = "sse2"
    ))]
    pub fn transpose(self) -> Self {
        #[cfg(target_arch = "x86")]
        use std::arch::x86 as intrin;
        #[cfg(target_arch = "x86_64")]
        use std::arch::x86_64 as intrin;

        let mut s = self;

        let result = unsafe {
            let get_ptr = |x: &Vector4F| x.as_ptr() as *const f32;
            let get_mut_ptr = |x: &mut Vector4F| x.as_mut_ptr() as *mut f32;

            let cols = [
                intrin::_mm_load_ps(get_ptr(&(s.0)[0])),
                intrin::_mm_load_ps(get_ptr(&(s.0)[1])),
                intrin::_mm_load_ps(get_ptr(&(s.0)[2])),
                intrin::_mm_load_ps(get_ptr(&(s.0)[3])),
            ];

            let [mut cols0, mut cols1, mut cols2, mut cols3] = cols;
            intrin::_MM_TRANSPOSE4_PS(
                &mut cols0,
                &mut cols1,
                &mut cols2,
                &mut cols3
            );

            intrin::_mm_store_ps(get_mut_ptr(&mut s.0[0]), cols0);
            intrin::_mm_store_ps(get_mut_ptr(&mut s.0[1]), cols1);
            intrin::_mm_store_ps(get_mut_ptr(&mut s.0[2]), cols2);
            intrin::_mm_store_ps(get_mut_ptr(&mut s.0[3]), cols3);
            
            s
        };

        result
    }
}
