use std::ops::Add;
use vector2::Vector2;

mod math;
mod vector2;

fn main() {
    
    // math functions
    // println!("|-10| = {}", math::abs(-10.0));
    // println!("sign of -23 = {}, sign of 42 = {}, sign of 0 = {}", math::sign(-23.0), math::sign(42.0), math::sign(0.0));
    
    // vector2 debug
    let vec1 = Vector2::new(50.0, 50.0);
    let vec2 = Vector2::new(10.0, 20.0);

    // println!("vec1 = {}", vec1.to_string());
    // println!("vec2 = {}", vec2.to_string());
    // println!("vec1 + vec2 = {}", (vec1 + vec2).to_string());
    // println!("vec1.add(vec2) = {}", vec1.add(vec2).to_string());
    // println!("vec1 * vec2 = {}", (vec1 * vec2).to_string());
    println!("vec1 / vec2 = {}", (vec1 / vec2).to_string());
    println!("vec1 / 2.0 = {}", (vec1 / 2.0).to_string());
    // println!("debug vec1: {:?}", vec1);
    // println!("pretty printed vec1: {:#?}", vec1);
    // println!("vec zero: {:?}", Vector2::zero());
    // println!("vec one: {:?}", Vector2::one());
    println!("vector length = {}", Vector2::new(2.0, 5.0).length());
    println!("normalized vector = {}", Vector2::one().normalized().to_string());
}