use std::ops::{Add, AddAssign, Index, Sub, SubAssign};

#[derive(Debug, PartialEq, Eq, Copy, Clone, Default, Hash)]
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

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
pub enum Field {
    X,
    Y,
    Z,
}

impl Index<Field> for IPoint3 {
    type Output = isize;

    fn index(&self, field: Field) -> &Self::Output {
        match field {
            Field::X => &self.x,
            Field::Y => &self.y,
            Field::Z => &self.z,
        }
    }
}

impl IPoint3 {
    pub fn length(&self) -> f64 {
        ((self.x.pow(2) + self.y.pow(2) + self.z.pow(2)) as f64).sqrt()
    }

    pub fn manhattan(&self) -> isize {
        self.x.abs() + self.y.abs() + self.z.abs()
    }

    pub fn to_vec(&self) -> Vec<isize> {
        vec![self.x, self.y, self.z]
    }
}
