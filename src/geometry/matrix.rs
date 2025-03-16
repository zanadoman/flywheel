use core::ops::Mul;

use super::{Angle, Scale, Vector};

/// 2.5D transformation `Matrix`.
#[repr(C)]
#[derive(Debug, PartialEq, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Matrix {
    angle: f32,
    scale: f32,
    columns: ((f32, f32), (f32, f32)),
}

impl Matrix {
    /// Constructs a new `Matrix` from the given angle, scale.
    #[must_use]
    pub fn new(angle: f32, scale: f32) -> Self {
        let sin_scale = angle.sin() * scale;
        let cos_scale = angle.cos() * scale;
        Self {
            angle,
            scale,
            columns: ((cos_scale, sin_scale), (-sin_scale, cos_scale)),
        }
    }

    /// Constructs a new identity `Matrix`.
    #[must_use]
    pub const fn identity() -> Self {
        Self {
            angle: 0.0,
            scale: 1.0,
            columns: ((1.0, 0.0), (0.0, 1.0)),
        }
    }

    /// Returns the transformed X component of a `Vector`.
    #[must_use]
    pub const fn transform_x(&self, vector: &Vector) -> f32 {
        vector.x * self.columns.0.0 + vector.y * self.columns.1.0
    }

    /// Returns the transformed Y component of a `Vector`.
    #[must_use]
    pub const fn transform_y(&self, vector: &Vector) -> f32 {
        vector.x * self.columns.0.1 + vector.y * self.columns.1.1
    }

    /// Returns the transformed Z component of a `Vector`.
    #[must_use]
    pub const fn transform_z(&self, vector: &Vector) -> f32 {
        vector.z * self.scale
    }
}

impl Angle for Matrix {
    fn angle(&self) -> f32 {
        self.angle
    }

    fn set_angle(&mut self, value: f32) {
        *self = Self::new(value, self.scale);
    }
}

impl Scale for Matrix {
    fn scale(&self) -> f32 {
        self.scale
    }

    fn set_scale(&mut self, value: f32) {
        *self = Self::new(self.angle, value);
    }
}

impl Mul<Vector> for Matrix {
    type Output = Vector;

