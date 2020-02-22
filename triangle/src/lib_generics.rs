use std::cmp::{PartialEq, PartialOrd};
use std::convert::From;
use std::ops::Add;

pub struct Triangle<T> {
    a: T,
    b: T,
    c: T,
}

impl<T> Triangle<T>
where
    T: PartialEq + PartialOrd + Add<Output = T> + From,
{
    pub fn build(sides: [T; 3]) -> Option<Triangle<T>> {
        let (a, b, c) = (sides[0], sides[1], sides[2]);
        let zero = T::from(0);

        if a == zero || b == zero || c == zero {
            return None;
        }

        if !(a + b >= c && a + c >= b && b + c >= a) {
            return None;
        }

        Some(Triangle { a, b, c })
    }

    pub fn is_equilateral(&self) -> bool {
        self.a == self.c && self.a == self.b
    }

    pub fn is_scalene(&self) -> bool {
        self.a != self.c && self.a != self.b && self.b != self.c
    }

    pub fn is_isosceles(&self) -> bool {
        self.a == self.c || self.a == self.b || self.c == self.b
    }
}
