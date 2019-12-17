use std::ops::{Add, AddAssign, Index, Sub, SubAssign};

#[derive(Debug, PartialEq, Eq, Copy, Clone, Default, Hash)]
pub struct IPoint2 {
    pub x: isize,
    pub y: isize,
}

impl Add for IPoint2 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl AddAssign for IPoint2 {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl Sub for IPoint2 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl SubAssign for IPoint2 {
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
pub enum Field {
    X,
    Y,
}

impl Index<Field> for IPoint2 {
    type Output = isize;

    fn index(&self, field: Field) -> &Self::Output {
        match field {
            Field::X => &self.x,
            Field::Y => &self.y,
        }
    }
}

impl IPoint2 {
    pub fn length(&self) -> f64 {
        ((self.x.pow(2) + self.y.pow(2)) as f64).sqrt()
    }

    pub fn to_vec(&self) -> Vec<isize> {
        vec![self.x, self.y]
    }
}
