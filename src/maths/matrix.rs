use crate::maths::*;

use std::ops::{ Add, Sub, Mul };
use std::ops::{ AddAssign, SubAssign, MulAssign };
use std::ops::{ Index, IndexMut };

use super::{ Vector2F, Vector3F, Vector4F };

#[repr(align(16))]
#[derive(Clone, Debug, PartialEq)]
pub struct Matrix2(pub(in super) [Vector2F; 2]);

#[repr(align(16))]
#[derive(Clone, Debug, PartialEq)]
pub struct Matrix3(pub(in super) [Vector3F; 3]);

#[repr(align(16))]
#[derive(Clone, Debug, PartialEq)]
pub struct Matrix4(pub(in super) [Vector4F; 4]);

impl Matrix2 {
    pub fn rotate<A>(angle: A) -> Self 
        where A: Into<Rad>
    {
        let angle = angle.into().0;
        let sin = angle.sin();
        let cos = angle.cos();
        Self([
            Vector2F::new( cos,  sin),
            Vector2F::new(-sin,  cos)
        ])
    }
}

impl Matrix3 {
    pub fn from_mat2(mat2: Matrix2) -> Self {
        let mut result = Self::new();
        for (c, col) in mat2.0.iter().enumerate() {
            for (r, row) in col.as_ref().iter().enumerate() {
                result[c][r] = *row;
            }
        };
        result
    }

    pub fn rotate_x_axis<A>(angle: A) -> Self 
        where A: Into<Rad>
    {
        let angle = angle.into().0;
        let sin = angle.sin();
        let cos = angle.cos();
        Self([
            Vector3F::new(1.0,  0.0, 0.0),
            Vector3F::new(0.0,  cos, sin),
            Vector3F::new(0.0, -sin, cos),
        ])
    }

    pub fn rotate_y_axis<A>(angle: A) -> Self 
        where A: Into<Rad>
    {
        let angle = angle.into().0;
        let sin = angle.sin();
        let cos = angle.cos();
        Self([
            Vector3F::new( cos, 0.0, sin),
            Vector3F::new( 0.0, 1.0, 0.0),
            Vector3F::new(-sin, 0.0, cos)
        ])
    }

    pub fn rotate_z_axis<A>(angle: A) -> Self 
        where A: Into<Rad>
    {
        let angle = angle.into().0;
        let sin = angle.sin();
        let cos = angle.cos();
        Self([
            Vector3F::new( cos, sin, 0.0),
            Vector3F::new(-sin, cos, 0.0),
            Vector3F::new( 0.0, 0.0, 1.0)
        ])
    }

    pub fn rotate_axis_angle<A>(axis: Vector3F, angle: A) -> Self 
        where A: Into<Rad>
    {
        let angle = angle.into().0;
        let mut result = Self::new();
        
        let axis_m = {
            let mut mat = Self::zero();

            let index = |row, col| (6 - col - row) % 3;
            let factor = |row, col| [-1.0, 1.0][(col == (row + 1) % 3) as usize];

            for (c, col) in mat.0.iter_mut().enumerate() {
                for (r, row) in col.as_mut().iter_mut().enumerate() {
                    if r != c {
                        *row = factor(r, c) * axis[index(r, c)];
                    }
                }
            };

            mat
        };
        
        result += angle.sin() * axis_m.clone() + 
            (1.0 - angle.cos()) * axis_m.clone() * axis_m;
        
        result
    }
}

impl Matrix4 {
    pub fn from_mat2(mat2: Matrix2) -> Self {
        let mut result = Self::new();
        for (c, col) in mat2.0.iter().enumerate() {
            for (r, row) in col.as_ref().iter().enumerate() {
                result[c][r] = *row;
            }
        };
        result
    }

    pub fn from_mat3(mat3: Matrix3) -> Self {
        let mut result = Self::new();
        for (c, col) in mat3.0.iter().enumerate() {
            for (r, row) in col.as_ref().iter().enumerate() {
                result[c][r] = *row;
            }
        };
        result
    }

    pub fn look_at(pos: Vector3F, target: Vector3F, up: Vector3F) -> Self {
        let cam_direction = (target - pos).normalize();
        let cam_right = cam_direction.cross(up).normalize();
        let cam_up = cam_right.cross(cam_direction);
        
        let mut mat = Matrix4::from_mat3(Matrix3([
            cam_right,
            cam_up,
            cam_direction * -1.0
        ]).transpose());
        
        mat[3] = Vector4F::new(
            -pos.dot(cam_right),
            -pos.dot(cam_up),
            pos.dot(cam_direction),
            1.0,
        );

        mat
    }

