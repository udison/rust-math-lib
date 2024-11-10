/// Represents a two-dimensional vector, along with utility function to work with those vectors.
#[derive(Debug, Clone, Copy)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

impl Vector2 {
    /// Returns a new instance of ```Vector2```
    pub fn new(in_x: f32, in_y: f32) -> Vector2 {
        return Vector2{ x: in_x, y: in_y }
    }

    /// Returns a zeroed vector. Shorthand for ```Vector2::new(0, 0)```
    pub fn zero() -> Vector2 {
        return Vector2::new(0.0, 0.0)
    }

    /// Returns a vector with 1 one both axes. Shorthand for ```Vector2::new(1, 1)```
    pub fn one() -> Vector2 {
        return Vector2::new(1.0, 1.0)
    }

    /// Returns the length of this vector
    pub fn length(&self) -> f32 {
        return f32::sqrt(self.x * self.x + self.y * self.y);
    }

    /// Returns the normalized ```Vector2``` of this vector
    pub fn normalized(&self) -> Vector2 {
        return *self / self.length();
    }

    /// Normalize this vector. This function changes the values of this ```Vector2``` instance
    pub fn normalize(&mut self) {
        let self_len = self.length();

        self.x = self.x / self_len;
        self.y = self.y / self_len;
    }

    /// Converts a ```Vector2``` to a easy to read string
    pub fn to_string(&self) -> String {
        return std::format!("Vector2(x = {}, y = {})", self.x, self.y);
    }

}

/*
 * Operators
 */
impl std::ops::Add for Vector2 {
    type Output = Vector2;

    fn add(self, rhs: Vector2) -> Vector2 {
        return Vector2::new(self.x + rhs.x, self.y + rhs.y);
    }
}

impl std::ops::Mul for Vector2 {
    type Output = Vector2;

    fn mul(self, rhs: Vector2) -> Vector2 {
        return Vector2::new(self.x * rhs.x, self.y * rhs.y);
    }
}

impl std::ops::Div for Vector2 {
    type Output = Vector2;

    fn div(self, rhs: Vector2) -> Vector2 {
        return Vector2::new(self.x / rhs.x, self.y / rhs.y);
    }
}

impl std::ops::Div<f32> for Vector2 {
    type Output = Vector2;

    fn div(self, rhs: f32) -> Vector2 {
        return Vector2::new(self.x / rhs, self.y / rhs);
    }
}