    fn mul(self, rhs: Self::Output) -> Self::Output {
        Self::Output::new(
            self.transform_x(&rhs),
            self.transform_y(&rhs),
            self.transform_z(&rhs),
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::geometry;

    use super::*;

    #[test]
    fn new() {
        const ANGLE: f32 = geometry::into_rads(45.0);
        const SCALE: f32 = 7.0;
        let matrix = Matrix::new(ANGLE, SCALE);
        assert_eq!(matrix.angle(), ANGLE);
        assert_eq!(matrix.scale(), SCALE);
    }

    #[test]
    fn identity() {
        const MATRIX: Matrix = Matrix::identity();
        const VECTOR: Vector = Vector::new(2.0, 3.0, 6.0);
        assert_eq!(MATRIX.angle(), 0.0);
        assert_eq!(MATRIX.scale(), 1.0);
        assert_eq!(MATRIX * VECTOR, VECTOR);
    }

    #[test]
    fn transform_x() {
        const SCALE: f32 = 7.0;
        const MAGNITUDE2: f32 = 3.6055512;
        const Z: f32 = 6.0;
        const VECTOR: Vector = Vector::new(MAGNITUDE2, 0.0, Z);
        let angle = geometry::into_rads(45.0);
        assert_eq!(
            Matrix::new(angle, SCALE).transform_x(&VECTOR),
            (Vector::from_angle(angle, MAGNITUDE2, Z) * SCALE).x
        );
        let angle = geometry::into_rads(135.0);
        assert_eq!(
            Matrix::new(angle, SCALE).transform_x(&VECTOR),
            (Vector::from_angle(angle, MAGNITUDE2, Z) * SCALE).x
        );
        let angle = geometry::into_rads(-135.0);
        assert_eq!(
            Matrix::new(angle, SCALE).transform_x(&VECTOR),
            (Vector::from_angle(angle, MAGNITUDE2, Z) * SCALE).x
        );
        let angle = geometry::into_rads(-45.0);
        assert_eq!(
            Matrix::new(angle, SCALE).transform_x(&VECTOR),
            (Vector::from_angle(angle, MAGNITUDE2, Z) * SCALE).x
        );
    }

    #[test]
    fn transform_y() {
        const SCALE: f32 = 7.0;
        const MAGNITUDE2: f32 = 3.6055512;
        const Z: f32 = 6.0;
        const VECTOR: Vector = Vector::new(MAGNITUDE2, 0.0, Z);
        let angle = geometry::into_rads(45.0);
        assert_eq!(
            Matrix::new(angle, SCALE).transform_y(&VECTOR),
            (Vector::from_angle(angle, MAGNITUDE2, Z) * SCALE).y
        );
        let angle = geometry::into_rads(135.0);
        assert_eq!(
            Matrix::new(angle, SCALE).transform_y(&VECTOR),
            (Vector::from_angle(angle, MAGNITUDE2, Z) * SCALE).y
        );
        let angle = geometry::into_rads(-135.0);
        assert_eq!(
            Matrix::new(angle, SCALE).transform_y(&VECTOR),
            (Vector::from_angle(angle, MAGNITUDE2, Z) * SCALE).y
        );
        let angle = geometry::into_rads(-45.0);
        assert_eq!(
            Matrix::new(angle, SCALE).transform_y(&VECTOR),
            (Vector::from_angle(angle, MAGNITUDE2, Z) * SCALE).y
        );
    }

    #[test]
    fn transform_z() {
        const SCALE: f32 = 7.0;
        const MAGNITUDE2: f32 = 3.6055512;
        const Z: f32 = 6.0;
        const VECTOR: Vector = Vector::new(MAGNITUDE2, 0.0, Z);
        let angle = geometry::into_rads(45.0);
        assert_eq!(
            Matrix::new(angle, SCALE).transform_z(&VECTOR),
            (Vector::from_angle(angle, MAGNITUDE2, Z) * SCALE).z
        );
        let angle = geometry::into_rads(135.0);
        assert_eq!(
            Matrix::new(angle, SCALE).transform_z(&VECTOR),
            (Vector::from_angle(angle, MAGNITUDE2, Z) * SCALE).z
        );
        let angle = geometry::into_rads(-135.0);
        assert_eq!(
            Matrix::new(angle, SCALE).transform_z(&VECTOR),
            (Vector::from_angle(angle, MAGNITUDE2, Z) * SCALE).z
        );
        let angle = geometry::into_rads(-45.0);
        assert_eq!(
            Matrix::new(angle, SCALE).transform_z(&VECTOR),
            (Vector::from_angle(angle, MAGNITUDE2, Z) * SCALE).z
        );
    }

    #[test]
    fn set_angle() {
        const ANGLE: f32 = 45.0;
        let mut matrix = Matrix::identity();
        matrix.set_angle(ANGLE);
        assert_eq!(matrix.angle(), ANGLE);
    }

    #[test]
    fn set_scale() {
        const SCALE: f32 = 7.0;
        let mut matrix = Matrix::identity();
        matrix.set_scale(SCALE);
        assert_eq!(matrix.scale(), SCALE);
    }

    #[test]
    fn mul() {
        const SCALE: f32 = 7.0;
        const MAGNITUDE2: f32 = 3.6055512;
        const Z: f32 = 6.0;
        const VECTOR: Vector = Vector::new(MAGNITUDE2, 0.0, Z);
        let angle = geometry::into_rads(45.0);
        assert_eq!(
            Matrix::new(angle, SCALE) * VECTOR,
            Vector::from_angle(angle, MAGNITUDE2, Z) * SCALE
        );
        let angle = geometry::into_rads(135.0);
        assert_eq!(
            Matrix::new(angle, SCALE) * VECTOR,
            Vector::from_angle(angle, MAGNITUDE2, Z) * SCALE
        );
        let angle = geometry::into_rads(-135.0);
        assert_eq!(
            Matrix::new(angle, SCALE) * VECTOR,
            Vector::from_angle(angle, MAGNITUDE2, Z) * SCALE
        );
        let angle = geometry::into_rads(-45.0);
        assert_eq!(
            Matrix::new(angle, SCALE) * VECTOR,
            Vector::from_angle(angle, MAGNITUDE2, Z) * SCALE
        );
    }
}