    pub fn perspective<A>(
        fov: A, 
        aspect_ratio: f32,
        near: f32,
        far: f32
    ) -> Self 
        where A: Into<Rad>
    {
        // basically the matrix described by gluPerspective()

        let fov = fov.into().0;
        let f = (fov * 0.5).tan().recip();
        // make the x-axis N times longer than y-axis; N=aspect ratio
        let g = f / aspect_ratio;
        let z_z = (far + near) / (near - far);
        let w_z = (2.0 * far * near) / (near - far);

        Self ([
            Vector4F::new(  g, 0.0, 0.0,  0.0),
            Vector4F::new(0.0,   f, 0.0,  0.0),
            Vector4F::new(0.0, 0.0, z_z, -1.0),
            Vector4F::new(0.0, 0.0, w_z,  0.0),
        ])
    }

    pub fn rotate_x_axis<A>(angle: A) -> Self 
        where A: Into<Rad>
    {
        Self::from_mat3(Matrix3::rotate_x_axis(angle))
    }

    pub fn rotate_y_axis<A>(angle: A) -> Self 
        where A: Into<Rad>
    {
        Self::from_mat3(Matrix3::rotate_y_axis(angle))
    }

    pub fn rotate_z_axis<A>(angle: A) -> Self 
        where A: Into<Rad>
    {
        Self::from_mat3(Matrix3::rotate_z_axis(angle))
    }

    pub fn rotate_axis_angle<A>(axis: Vector3F, angle: A) -> Self 
        where A: Into<Rad>
    {
        Self::from_mat3(Matrix3::rotate_axis_angle(axis, angle))
    }

    pub fn translate(x: f32, y: f32, z: f32) -> Self {
        let mut result = Self::new();
        result[3][0] = x;
        result[3][1] = y;
        result[3][2] = z;
        result
    }
}

// Generic matrix implementation
// Implements common methods (new(), *_size(), etc.) for the Matrices

macro_rules! impl_matrix {
    ($name:ident, $size:expr, $inner:ty) => {
        impl $name {
            pub fn new() -> Self {
                Self::scale(1.0)
            }

            pub fn zero() -> Self {
                Self::scale(0.0)
            }

            pub fn from_array(arr: &[$inner; $size]) -> Self {
                Self(*arr)
            }

            pub fn as_array(&self) -> &[$inner; $size] {
                &self.0
            }

            pub const fn column_size() -> usize {
                $size
            }

            pub const fn row_size() -> usize {
                $size
            }

            pub const fn component_size() -> usize {
                $size * $size
            }

            pub fn as_ptr(&self) -> *const f32 {
                self.0.as_ptr() as *const f32
            }

            pub fn as_ptr_mut(&mut self) -> *mut f32 {
                self.0.as_mut_ptr() as *mut f32
            }

            pub fn scale(by: f32) -> Self {
                let mut inner = [<$inner>::from_array([0.0; $size]); $size];
                
                for i in 0..Self::column_size() {
                    inner[i][i] = by;
                }

                // We leave w alone.
                if $size == 4 {
                    // Make `inner` a slice to bypass rustc checks
                    // (when $size == 2 or 3, we are doing inner[3] which
                    // is obviously out of bounds, and rustc hates that.)
                    // This check will be optimized out for Vector2F/3 anyways.
                    let inner_mut = &mut inner[0..];
                    inner_mut[3][3] = 1.0;
                }

                Self(inner)
            }

            #[cfg(not(target_feature = "sse2"))]
            pub fn transpose(self) -> Self {
                let mut result = Self::zero();
                const N: usize = $size;
                
                for i in (0..N){
                    for j in (0..N) {
                        (result.0)[j][i] = (self.0)[i][j];
                    }
                };

                result
            }
        }
    };
}

impl_matrix!(Matrix2, 2, Vector2F);
impl_matrix!(Matrix3, 3, Vector3F);
impl_matrix!(Matrix4, 4, Vector4F);


//
// Arithmetic operator implementation
// This implements +, -, * for Matrix
// Note: multiplication has three forms: mat-scalar, mat-mat, mat-vec
//

