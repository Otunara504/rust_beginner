#![allow(dead_code)] // Suppresses warnings when a declared function is not used.

use core::cmp::Ordering;

// Enumeration for comparison
pub enum Comp {
    LessThan,
    GreaterThan,
    Equal,
}

// Enumeration for Gender
pub enum Gender {
    Male,
    Female,
}

// Struct representing a person
#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

struct Unit; // A unit struct

// Tuple struct
struct Pair(i32, f32);

// Struct representing a point in space
struct Point {
    x: f32,
    y: f32,
}

// Struct representing a rectangle
struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

// Trait defining common methods for shapes
trait Shape {
    fn new(length: i32, width: i32, name: &'static str) -> Self;
    fn area(&self) -> i32;
    fn perimeter(&self) -> i32;
    fn set_length(&mut self, length: i32);
    fn get_length(&self) -> i32;
    fn set_width(&mut self, width: i32);
    fn get_width(&self) -> i32;
    fn set_name(&mut self, name: &'static str);
    fn get_name(&self) -> &str;
}

// Struct representing a rectangle implementing the Shape trait
#[derive(Default, Debug, Clone)]
struct Rect {
    length: i32,
    width: i32,
    name: &'static str,
}

// Implementation of methods for Rect
impl Rect {
    // Default constructor for Rect
    fn default() -> Self {
        Rect {
            length: 1,
            width: 1,
            name: "default_name",
        }
    }
}

// Implementation of Shape trait for Rect
impl Shape for Rect {
    fn new(length: i32, width: i32, name: &'static str) -> Self {
        Rect {
            length,
            width,
            name,
        }
    }

    fn area(&self) -> i32 {
        self.length * self.width
    }

    fn perimeter(&self) -> i32 {
        2 * (self.length + self.width)
    }

    fn set_length(&mut self, length: i32) {
        self.length = length;
    }

    fn get_length(&self) -> i32 {
        self.length
    }

    fn set_width(&mut self, width: i32) {
        self.width = width;
    }

    fn get_width(&self) -> i32 {
        self.width
    }

    fn set_name(&mut self, name: &'static str) {
        self.name = name;
    }

    fn get_name(&self) -> &str {
        self.name
    }
}

// Implementation of PartialEq for Rect
impl PartialEq for Rect {
    fn eq(&self, other: &Self) -> bool {
        self.area() == other.area()
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

// Implementation of PartialOrd for Rect
impl PartialOrd for Rect {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.area().partial_cmp(&other.area())
    }
}

// Implementation of conversion into String for Rect
impl From<&'static str> for Rect {
    fn from(s: &'static str) -> Rect {
        let mut parts = s.split(',');
        let length = match parts.next() {
            Some(val) => val.parse::<i32>().unwrap(),
            None => 0,
        };
        let width = match parts.next() {
            Some(val) => val.parse::<i32>().unwrap(),
            None => 0,
        };
        let name = match parts.next() {
            Some(val) => val,
            None => "",
        };
        Rect { length, width, name: &name }
    }
}

// Struct representing a circle implementing the Shape trait
struct Circle {
    radius: i32,
    name: &'static str,
}

// Implementation of methods for Circle
impl Circle {
    fn new(radius: i32, name: &'static str) -> Self {
        Circle { radius, name }
    }
}

// Implementation of Shape trait for Circle
impl Shape for Circle {
    fn new(_length: i32, _width: i32, name: &'static str) -> Self {
        // Ignoring length and width for Circle
        Circle { radius: 0, name }
    }

    fn area(&self) -> i32 {
        // Area of a circle = π * r^2
        (std::f64::consts::PI * (self.radius.pow(2) as f64)) as i32
    }

    fn perimeter(&self) -> i32 {
        // Perimeter of a circle = 2 * π * r
        (2.0 * std::f64::consts::PI * (self.radius as f64)) as i32
    }

    fn set_length(&mut self, _length: i32) {
        // Ignoring length for Circle
    }

    fn get_length(&self) -> i32 {
        // Ignoring length for Circle
        0
    }

    fn set_width(&mut self, _width: i32) {
        // Ignoring width for Circle
    }

    fn get_width(&self) -> i32 {
        // Ignoring width for Circle
        0
    }

    fn set_name(&mut self, name: &'static str) {
        self.name = name;
    }

    fn get_name(&self) -> &str {
        self.name
    }
}

// Struct representing a triangle implementing the Shape trait
struct Triangle {
    base: i32,
    height: i32,
    name: &'static str,
}

// Implementation of methods for Triangle
impl Triangle {
    fn new(base: i32, height: i32, name: &'static str) -> Self {
        Triangle { base, height, name }
    }
}

// Implementation of Shape trait for Triangle
impl Shape for Triangle {
    fn new(_length: i32, _width: i32, name: &'static str) -> Self {
        // Ignoring length and width for Triangle
        Triangle { base: 0, height: 0, name }
    }

    fn area(&self) -> i32 {
        // Area of a triangle = 0.5 * base * height
        (0.5 * (self.base * self.height) as f64) as i32
    }

    fn perimeter(&self) -> i32 {
        // Perimeter of a triangle = base + height + hypotenuse (ignoring the hypotenuse for simplicity)
        self.base + self.height
    }

    fn set_length(&mut self, _length: i32) {
        // Ignoring length for Triangle
    }

    fn get_length(&self) -> i32 {
        // Ignoring length for Triangle
        0
    }

    fn set_width(&mut self, _width: i32) {
        // Ignoring width for Triangle
    }

    fn get_width(&self) -> i32 {
        // Ignoring width for Triangle
        0
    }

    fn set_name(&mut self, name: &'static str) {
        self.name = name;
    }

    fn get_name(&self) -> &str {
        self.name
    }
}

pub fn run2() {
    // Example usage for Rect
    let rectangle1 = Rect::default();
    println!("{:#?}", rectangle1);
    println!("Area of rectangle1: {}", rectangle1.area());
    println!("Perimeter of rectangle1: {}", rectangle1.perimeter());

    let rectangle2 = Rect::new(2, 4, "Rectangle2");
    let rectangle3: Rect = "3,5,Rectangle3".into();

    // Comparison using PartialOrd
    let result1 = rectangle1.partial_cmp(&rectangle2);
    println!("Result of comparison (rectangle1 vs rectangle2): {:?}", result1);

    let result2 = rectangle1.le(&rectangle2);
    println!("Result of less than or equal (rectangle1 vs rectangle2): {:?}", result2);

    // Comparison using PartialEq
    let result3 = rectangle2.eq(&rectangle3);
    println!("Result of equality (rectangle2 vs rectangle3): {:?}", result3);

    let result4 = rectangle2.ne(&rectangle3);
    println!("Result of inequality (rectangle2 vs rectangle3): {:?}", result4);

    // Example usage for Circle
    let circle1 = Circle::new(2, "Circle1");
    println!("{:#?}", circle1);
    println!("Area of circle1: {}", circle1.area());
    println!("Perimeter of circle1: {}", circle1.perimeter());

    let circle2 = Circle::new(3, "Circle2");
    let circle3: Circle = "4, Circle3".into();

    // Comparison using PartialOrd
    let result5 = circle1.partial_cmp(&circle2);
    println!("Result of comparison (circle1 vs circle2): {:?}", result5);

    let result6 = circle1.le(&circle2);
    println!("Result of less than or equal (circle1 vs circle2): {:?}", result6);

    // Comparison using PartialEq
    let result7 = circle2.eq(&circle3);
    println!("Result of equality (circle2 vs circle3): {:?}", result7);

    let result8 = circle2.ne(&circle3);
    println!("Result of inequality (circle2 vs circle3): {:?}", result8);

    // Example usage for Triangle
    let triangle1 = Triangle::new(3, 4, "Triangle1");
    println!("{:#?}", triangle1);
    println!("Area of triangle1: {}", triangle1.area());
    println!("Perimeter of triangle1: {}", triangle1.perimeter());

    let triangle2 = Triangle::new(5, 12, "Triangle2");
    let triangle3: Triangle = "6,8,Triangle3".into();

    // Comparison using PartialOrd
    let result9 = triangle1.partial_cmp(&triangle2);
    println!("Result of comparison (triangle1 vs triangle2): {:?}", result9);

    let result10 = triangle1.le(&triangle2);
    println!("Result of less than or equal (triangle1 vs triangle2): {:?}", result10);

    // Comparison using PartialEq
    let result11 = triangle2.eq(&triangle3);
    println!("Result of equality (triangle2 vs triangle3): {:?}", result11);

    let result12 = triangle2.ne(&triangle3);
    println!("Result of inequality (triangle2 vs triangle3): {:?}", result12);
}

fn main() {
    run();
    run2();
}
