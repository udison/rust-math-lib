
/// Returns the absolute value of a f32 number
/// 
/// ### Examples
/// ```
/// math::abs(-1.2) -> 1.2
/// math::abs(1.0) -> 1.0
/// math::abs(-32.345) -> 32.345
/// ```
pub fn abs(n: f32) -> f32 {
    if n < 0.0 {
        return n * -1.0;
    }

    return n;
}

/// Returns the absolute value of an i32 number
/// 
/// ### Examples
/// ```
/// math::absi(-1) -> 1
/// math::absi(1) -> 1
/// math::absi(-32) -> 32
/// ```
pub fn absi(n: i32) -> i32 {
    if n < 0 {
        return n * -1;
    }

    return n;
}

/// Returns the sign of a f32 number
/// 
/// ### Examples
/// ```
/// math::sign(-3.0) -> -1.0
/// math::sign(0.0) -> 0.0
/// math::sign(7.0) -> 1.0
/// ```
pub fn sign(n: f32) -> f32 {
    return 
        if n < 0.0 { -1.0 }
        else if n > 0.0 { 1.0 }
        else { 0.0 }
}

/// Returns the sign of an i32 number
/// 
/// ### Examples
/// ```
/// math::sign(-3) -> -1
/// math::sign(0) -> 0
/// math::sign(7) -> 1
/// ```
pub fn signi(n: i32) -> i32 {
    return 
        if n < 0 { -1 }
        else if n > 0 { 1 }
        else { 0 }
}