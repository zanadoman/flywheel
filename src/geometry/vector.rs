use core::ops::{
    Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign,
};

use super::Angle;

/// 2.5D `Vector`.
#[repr(C)]
#[derive(Debug, PartialEq, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Vector {
    /// X component of the `Vector`.
    pub x: f32,
    /// Y component of the `Vector`.
    pub y: f32,
    /// Z component of the `Vector`.
    pub z: f32,
}

impl Vector {
    /// Constructs a new `Vector` from the given X, Y, Z components.
    #[must_use]
    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    /// Constructs a new `Vector` from the given angle, 2D magnitude, Z
    /// component.
    #[must_use]
    pub fn from_angle(angle: f32, magnitude2: f32, z: f32) -> Self {
        Self::new(angle.cos() * magnitude2, angle.sin() * magnitude2, z)
    }

    /// Calculates the 2D magnitude of the `Vector`.
    #[must_use]
    pub fn magnitude2(&self) -> f32 {
        self.x.hypot(self.y)
    }

    /// Calculates the 3D magnitude of the `Vector`.
    #[must_use]
    pub fn magnitude3(&self) -> f32 {
        self.magnitude2().hypot(self.z)
    }

    /// Constructs a new 2D unit `Vector` from the `Vector`.
    #[must_use]
    pub fn normalize2(mut self) -> Self {
        let magnitude2 = self.magnitude2();
        if magnitude2 == 0.0 {
            self.x = 0.0;
            self.y = 0.0;
        } else {
            self.x /= magnitude2;
            self.y /= magnitude2;
        }
        self.z = 0.0;
        self
    }

    /// Constructs a new 3D unit `Vector` from the `Vector`.
    #[must_use]
    pub fn normalize3(mut self) -> Self {
        let magnitude3 = self.magnitude3();
        if magnitude3 == 0.0 {
            self.x = 0.0;
            self.y = 0.0;
            self.z = 0.0;
        } else {
            self /= magnitude3;
        }
        self
    }
}

impl Angle for Vector {
    fn angle(&self) -> f32 {
        self.y.atan2(self.x)
    }

    fn set_angle(&mut self, value: f32) {
        *self = Self::from_angle(value, self.magnitude2(), self.z);
    }
}

impl Neg for Vector {
    type Output = Self;

    fn neg(mut self) -> Self::Output {
        self.x = -self.x;
        self.y = -self.y;
        self.z = -self.z;
        self
    }
}

impl Add<Self> for Vector {
    type Output = Self;

    fn add(mut self, rhs: Self::Output) -> Self::Output {
        self += rhs;
        self
    }
}

impl AddAssign<Self> for Vector {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl Sub<Self> for Vector {
    type Output = Self;

    fn sub(mut self, rhs: Self::Output) -> Self::Output {
        self -= rhs;
        self
    }
}

impl SubAssign<Self> for Vector {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl Mul<f32> for Vector {
    type Output = Self;

    fn mul(mut self, rhs: f32) -> Self::Output {
        self *= rhs;
        self
    }
}

impl MulAssign<f32> for Vector {
    fn mul_assign(&mut self, rhs: f32) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl Div<f32> for Vector {
    type Output = Self;

    fn div(mut self, rhs: f32) -> Self::Output {
        self /= rhs;
        self
    }
}

impl DivAssign<f32> for Vector {
    fn div_assign(&mut self, rhs: f32) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geometry;

    #[test]
    fn new() {
        const X: f32 = 2.0;
        const Y: f32 = 3.0;
        const Z: f32 = 6.0;
        const VECTOR: Vector = Vector::new(X, Y, Z);
        approx::assert_relative_eq!(VECTOR.x, X);
        approx::assert_relative_eq!(VECTOR.y, Y);
        approx::assert_relative_eq!(VECTOR.z, Z);
    }

    #[test]
    fn from_angle() {
        const MAGNITUDE2: f32 = 3.605_551_2;
        const Z: f32 = 6.0;
        const MAGNITUDE3: f32 = 7.0;
        let angle = geometry::into_rads(45.0);
        let vector = Vector::from_angle(angle, MAGNITUDE2, Z);
        approx::assert_relative_eq!(vector.angle(), angle);
        approx::assert_relative_eq!(vector.magnitude2(), MAGNITUDE2);
        approx::assert_relative_eq!(vector.magnitude3(), MAGNITUDE3);
        let angle = geometry::into_rads(135.0);
        let vector = Vector::from_angle(angle, MAGNITUDE2, Z);
        approx::assert_relative_eq!(vector.angle(), angle);
        approx::assert_relative_eq!(vector.magnitude2(), MAGNITUDE2);
        approx::assert_relative_eq!(vector.magnitude3(), MAGNITUDE3);
        let angle = geometry::into_rads(-135.0);
        let vector = Vector::from_angle(angle, MAGNITUDE2, Z);
        approx::assert_relative_eq!(vector.angle(), angle);
        approx::assert_relative_eq!(vector.magnitude2(), MAGNITUDE2);
        approx::assert_relative_eq!(vector.magnitude3(), MAGNITUDE3);
        let angle = geometry::into_rads(-45.0);
        let vector = Vector::from_angle(angle, MAGNITUDE2, Z);
        approx::assert_relative_eq!(vector.angle(), angle);
        approx::assert_relative_eq!(vector.magnitude2(), MAGNITUDE2);
        approx::assert_relative_eq!(vector.magnitude3(), MAGNITUDE3);
    }

    #[test]
    fn magnitude2() {
        approx::assert_relative_eq!(
            Vector::new(2.0, 3.0, 6.0).magnitude2(),
            3.605_551_2
        );
    }

