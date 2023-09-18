// and enums

use std::fmt::Display;

pub trait Euclidean {
    fn cross();
    fn magnitude() {
        println!("Hello World");
    }
}

#[derive(Debug)]
pub struct Vector3 {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl Vector3 {
    fn dot(&self, other: &Vector3) -> i32 {
        self.x * other.x
        +
        self.y * other.y
        +
        self.z * other.z
    }
}

impl Euclidean for Vector3 {
    fn cross() {}
}

// FIELD INIT SHORTHAND
pub fn build_vector3(x: i32, y: i32, z: i32) -> Vector3 {
    Vector3 {
        x, y, z
    }
}

// STRUCT UPDATE SYNTAX
pub fn update_x(vec: Vector3) -> Vector3 {
    Vector3 {
        x: 5,
        ..vec
    }
}

// use trait as a parameter type
pub fn compound_mag(x: &impl Euclidean, 
    y: &(impl Euclidean + Display)) // multiple required traits
{

}

// above is sugar for below, restrictions on generic types
pub fn compound_mag_long<T: Euclidean + Display>(x: &T) 
    -> impl Display // specify just traits of a return type
{
    5 // for display trait
}

// use where for easier reading
pub fn compound_mag_where<T>(x: &T) 
    where T: Euclidean + Display {
}

enum ColourChannel {
    Red,
    Green,
    Blue
}

#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

// can add methods to enums as well
impl IpAddr {
    fn print(&self) {
        println!("{:?}", self);
    }
}