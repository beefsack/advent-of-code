use std::ops::{Add, AddAssign, Sub, SubAssign};

#[derive(Debug, PartialEq, Eq, Copy, Clone, Default)]
pub struct IPoint3 {
    pub x: isize,
    pub y: isize,
    pub z: isize,
}

impl Add for IPoint3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl AddAssign for IPoint3 {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl Sub for IPoint3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl SubAssign for IPoint3 {
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
    }
}

impl IPoint3 {
    pub fn length(&self) -> f64 {
        ((self.x.pow(2) + self.y.pow(2) + self.z.pow(2)) as f64).sqrt()
    }

    pub fn to_vec(&self) -> Vec<isize> {
        vec![self.x, self.y, self.z]
    }
}
