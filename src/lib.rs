#![feature(tuple_indexing)]

#[deriving(Show, PartialEq)]
pub struct Vec2<T>(T, T);

impl<T: 'static + Copy + Add<T, T>> Add<Vec2<T>, Vec2<T>> for Vec2<T> {
    #[inline(always)]
    fn add(&self, rhs: &Vec2<T>) -> Vec2<T> {
        use std::intrinsics::TypeId;
        use std::simd::f64x2;
        let ty = TypeId::of::<T>();
        match ty {
            x if x == TypeId::of::<f64>() => {
                unsafe {
                    let a = f64x2(
                        *(&self.0 as *const T as *const f64),
                        *(&self.1 as *const T as *const f64)
                    );
                    let b = f64x2(
                        *(&rhs.0 as *const T as *const f64),
                        *(&rhs.1 as *const T as *const f64)
                    );
                    let res = a + b;
                    Vec2(
                        *(&res.0 as *const f64 as *const T),
                        *(&res.1 as *const f64 as *const T)
                    )
                }
            }
            _ => {
                Vec2(self.0 + rhs.0, self.1 + rhs.1)
            }
        }
    }
}

#[test]
fn test_add() {
    let x = Vec2(1.0f64, 0.0);
    let y = Vec2(0.0f64, 1.0);
    let z = x + y;
    assert_eq!(z, Vec2(1.0, 1.0));
}


