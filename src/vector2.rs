#[derive(Debug, Clone, Copy)]
pub struct Vector2 {
    pub x: i32,
    pub y: i32,
}

impl Vector2 {
    /* Constants */
    pub fn zero() -> Vector2 {
        return Vector2::new(0, 0)
    }

    pub fn one() -> Vector2 {
        return Vector2::new(1, 1)
    }

    /* Public methods */
    pub fn new(in_x: i32, in_y: i32) -> Vector2 {
        return Vector2{ x: in_x, y: in_y }
    }

    pub fn to_string(&self) -> String {
        return std::format!("Vector2({}, {})", self.x, self.y);
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
