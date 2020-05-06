use std::ops::*;

// unused:
// #[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
// pub struct Scalar(pub f32);

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Vector2D([f64; 2]);

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Vector3D([f64; 3]);

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Vector4D([f64; 4]);

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Vector2F([f32; 2]);

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Vector3F([f32; 3]);

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Vector4F([f32; 4]);

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub struct Vector2I([i32; 2]);

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub struct Vector3I([i32; 3]);

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub struct Vector4I([i32; 4]);

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub struct Vector2L([i64; 2]);

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub struct Vector3L([i64; 3]);

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub struct Vector4L([i64; 4]);


impl Vector3F {
    pub fn vec2_new(xy: Vector2F, z: f32) -> Self {
        Self([xy.x(), xy.y(), z])
    }

    pub fn new_vec2(x: f32, yz: Vector2F) -> Self {
        Self([x, yz.x(), yz.y()])
    }

    pub fn cross(self, rhs: Vector3F) -> Self {
        let a = self[2] * rhs[1];
        let b = self[0] * rhs[2];
        let c = self[1] * rhs[0];
        
        Vector3F([
            rhs[2] * self[1] - a,
            rhs[0] * self[2] - b,
            rhs[1] * self[0] - c,
        ])
    }

    pub fn trunc2(self) -> Vector2F {
        Vector2F([(self.0)[0], (self.0)[1]])
    }
}

impl Vector4F {
    pub fn trunc2(self) -> Vector2F {
        Vector2F([(self.0)[0], (self.0)[1]])
    }

    pub fn trunc3(self) -> Vector3F {
        Vector3F([(self.0)[0], (self.0)[1], (self.0)[2]])
    }
}



// ------
// Macro shenanigans begin here
// ------
macro_rules! impl_as_ref_mut {
    ($name:ident, $refs:ty) => {
        impl AsRef<$refs> for $name {
            fn as_ref(&self) -> &$refs {
                &self.0
            }
        }

        impl AsMut<$refs> for $name {
            fn as_mut(&mut self) -> &mut $refs {
                &mut self.0
            }
        }
    };
}

macro_rules! impl_scalar_arith {
    (+ $lhs:path, $rhs:ty) => {
        impl Add<$rhs> for $lhs {
            type Output = Self;
        
            fn add(self, rhs: $rhs) -> Self {
                let mut s = self.0;
                for x in s.iter_mut() {
                    *x += rhs;
                };
                Self(s)
            }
        }
    };
    
    (- $lhs:path, $rhs:ty) => {
        impl Sub<$rhs> for $lhs {
            type Output = Self;
        
            fn sub(self, rhs: $rhs) -> Self {
                let mut s = self.0;
                for x in s.iter_mut() {
                    *x -= rhs;
                };
                Self(s)
            }
        }
    };

    (* $lhs:path, $rhs:ty) => {
        impl Mul<$rhs> for $lhs {
            type Output = Self;
        
            fn mul(self, rhs: $rhs) -> Self {
                let mut s = self.0;
                for x in s.iter_mut() {
                    *x *= rhs;
                };
                Self(s)
            }
        }

        impl Mul<$lhs> for $rhs {
            type Output = $lhs;
        
            fn mul(self, rhs: $lhs) -> Self::Output {
                let mut s = rhs.0;
                for x in s.iter_mut() {
                    *x *= self;
                };

                $lhs(s)
            }
        }
    };

    (/ $lhs:path, $rhs:ty) => {
        impl Div<$rhs> for $lhs {
            type Output = Self;
        
            fn div(self, rhs: $rhs) -> Self {
                let mut s = self.0;
                for x in s.iter_mut() {
                    *x /= rhs;
                };
                Self(s)
            }
        }
    };

    (& $lhs:path, $rhs:ty) => {
        impl BitAnd<$rhs> for $lhs {
            type Output = Self;
        
            fn bitand(self, rhs: $rhs) -> Self {
                let mut s = self.0;
                for x in s.iter_mut() {
                    *x &= rhs;
                };
                Self(s)
            }
        }
    };

    (| $lhs:path, $rhs:ty) => {
        impl BitOr<$rhs> for $lhs {
            type Output = Self;
        
            fn bitor(self, rhs: $rhs) -> Self {
                let mut s = self.0;
                for x in s.iter_mut() {
                    *x |= rhs;
                };
                Self(s)
            }
        }
    };
}

