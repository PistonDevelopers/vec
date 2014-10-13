#![feature(tuple_indexing)]

#[deriving(Show, PartialEq)]
pub struct Vec2<T>(pub T, pub T);

pub const TEST_ITERATIONS: uint = 1_000_000_000;

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
fn add2_t<T: Add<T, T>>(a: &Vec2<T>, b: &Vec2<T>) -> Vec2<T> {
    Vec2(a.0 + b.0, a.1 + b.1)
}

#[inline(always)]
unsafe fn add2_func<T: 'static + Copy + Add<T, T>>() 
-> fn (&Vec2<T>, &Vec2<T>) -> Vec2<T> {
    use std::intrinsics::type_id;
    let ty = type_id::<T>();
    let ty_f64 = type_id::<f64>();
    if ty == ty_f64 {
        add2_f64
    } else {
        add2_t
    }
}

impl<T: 'static + Copy + Add<T, T>> Add<Vec2<T>, Vec2<T>> for Vec2<T> {
    #[inline(always)]
    fn add(&self, rhs: &Vec2<T>) -> Vec2<T> {
        unsafe { add2_func::<T>()(self, rhs) }
    }
}

#[test]
fn test_add() {
    let x = Vec2(1.0f64, 0.0);
    let y = Vec2(0.0f64, 1.0);
    let z = x + y;
    assert_eq!(z, Vec2(1.0, 1.0));
}


