// --- Unsafety utilities start here ---

use std::mem::MaybeUninit;
use std::fmt::{ Debug, Formatter, Result as FmtResult };

pub type Iter<'a, T> = std::iter::Take<std::slice::Iter<'a, T>>;
pub type IterMut<'a, T> = std::iter::Take<std::slice::IterMut<'a, T>>;

pub struct PartialArray<T, const N: usize> {
    inner: [T; N],
    len: usize
}

pub struct PartialHeapArray<T, const N: usize> {
    inner: Box<[T; N]>,
    len: usize
}

impl<T, const N: usize> PartialArray<T, N> {
    pub fn new() -> Self {
        let arr = unsafe {
            // Why not MaybeUninit<T> too? You may ask. That's because rustc is stupid and
            // programmers CANNOT, in any conceivable way, transmute a [MaybeUninit<T>; N]
            // into [T; N] because rustc is stupid (yes, I said that TWICE.)
            //
            // This is EXTREMELY frustrating. Hours wasted just to think of a workaround.
            // In the end I decided to just ditch MaybeUninit for the most part and handle
            // things myself.
            MaybeUninit::<[T; N]>::uninit().assume_init()
        };

        Self {
            inner: arr,
            len: 0,
        }
    }

    pub fn push(&mut self, elem: T) -> Result<(), T> {
        if self.len >= N {
            return Err(elem)
        };

        unsafe {
            let ptr = self.inner.as_mut_ptr().offset(self.len as isize);
            std::ptr::write(ptr, elem) 
        };
        self.len += 1;
        Ok(())
    }

    pub fn get_ref(&self, index: usize) -> Option<&T> {
        if index < self.len {
            Some(&self.inner[index])
        } else {
            None
        }
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        if index < self.len {
            Some(&mut self.inner[index])
        } else {
            None
        }
    }

    pub fn iter(&self) -> Iter<'_, T> {
        self.inner.iter().take(self.len)
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        self.inner.iter_mut().take(self.len)
    }

    pub fn into_full_array(self) -> Result<[T; N], Self> {
        if self.len == N {
            Ok(self.inner)
        } else {
            Err(self)
        }
    }
}

impl<T: Debug, const N: usize> Debug for PartialArray<T, N> {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "[")?;

        for x in 0..N {
            match self.get_ref(x) {
                Some(x) => write!(f, "{:?}", x)?,
                None => write!(f, "{{uninit}}")?
            };

            if x == N - 1 {
                write!(f, "]")?;
            } else {
                write!(f, ", ")?;
            }
        }

        Ok(())
    }
}

impl<T, const N: usize> PartialHeapArray<T, N> {
    pub fn new() -> Self {
        let arr = unsafe {
            Box::new(MaybeUninit::uninit().assume_init())
        };

        Self {
            inner: arr,
            len: 0,
        }
    }

    pub fn push(&mut self, elem: T) -> Result<(), T> {
        if self.len >= N {
            return Err(elem)
        };

        unsafe {
            let ptr = self.inner.as_mut_ptr().offset(self.len as isize);
            std::ptr::write(ptr, elem) 
        };
        self.len += 1;
        Ok(())
    }

    pub fn get_ref(&self, index: usize) -> Option<&T> {
        if index < self.len {
            Some(&self.inner[index])
        } else {
            None
        }
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        if index < self.len {
            Some(&mut self.inner[index])
        } else {
            None
        }
    }

    pub fn iter(&self) -> Iter<'_, T> {
        self.inner.iter().take(self.len)
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        self.inner.iter_mut().take(self.len)
    }

    pub fn into_full_array(self) -> Result<Box<[T; N]>, Self> {
        if self.len == N {
            Ok(self.inner)
        } else {
            Err(self)
        }
    }
}

impl<T: Debug, const N: usize> Debug for PartialHeapArray<T, N> {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "[")?;

        for x in 0..N {
            match self.get_ref(x) {
                Some(x) => write!(f, "{:?}", x)?,
                None => write!(f, "{{uninit}}")?
            };

            if x == N - 1 {
                write!(f, "]")?;
            } else {
                write!(f, ", ")?;
            }
        }

        Ok(())
    }
}
