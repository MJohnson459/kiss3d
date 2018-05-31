//! Structures that a gpu buffer may contain.

use context::{self, Context, UniformLocation};
use std::{mem, slice};

use na::{Matrix2, Matrix3, Matrix4, Point2, Point3, Rotation2, Rotation3, Vector2, Vector3};

#[path = "../error.rs"]
mod error;

pub enum PrimitiveArray<'a> {
    Float32(&'a [f32]),
    Int32(&'a [i32]),
}

/// Trait implemented by structures that can be uploaded to a uniform or contained by a gpu array.
pub unsafe trait GLPrimitive: Copy {
    /// The Opengl primitive type of this structure content.
    fn gl_type() -> u32;
    /// The number of elements of type `self.gl_type()` this structure stores.
    fn size() -> u32;
    /// Uploads the element to a gpu location.
    fn upload(&self, location: &UniformLocation);
    /// Converts an array of `Self` into an array of f32 or i32 primitives.
    fn flatten(array: &[Self]) -> PrimitiveArray {
        unsafe {
            let len = array.len() * Self::size() as usize;
            let ptr = array.as_ptr();

            PrimitiveArray::Float32(slice::from_raw_parts(ptr as *const f32, len))
        }
    }
}

/*
 *
 * Impl for primitive types
 *
 */
unsafe impl GLPrimitive for f32 {
    #[inline]
    fn gl_type() -> u32 {
        Context::FLOAT
    }

    #[inline]
    fn size() -> u32 {
        1
    }

    #[inline]
    fn flatten(array: &[Self]) -> PrimitiveArray {
        unsafe {
            let len = array.len() * Self::size() as usize;
            let ptr = array.as_ptr();

            PrimitiveArray::Float32(slice::from_raw_parts(ptr as *const f32, len))
        }
    }

    #[inline]
    fn upload(&self, location: &UniformLocation) {
        verify!(Context::get().uniform1f(Some(location), self.clone()));
    }
}

unsafe impl GLPrimitive for i32 {
    #[inline]
    fn gl_type() -> u32 {
        Context::INT
    }

    #[inline]
    fn size() -> u32 {
        1
    }

    fn flatten(array: &[Self]) -> PrimitiveArray {
        unsafe {
            let len = array.len() * Self::size() as usize;
            let ptr = array.as_ptr();

            PrimitiveArray::Int32(slice::from_raw_parts(ptr, len))
        }
    }

    #[inline]
    fn upload(&self, location: &UniformLocation) {
        verify!(Context::get().uniform1i(Some(location), self.clone()));
    }
}

// // unsafe impl GLPrimitive for u32 {
// //     #[inline]
// //     fn gl_type(_: Option<u32>) -> u32 {
// //         gl::UNSIGNED_INT
// //     }

// //     #[inline]
// //     fn size(_: Option<u32>) -> u32 {
// //         1
// //     }

// //     #[inline]
// //     fn upload(&self, location: &UniformLocation) {
// //         verify!(Context::get().uniform1ui(Some(location), self.clone()));
// //     }
// // }

/*
 *
 * Impl for matrices
 *
 */
unsafe impl GLPrimitive for Matrix2<f32> {
    #[inline]
    fn gl_type() -> u32 {
        Context::FLOAT
    }

    #[inline]
    fn size() -> u32 {
        4
    }

    #[inline]
    fn flatten(array: &[Self]) -> PrimitiveArray {
        unsafe {
            let len = array.len() * 4;
            let ptr = array.as_ptr();

            PrimitiveArray::Float32(slice::from_raw_parts(ptr as *const f32, len))
        }
    }

    #[inline]
    fn upload(&self, location: &UniformLocation) {
        unsafe {
            verify!(Context::get().uniform_matrix2fv(Some(location), false, self));
        }
    }
}

unsafe impl GLPrimitive for Rotation2<f32> {
    #[inline]
    fn gl_type() -> u32 {
        Context::FLOAT
    }

    #[inline]
    fn size() -> u32 {
        4
    }

    #[inline]
    fn flatten(array: &[Self]) -> PrimitiveArray {
        unsafe {
            let len = array.len() * Self::size() as usize;
            let ptr = array.as_ptr();

            PrimitiveArray::Float32(slice::from_raw_parts(ptr as *const f32, len))
        }
    }

    #[inline]
    fn upload(&self, location: &UniformLocation) {
        unsafe {
            verify!(Context::get().uniform_matrix2fv(Some(location), false, self.matrix()));
        }
    }
}

unsafe impl GLPrimitive for Matrix3<f32> {
    #[inline]
    fn gl_type() -> u32 {
        Context::FLOAT
    }

    #[inline]
    fn size() -> u32 {
        9
    }

    #[inline]
    fn flatten(array: &[Self]) -> PrimitiveArray {
        unsafe {
            let len = array.len() * 9;
            let ptr = array.as_ptr();

            PrimitiveArray::Float32(slice::from_raw_parts(ptr as *const f32, len))
        }
    }

    #[inline]
    fn upload(&self, location: &UniformLocation) {
        unsafe {
            verify!(Context::get().uniform_matrix3fv(Some(location), false, self));
        }
    }
}

unsafe impl GLPrimitive for Rotation3<f32> {
    #[inline]
    fn gl_type() -> u32 {
        Context::FLOAT
    }

    #[inline]
    fn size() -> u32 {
        9
    }

    #[inline]
    fn flatten(array: &[Self]) -> PrimitiveArray {
        unsafe {
            let len = array.len() * Self::size() as usize;
            let ptr = array.as_ptr();

            PrimitiveArray::Float32(slice::from_raw_parts(ptr as *const f32, len))
        }
    }

    #[inline]
    fn upload(&self, location: &UniformLocation) {
        unsafe {
            verify!(Context::get().uniform_matrix3fv(Some(location), false, self.matrix()));
        }
    }
}

unsafe impl GLPrimitive for Matrix4<f32> {
    #[inline]
    fn gl_type() -> u32 {
        Context::FLOAT
    }

    #[inline]
    fn flatten(array: &[Self]) -> PrimitiveArray {
        unsafe {
            let len = array.len() * 16;
            let ptr = array.as_ptr();

            PrimitiveArray::Float32(slice::from_raw_parts(ptr as *const f32, len))
        }
    }

    #[inline]
    fn size() -> u32 {
        16
    }

    #[inline]
    fn upload(&self, location: &UniformLocation) {
        unsafe {
            verify!(Context::get().uniform_matrix4fv(Some(location), false, self));
        }
    }
}

/*
 *
 * Impl for vectors
 *
 */
unsafe impl GLPrimitive for Vector3<f32> {
    #[inline]
    fn gl_type() -> u32 {
        Context::FLOAT
    }

    #[inline]
    fn flatten(array: &[Self]) -> PrimitiveArray {
        unsafe {
            let len = array.len() * 3;
            let ptr = array.as_ptr();

            PrimitiveArray::Float32(slice::from_raw_parts(ptr as *const f32, len))
        }
    }

    #[inline]
    fn size() -> u32 {
        3
    }

    #[inline]
    fn upload(&self, location: &UniformLocation) {
        verify!(Context::get().uniform3f(Some(location), self.x, self.y, self.z));
    }
}

unsafe impl GLPrimitive for Vector2<f32> {
    #[inline]
    fn gl_type() -> u32 {
        Context::FLOAT
    }

    #[inline]
    fn flatten(array: &[Self]) -> PrimitiveArray {
        unsafe {
            let len = array.len() * 2;
            let ptr = array.as_ptr();

            PrimitiveArray::Float32(slice::from_raw_parts(ptr as *const f32, len))
        }
    }

    #[inline]
    fn size() -> u32 {
        2
    }

    #[inline]
    fn upload(&self, location: &UniformLocation) {
        verify!(Context::get().uniform2f(Some(location), self.x, self.y));
    }
}

// unsafe impl GLPrimitive for Vector2<u32> {
//     #[inline]
//     fn gl_type(_: Option<Vector2<u32>>) -> u32 {
//         gl::UNSIGNED_INT
//     }

//     #[inline]
//     fn size(_: Option<Vector2<u32>>) -> u32 {
//         2
//     }

//     #[inline]
//     fn upload(&self, location: &UniformLocation) {
//         verify!(Context::get().uniform2ui(Some(location), self.x, self.y));
//     }
// }

// unsafe impl GLPrimitive for Vector3<u32> {
//     #[inline]
//     fn gl_type(_: Option<Vector3<u32>>) -> u32 {
//         gl::UNSIGNED_INT
//     }

//     #[inline]
//     fn size(_: Option<Vector3<u32>>) -> u32 {
//         3
//     }

//     #[inline]
//     fn upload(&self, location: &UniformLocation) {
//         verify!(Context::get().uniform3ui(Some(location), self.x, self.y, self.z));
//     }
// }

/*
 *
 * Impl for points
 *
 */
unsafe impl GLPrimitive for Point3<f32> {
    #[inline]
    fn gl_type() -> u32 {
        Context::FLOAT
    }

    #[inline]
    fn flatten(array: &[Self]) -> PrimitiveArray {
        unsafe {
            let len = array.len() * Self::size() as usize;
            let ptr = array.as_ptr();

            PrimitiveArray::Float32(slice::from_raw_parts(ptr as *const f32, len))
        }
    }

    #[inline]
    fn size() -> u32 {
        3
    }

    #[inline]
    fn upload(&self, location: &UniformLocation) {
        verify!(Context::get().uniform3f(Some(location), self.x, self.y, self.z));
    }
}

unsafe impl GLPrimitive for Point2<f32> {
    #[inline]
    fn gl_type() -> u32 {
        Context::FLOAT
    }

    #[inline]
    fn flatten(array: &[Self]) -> PrimitiveArray {
        unsafe {
            let len = array.len() * Self::size() as usize;
            let ptr = array.as_ptr();

            PrimitiveArray::Float32(slice::from_raw_parts(ptr as *const f32, len))
        }
    }

    #[inline]
    fn size() -> u32 {
        2
    }

    #[inline]
    fn upload(&self, location: &UniformLocation) {
        verify!(Context::get().uniform2f(Some(location), self.x, self.y));
    }
}

// unsafe impl GLPrimitive for Point2<u32> {
//     #[inline]
//     fn gl_type(_: Option<Point2<u32>>) -> u32 {
//         gl::UNSIGNED_INT
//     }

//     #[inline]
//     fn size(_: Option<Point2<u32>>) -> u32 {
//         2
//     }

//     #[inline]
//     fn upload(&self, location: &UniformLocation) {
//         verify!(Context::get().uniform2ui(Some(location), self.x, self.y));
//     }
// }

// unsafe impl GLPrimitive for Point3<u32> {
//     #[inline]
//     fn gl_type(_: Option<Point3<u32>>) -> u32 {
//         gl::UNSIGNED_INT
//     }

//     #[inline]
//     fn size(_: Option<Point3<u32>>) -> u32 {
//         3
//     }

//     #[inline]
//     fn upload(&self, location: &UniformLocation) {
//         verify!(Context::get().uniform3ui(Some(location), self.x, self.y, self.z));
//     }
// }

/*
 *
 * Impl for tuples
 *
 */
unsafe impl GLPrimitive for (f32, f32, f32) {
    #[inline]
    fn gl_type() -> u32 {
        Context::FLOAT
    }

    #[inline]
    fn flatten(array: &[Self]) -> PrimitiveArray {
        unsafe {
            let len = array.len() * Self::size() as usize;
            let ptr = array.as_ptr();

            PrimitiveArray::Float32(slice::from_raw_parts(ptr as *const f32, len))
        }
    }

    #[inline]
    fn size() -> u32 {
        3
    }

    #[inline]
    fn upload(&self, location: &UniformLocation) {
        verify!(Context::get().uniform3f(Some(location), self.0, self.1, self.2));
    }
}

unsafe impl GLPrimitive for (f32, f32) {
    #[inline]
    fn gl_type() -> u32 {
        Context::FLOAT
    }

    #[inline]
    fn flatten(array: &[Self]) -> PrimitiveArray {
        unsafe {
            let len = array.len() * Self::size() as usize;
            let ptr = array.as_ptr();

            PrimitiveArray::Float32(slice::from_raw_parts(ptr as *const f32, len))
        }
    }

    #[inline]
    fn size() -> u32 {
        2
    }

    #[inline]
    fn upload(&self, location: &UniformLocation) {
        verify!(Context::get().uniform2f(Some(location), self.0, self.1));
    }
}

// unsafe impl GLPrimitive for (u32, u32) {
//     #[inline]
//     fn gl_type(_: Option<(u32, u32)>) -> u32 {
//         gl::UNSIGNED_INT
//     }

//     #[inline]
//     fn size(_: Option<(u32, u32)>) -> u32 {
//         2
//     }

//     #[inline]
//     fn upload(&self, location: &UniformLocation) {
//         verify!(Context::get().uniform2ui(Some(location), self.0, self.1));
//     }
// }

// unsafe impl GLPrimitive for (u32, u32, u32) {
//     #[inline]
//     fn gl_type(_: Option<(u32, u32, u32)>) -> u32 {
//         gl::UNSIGNED_INT
//     }

//     #[inline]
//     fn size(_: Option<(u32, u32, u32)>) -> u32 {
//         3
//     }

//     #[inline]
//     fn upload(&self, location: &UniformLocation) {
//         verify!(Context::get().uniform3ui(Some(location), self.0, self.1, self.2));
//     }
// }
