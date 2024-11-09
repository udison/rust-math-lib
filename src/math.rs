pub fn abs(n: f32) -> f32 {
    if (n < 0.0) {
        return n * -1.0;
    }

    return n;
}