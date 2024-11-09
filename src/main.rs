use std::ops::Add;
use vector2::Vector2;

mod math;
mod vector2;

fn main() {
    let vec1 = Vector2::new(50, 50);
    let vec2 = Vector2::new(10, 20);

    // math functions
    println!("|-10| = {}", math::abs(-10.0));

    // vector2 debug
    // println!("vec1 = {}", vec1.to_string());
    // println!("vec2 = {}", vec2.to_string());
    // println!("vec1 + vec2 = {}", (vec1 + vec2).to_string());
    // println!("vec1.add(vec2) = {}", vec1.add(vec2).to_string());
    // println!("vec1 * vec2 = {}", (vec1 * vec2).to_string());
    // println!("debug vec1: {:?}", vec1);
    // println!("pretty printed vec1: {:#?}", vec1);
    // println!("vec zero: {:?}", Vector2::zero());
    // println!("vec one: {:?}", Vector2::one());
}