macro_rules! impl_arithmetic_ops {
    (+ $mat:ty) => {
        impl Add<$mat> for $mat {
            type Output = Self;
        
            fn add(self, rhs: Self) -> Self {
                let mut result = Self::zero();
                let n = self.0.len();
                for (i, j) in (0..n).flat_map(|x| (0..n).map(move |y| (x, y))) {
                    (result.0)[i][j] = (self.0)[i][j] + (rhs.0)[i][j];
                };
                result
            }
        }
    };

    (- $mat:ty) => {
        impl Sub<$mat> for $mat {
            type Output = Self;
        
            fn sub(self, rhs: Self) -> Self {
                let mut result = Self::zero();
                let n = self.0.len();
                for (i, j) in (0..n).flat_map(|x| (0..n).map(move |y| (x, y))) {
                    (result.0)[i][j] = (self.0)[i][j] - (rhs.0)[i][j];
                };
                result
            }
        }
    };

    (* scalar $mat:ty, $rhs:ty) => {
        impl Mul<$rhs> for $mat {
            type Output = Self;
        
            fn mul(self, rhs: $rhs) -> Self {
                let mut result = self;
                let n = result.0.len();
                for (i, j) in (0..n).flat_map(|x| (0..n).map(move |y| (x, y))) {
                    (result.0)[i][j] *= rhs;
                };
                result
            }
        }

        impl Mul<$mat> for $rhs {
            type Output = $mat;
        
            fn mul(self, rhs: $mat) -> $mat {
                let mut result = rhs;
                let n = result.0.len();
                for (i, j) in (0..n).flat_map(|x| (0..n).map(move |y| (x, y))) {
                    (result.0)[i][j] *= self;
                };
                result
            }
        }
    };
    
    (* vector $mat:ty) => {
        impl Mul<$mat> for $mat {
            type Output = Self;
        
            fn mul(self, rhs: Self) -> Self {
                let mut result = Self::zero();
                let s = self.transpose();
                for (r, row) in s.0.iter().enumerate() {
                    for (c, col) in rhs.0.iter().enumerate() {
                        (result.0)[c][r] = row.dot(*col);
                    }
                };
                result
            }
        }
    };

    (* vector $mat:ty, $vec:ty) => {
        impl Mul<$vec> for $mat {
            type Output = $vec;
        
            fn mul(self, rhs: $vec) -> $vec {
                let mut result = [0.0; Self::column_size()];
                let s = self.transpose();
                for (r, row) in s.0.iter().enumerate() {
                    result[r] = row.dot(rhs);
                };
                
                <$vec>::from_array(result)
            }
        }
    };
}

impl_arithmetic_ops!(+ Matrix2);
impl_arithmetic_ops!(+ Matrix3);
impl_arithmetic_ops!(+ Matrix4);
impl_arithmetic_ops!(- Matrix2);
impl_arithmetic_ops!(- Matrix3);
impl_arithmetic_ops!(- Matrix4);
impl_arithmetic_ops!(* scalar Matrix2 , f32);
impl_arithmetic_ops!(* scalar Matrix3 , f32);
impl_arithmetic_ops!(* scalar Matrix4 , f32);
impl_arithmetic_ops!(* vector Matrix2);
impl_arithmetic_ops!(* vector Matrix3);
impl_arithmetic_ops!(* vector Matrix4);
impl_arithmetic_ops!(* vector Matrix2 , Vector2F);
impl_arithmetic_ops!(* vector Matrix3 , Vector3F);
impl_arithmetic_ops!(* vector Matrix4 , Vector4F);


//
// Index[Mut] implementation
// This implements indexing for Matrix
//

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

impl_index!(Matrix2, usize, Vector2F);
impl_index!(Matrix3, usize, Vector3F);
impl_index!(Matrix4, usize, Vector4F);


//
// [Op]Assign implementation
// This implements +=, -=, *= for Matrix
//

macro_rules! impl_assign {
    (+ $lhs:ty, $rhs:ty) => {
        impl AddAssign<$rhs> for $lhs {
            fn add_assign(&mut self, rhs: $rhs) {
                *self = self.clone() + rhs;
            }
        }
    };

    (- $lhs:ty, $rhs:ty) => {
        impl SubAssign<$rhs> for $lhs {
            fn sub_assign(&mut self, rhs: $rhs) {
                *self = self.clone() - rhs;
            }
        }
    };

    (* $lhs:ty, $rhs:ty) => {
        impl MulAssign<$rhs> for $lhs {
            fn mul_assign(&mut self, rhs: $rhs) {
                *self = self.clone() * rhs;
            }
        }
    };
}

impl_assign!(* Matrix2, f32);
impl_assign!(* Matrix3, f32);
impl_assign!(* Matrix4, f32);
impl_assign!(+ Matrix2, Matrix2);
impl_assign!(+ Matrix3, Matrix3);
impl_assign!(+ Matrix4, Matrix4);
impl_assign!(- Matrix2, Matrix2);
impl_assign!(- Matrix3, Matrix3);
impl_assign!(- Matrix4, Matrix4);
impl_assign!(* Matrix2, Matrix2);
impl_assign!(* Matrix3, Matrix3);
impl_assign!(* Matrix4, Matrix4);