macro_rules! impl_vector_arith {
    (+ $lhs:ty) => {
        impl Add<$lhs> for $lhs {
            type Output = Self;
        
            fn add(self, rhs: $lhs) -> Self {
                let mut s = self.0;
                for (i, x) in s.iter_mut().enumerate() {
                    *x += (rhs.0)[i];
                };
                Self(s)
            }
        }
    };
    
    (- $lhs:ty) => {
        impl Sub<$lhs> for $lhs {
            type Output = Self;
        
            fn sub(self, rhs: $lhs) -> Self {
                let mut s = self.0;
                for (i, x) in s.iter_mut().enumerate() {
                    *x -= (rhs.0)[i];
                };
                Self(s)
            }
        }
    };
    
    (* $lhs:ty) => {
        impl Mul<$lhs> for $lhs {
            type Output = Self;
        
            fn mul(self, rhs: $lhs) -> Self {
                let mut s = self.0;
                for (i, x) in s.iter_mut().enumerate() {
                    *x *= (rhs.0)[i];
                };
                Self(s)
            }
        }
    };
    
    (/ $lhs:ty) => {
        impl Div<$lhs> for $lhs {
            type Output = Self;
        
            fn div(self, rhs: $lhs) -> Self {
                let mut s = self.0;
                for (i, x) in s.iter_mut().enumerate() {
                    *x /= (rhs.0)[i];
                };
                Self(s)
            }
        }
    };
}

macro_rules! impl_assign {
    (+= $lhs:ty, $rhs:ty) => {
        impl AddAssign<$rhs> for $lhs {
            fn add_assign(&mut self, rhs: $rhs) {
                *self = *self + rhs;
            }
        }
    };

    (-= $lhs:ty, $rhs:ty) => {
        impl SubAssign<$rhs> for $lhs {
            fn sub_assign(&mut self, rhs: $rhs) {
                *self = *self - rhs;
            }
        }
    };

    (*= $lhs:ty, $rhs:ty) => {
        impl MulAssign<$rhs> for $lhs {
            fn mul_assign(&mut self, rhs: $rhs) {
                *self = *self * rhs;
            }
        }
    };

    (/= $lhs:ty, $rhs:ty) => {
        impl DivAssign<$rhs> for $lhs {
            fn div_assign(&mut self, rhs: $rhs) {
                *self = *self / rhs;
            }
        }
    };
}

macro_rules! impl_index {
    ($vec:ty, $index:ty, $output:ty) => {
        impl Index<$index> for $vec {
            type Output = $output;

            fn index(&self, idx: $index) -> &Self::Output {
                self.0.index(idx)
            }
        }

        impl IndexMut<$index> for $vec {
            fn index_mut(&mut self, idx: $index) -> &mut Self::Output {
                self.0.index_mut(idx)
            }
        }
    };
}

macro_rules! impl_int_float_conversion {
    ($lhs:ty, $rhs:ty, $inner_int:ty, $inner_float:ty) => {
        impl From<$lhs> for $rhs {
            fn from(l: $lhs) -> $rhs {
                let mut result = <$rhs>::zeroed();
                for (i, x) in l.as_ref().iter().enumerate() {
                    result[i] = *x as $inner_int;
                }
                result
            }
        }

        impl From<$rhs> for $lhs {
            fn from(l: $rhs) -> $lhs {
                let mut result = <$lhs>::zeroed();
                for (i, x) in l.as_ref().iter().enumerate() {
                    result[i] = *x as $inner_float;
                }
                result
            }
        }
    };
}

