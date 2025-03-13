use super::Vector;

/// Represents an object that has a position.
pub trait Position {
    /// Returns a reference to the position `Vector` of the object.
    fn position(&self) -> &Vector;

    /// Returns a mutable reference to the position `Vector` of the object.
    fn position_mut(&mut self) -> &mut Vector;
}

/// Represents an object that has an angle.
pub trait Angle {
    /// Returns the angle of the object.
    fn angle(&self) -> f32;

    /// Sets the angle of the object.
    fn set_angle(&mut self, value: f32);
}

/// Represents an object that can be scaled.
pub trait Scale {
    /// Returns the scale of the object.
    fn scale(&self) -> f32;

    /// Sets the scale of the object.
    fn set_scale(&mut self, value: f32);
}

/// Represents an object that has a rectangular shape.
pub trait Rectangle {
    /// Returns the width of the object.
    fn width(&self) -> f32;

    /// Sets the width of the object.
    fn set_width(&mut self, value: f32);

    /// Returns the height of the object.
    fn height(&self) -> f32;

    /// Sets the height of the object.
    fn set_height(&mut self, value: f32);
}
