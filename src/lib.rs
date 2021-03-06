#![feature(core)]

use std::ops::Add;
use std::any::{ Any, TypeId };

#[derive(Debug, PartialEq)]
pub struct Vec2<T>(pub T, pub T);

#[derive(Debug, PartialEq)]
pub struct Vec4<T>(pub T, pub T, pub T, pub T);

pub const TEST_ITERATIONS: usize = 1_000_000_000;

#[inline(always)]
fn add2_f64<T: Copy>(a: &Vec2<T>, b: &Vec2<T>) -> Vec2<T> {
    use std::simd::f64x2;
    unsafe {
        let a = f64x2(
            *(&a.0 as *const T as *const f64),
            *(&a.1 as *const T as *const f64)
        );
        let b = f64x2(
            *(&b.0 as *const T as *const f64),
            *(&b.1 as *const T as *const f64)
        );
        let res = a + b;
        Vec2(
            *(&res.0 as *const f64 as *const T),
            *(&res.1 as *const f64 as *const T)
        )
    }
}

#[inline(always)]
fn add2_t<T>(a: &Vec2<T>, b: &Vec2<T>) -> Vec2<T>
    where T: Copy + Add<T, Output = T>
{
    Vec2(a.0 + b.0, a.1 + b.1)
}

#[inline(always)]
unsafe fn add2_func<T>() -> fn (&Vec2<T>, &Vec2<T>) -> Vec2<T>
    where T: 'static + Copy + Add<T, Output = T>
{
    use std::intrinsics::type_id;
    let ty = type_id::<T>();
    let ty_f64 = type_id::<f64>();
    if ty == ty_f64 {
        add2_f64
    } else {
        add2_t
    }
}

impl<T> Add<Vec2<T>> for Vec2<T>
    where T: 'static + Copy + Add<T, Output = T>
{
    type Output = Vec2<T>;

    #[inline(always)]
    fn add(self, rhs: Vec2<T>) -> Vec2<T> {
        unsafe { add2_func::<T>()(&self, &rhs) }
    }
}

#[test]
fn test_add() {
    let x = Vec2(1.0f64, 0.0);
    let y = Vec2(0.0f64, 1.0);
    let z = x + y;
    assert_eq!(z, Vec2(1.0, 1.0));
}

#[inline(always)]
fn add4_f32<T: Copy>(a: &Vec4<T>, b: &Vec4<T>) -> Vec4<T> {
    use std::simd::f32x4;
    unsafe {
        let a = f32x4(
            *(&a.0 as *const T as *const f32),
            *(&a.1 as *const T as *const f32),
            *(&a.2 as *const T as *const f32),
            *(&a.3 as *const T as *const f32)
        );
        let b = f32x4(
            *(&b.0 as *const T as *const f32),
            *(&b.1 as *const T as *const f32),
            *(&b.2 as *const T as *const f32),
            *(&b.3 as *const T as *const f32)
        );
        let res = a + b;
        Vec4(
            *(&res.0 as *const f32 as *const T),
            *(&res.1 as *const f32 as *const T),
            *(&res.2 as *const f32 as *const T),
            *(&res.3 as *const f32 as *const T)
        )
    }
}

#[inline(always)]
fn add4_t<T>(a: &Vec4<T>, b: &Vec4<T>) -> Vec4<T>
    where T: Copy + Add<T, Output = T>
{
    Vec4(a.0 + b.0, a.1 + b.1, a.2 + b.2, a.3 + b.3)
}

unsafe fn add4_func<T>() -> fn (&Vec4<T>, &Vec4<T>) -> Vec4<T>
    where T: Any + Copy + Add<T, Output = T>
{
    let ty = TypeId::of::<T>();
    let ty_f32 = TypeId::of::<f32>();
    if ty == ty_f32 {
        add4_f32
    } else {
        add4_t
    }
}

impl<T> Add<Vec4<T>> for Vec4<T>
    where T: Any + Copy + Add<T, Output = T>
{
    type Output = Vec4<T>;

    #[inline(always)]
    fn add(self, rhs: Vec4<T>) -> Vec4<T> {
        unsafe { add4_func::<T>()(&self, &rhs) }
    }
}