macro_rules! impl_vector {
    (
        // Vector name
        $vec:ident,
        // Vector type
        $inner:ty,
        // Vector calculation type (e.g. f32 in `(i as f32).sqrt() as i32`)
        $calc:ty,
        // Vector size (in terms of components)
        $size:expr,
        // Vector getter/setter/order, axis orders are x=0,y=1,z=2,w=3
        // syntax: [<axis>(set_<axis>): <axis order>, ..]
        [$($comp:ident ($comp_mut:ident, $setter:ident): $order:expr),*]
    ) => {
        impl $vec {
            // Produces VectorN::new(x, y, ...)
            pub const fn new($($comp: $inner),*) -> Self {
                Self([$($comp),*])
            }

            pub const fn zeroed() -> Self {
                Self([
                    $({
                        // to avoid expr repeat without repeating macro vars
                        let _ = $order;
                        0 as $inner
                    }),*
                ])
            }

            pub const fn from_array(arr: [$inner; $size]) -> Self {
                Self(arr)
            }
        
            pub fn from_slice(arr: &[$inner]) -> Self {
                Self([$(arr[$order]),*])
            }

            pub fn as_ptr(&self) -> *const $inner {
                self.0.as_ptr()
            }
        
            pub fn as_mut_ptr(&mut self) -> *mut $inner {
                self.0.as_mut_ptr()
            }

            pub fn dot(self, rhs: Self) -> $inner {
                (self * rhs).sum()
            }
        
            pub fn magnitude(self) -> $inner {
                (self.dot(self) as $calc).sqrt() as $inner
            }
        
            pub fn normalize(self) -> Self {
                self / self.magnitude()
            }
        
            pub fn sum(self) -> $inner {
                (self.0).iter().fold(0 as $inner, |acc, &x| acc + x)
            }
        
            pub fn shuffle(self, shuf: [usize; $size]) -> Self {
                let mut new = Self::zeroed();
                for (i, s) in shuf.iter().enumerate() {
                    new[i] = (self.0)[*s];
                }
                new
            }
        
            pub const fn trunc1(self) -> $inner {
                (self.0)[0]
            }

            // Here comes the getters!
            $(pub const fn $comp(self) -> $inner {
                (self.0)[$order]
            })*

            // Here comes the mutables!
            $(pub fn $comp_mut(&mut self) -> &mut $inner {
                &mut (self.0)[$order]
            })*

            // And here comes the setters!
            $(pub fn $setter(&mut self, value: $inner) {
                self[$order] = value
            })*
        }

        impl_as_ref_mut!($vec, [$inner; $size]);
        
        impl_scalar_arith!(+ $vec, $inner);
        impl_scalar_arith!(- $vec, $inner);
        impl_scalar_arith!(* $vec, $inner);
        impl_scalar_arith!(/ $vec, $inner);

        impl_vector_arith!(+ $vec);
        impl_vector_arith!(- $vec);
        impl_vector_arith!(* $vec);
        impl_vector_arith!(/ $vec);

        impl_assign!(+= $vec, $inner);
        impl_assign!(+= $vec, $vec);
        impl_assign!(-= $vec, $inner);
        impl_assign!(-= $vec, $vec);
        impl_assign!(*= $vec, $inner);
        impl_assign!(*= $vec, $vec);
        impl_assign!(/= $vec, $inner);
        impl_assign!(/= $vec, $vec);

        impl_index!($vec, usize, $inner);
    };
}

