use std::{iter::Sum, ops::{Add, AddAssign}};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Default)]
pub struct Point(pub i32, pub i32);

impl Sum for Point {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        let mut temp: Point = (0_i32, 0_i32).into();

        for item in iter{
            temp += item;
        }

        return temp;
    }
}

impl From<(i32, i32)> for Point {
    fn from(from: (i32, i32)) -> Self {
        Self(from.0, from.1)
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Point(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self (
            self.0 + rhs.0,
            self.1 + rhs.1,
        );
    }
}