    #[test]
    fn magnitude3() {
        approx::assert_relative_eq!(
            Vector::new(2.0, 3.0, 6.0).magnitude3(),
            7.0
        );
    }

    #[test]
    fn normalize2() {
        let vector = Vector::new(0.0, 0.0, 6.0);
        approx::assert_relative_eq!(vector.normalize2().magnitude3(), 0.0);
        approx::assert_relative_eq!(
            vector.normalize2().angle(),
            vector.angle()
        );
        let vector = Vector::new(2.0, 3.0, 6.0);
        approx::assert_relative_eq!(vector.normalize2().magnitude3(), 1.0);
        approx::assert_relative_eq!(
            vector.normalize2().angle(),
            vector.angle()
        );
    }

    #[test]
    fn normalize3() {
        let vector = Vector::new(0.0, 0.0, 0.0);
        approx::assert_relative_eq!(vector.normalize3().magnitude3(), 0.0);
        approx::assert_relative_eq!(
            vector.normalize3().angle(),
            vector.angle()
        );
        let vector = Vector::new(2.0, 3.0, 6.0);
        approx::assert_relative_eq!(vector.normalize3().magnitude3(), 1.0);
        approx::assert_relative_eq!(
            vector.normalize3().angle(),
            vector.angle()
        );
    }

    #[test]
    fn angle() {
        approx::assert_relative_eq!(Vector::new(0.0, 0.0, 0.0).angle(), 0.0);
        approx::assert_relative_eq!(
            Vector::new(1.0, 1.0, 0.0).angle(),
            geometry::into_rads(45.0)
        );
        approx::assert_relative_eq!(
            Vector::new(-1.0, 1.0, 0.0).angle(),
            geometry::into_rads(135.0)
        );
        approx::assert_relative_eq!(
            Vector::new(-1.0, -1.0, 0.0).angle(),
            geometry::into_rads(-135.0)
        );
        approx::assert_relative_eq!(
            Vector::new(1.0, -1.0, 0.0).angle(),
            geometry::into_rads(-45.0)
        );
    }

    #[test]
    fn set_angle() {
        const MAGNITUDE2: f32 = 3.605_551_2;
        const MAGNITUDE3: f32 = 7.0;
        let mut vector = Vector::new(2.0, 3.0, 6.0);
        let angle = geometry::into_rads(45.0);
        vector.set_angle(angle);
        approx::assert_relative_eq!(vector.angle(), angle);
        approx::assert_relative_eq!(vector.magnitude2(), MAGNITUDE2);
        approx::assert_relative_eq!(vector.magnitude3(), MAGNITUDE3);
        let angle = geometry::into_rads(135.0);
        vector.set_angle(angle);
        approx::assert_relative_eq!(vector.angle(), angle);
        approx::assert_relative_eq!(vector.magnitude2(), MAGNITUDE2);
        approx::assert_relative_eq!(vector.magnitude3(), MAGNITUDE3);
        let angle = geometry::into_rads(-135.0);
        vector.set_angle(angle);
        approx::assert_relative_eq!(vector.angle(), angle);
        approx::assert_relative_eq!(vector.magnitude2(), MAGNITUDE2);
        approx::assert_relative_eq!(vector.magnitude3(), MAGNITUDE3);
        let angle = geometry::into_rads(-45.0);
        vector.set_angle(angle);
        approx::assert_relative_eq!(vector.angle(), angle);
        approx::assert_relative_eq!(vector.magnitude2(), MAGNITUDE2);
        approx::assert_relative_eq!(vector.magnitude3(), MAGNITUDE3);
    }

    #[test]
    fn neg() {
        const X: f32 = 2.0;
        const Y: f32 = 3.0;
        const Z: f32 = 6.0;
        assert_eq!(-Vector::new(X, Y, Z), Vector::new(-X, -Y, -Z));
    }

    #[test]
    fn add() {
        assert_eq!(
            Vector::new(1.0, 2.0, 3.0) + Vector::new(4.0, 5.0, 6.0),
            Vector::new(5.0, 7.0, 9.0)
        );
    }

    #[test]
    fn add_assign() {
        let mut vector = Vector::new(1.0, 2.0, 3.0);
        vector += Vector::new(4.0, 5.0, 6.0);
        assert_eq!(vector, Vector::new(5.0, 7.0, 9.0));
    }

    #[test]
    fn sub() {
        assert_eq!(
            Vector::new(1.0, 2.0, 3.0) - Vector::new(4.0, 5.0, 6.0),
            Vector::new(-3.0, -3.0, -3.0)
        );
    }

    #[test]
    fn sub_assign() {
        let mut vector = Vector::new(1.0, 2.0, 3.0);
        vector -= Vector::new(4.0, 5.0, 6.0);
        assert_eq!(vector, Vector::new(-3.0, -3.0, -3.0));
    }

    #[test]
    fn mul() {
        assert_eq!(
            Vector::new(2.0, 3.0, 6.0) * 7.0,
            Vector::new(14.0, 21.0, 42.0)
        );
    }

    #[test]
    fn mul_assign() {
        let mut vector = Vector::new(2.0, 3.0, 6.0);
        vector *= 7.0;
        assert_eq!(vector, Vector::new(14.0, 21.0, 42.0));
    }

    #[test]
    fn div() {
        assert_eq!(
            Vector::new(2.0, 3.0, 6.0) / 7.0,
            Vector::new(0.285_714_3, 0.428_571_43, 0.857_142_87)
        );
    }

    #[test]
    fn div_assign() {
        let mut vector = Vector::new(2.0, 3.0, 6.0);
        vector /= 7.0;
        assert_eq!(
            vector,
            Vector::new(0.285_714_3, 0.428_571_43, 0.857_142_87)
        );
    }
}