impl_vector!(Vector2D, f64, f64, 2, [x(x_mut, set_x): 0, y(y_mut, set_y): 1                                        ]);
impl_vector!(Vector3D, f64, f64, 3, [x(x_mut, set_x): 0, y(y_mut, set_y): 1, z(z_mut, set_z): 2                    ]);
impl_vector!(Vector4D, f64, f64, 4, [x(x_mut, set_x): 0, y(y_mut, set_y): 1, z(z_mut, set_z): 2, w(w_mut, set_w): 3]);
impl_vector!(Vector2F, f32, f32, 2, [x(x_mut, set_x): 0, y(y_mut, set_y): 1                                        ]);
impl_vector!(Vector3F, f32, f32, 3, [x(x_mut, set_x): 0, y(y_mut, set_y): 1, z(z_mut, set_z): 2                    ]);
impl_vector!(Vector4F, f32, f32, 4, [x(x_mut, set_x): 0, y(y_mut, set_y): 1, z(z_mut, set_z): 2, w(w_mut, set_w): 3]);
impl_vector!(Vector2I, i32, f64, 2, [x(x_mut, set_x): 0, y(y_mut, set_y): 1                                        ]);
impl_vector!(Vector3I, i32, f64, 3, [x(x_mut, set_x): 0, y(y_mut, set_y): 1, z(z_mut, set_z): 2                    ]);
impl_vector!(Vector4I, i32, f64, 4, [x(x_mut, set_x): 0, y(y_mut, set_y): 1, z(z_mut, set_z): 2, w(w_mut, set_w): 3]);
impl_vector!(Vector2L, i64, f64, 2, [x(x_mut, set_x): 0, y(y_mut, set_y): 1                                        ]);
impl_vector!(Vector3L, i64, f64, 3, [x(x_mut, set_x): 0, y(y_mut, set_y): 1, z(z_mut, set_z): 2                    ]);
impl_vector!(Vector4L, i64, f64, 4, [x(x_mut, set_x): 0, y(y_mut, set_y): 1, z(z_mut, set_z): 2, w(w_mut, set_w): 3]);

// Special case for integer vectors
impl_scalar_arith!(& Vector2I, i32);
impl_scalar_arith!(& Vector3I, i32);
impl_scalar_arith!(& Vector4I, i32);
impl_scalar_arith!(| Vector2I, i32);
impl_scalar_arith!(| Vector3I, i32);
impl_scalar_arith!(| Vector4I, i32);
impl_scalar_arith!(& Vector2L, i64);
impl_scalar_arith!(& Vector3L, i64);
impl_scalar_arith!(& Vector4L, i64);
impl_scalar_arith!(| Vector2L, i64);
impl_scalar_arith!(| Vector3L, i64);
impl_scalar_arith!(| Vector4L, i64);

// Conversion between int/float vectors
// f32 <-> i32, f32 <-> i64
impl_int_float_conversion!(Vector2F, Vector2I, i32, f32);
impl_int_float_conversion!(Vector3F, Vector3I, i32, f32);
impl_int_float_conversion!(Vector4F, Vector4I, i32, f32);
impl_int_float_conversion!(Vector2F, Vector2L, i64, f32);
impl_int_float_conversion!(Vector3F, Vector3L, i64, f32);
impl_int_float_conversion!(Vector4F, Vector4L, i64, f32);

// f64 <-> i32, f64 <-> i64
impl_int_float_conversion!(Vector2D, Vector2I, i32, f64);
impl_int_float_conversion!(Vector3D, Vector3I, i32, f64);
impl_int_float_conversion!(Vector4D, Vector4I, i32, f64);
impl_int_float_conversion!(Vector2D, Vector2L, i64, f64);
impl_int_float_conversion!(Vector3D, Vector3L, i64, f64);
impl_int_float_conversion!(Vector4D, Vector4L, i64, f64);

// i32 <-> i64, f32 <-> f64
impl_int_float_conversion!(Vector2I, Vector2L, i64, i32);
impl_int_float_conversion!(Vector3I, Vector3L, i64, i32);
impl_int_float_conversion!(Vector4I, Vector4L, i64, i32);
impl_int_float_conversion!(Vector2F, Vector2D, f64, f32);
impl_int_float_conversion!(Vector3F, Vector3D, f64, f32);
impl_int_float_conversion!(Vector4F, Vector4D, f64, f32);