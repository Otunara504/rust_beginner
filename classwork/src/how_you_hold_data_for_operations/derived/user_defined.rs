#![allow(dead_code)] //This suppresses warnings when a given declared function is  not used.
#![allow(unused_variables)]

use core::cmp::Ordering; //Used dor comparison of value sizes 

pub enum Comp { //Enumerate Comparison
    LessThan,
    GreaterThan,
    Equal,
}

pub enum Gender { //Enumerate Gender
    Male,
    Female,
}

#[derive(Debug)] //Decorate the struct Person. Debug is an inbuilt trait. This statement will provoke impl Debug for Person; Metaprogramming
struct Person {
    name: String,
    age: u8,
}
struct Unit;
// A unit struct
//Have no state of their own but useful for
//implementing some trait.
//E.g. struct Global is a unit struct that can implement traits like Allocator
//std::fmt::Error is a unit struct that implements
//traits like Error

//A tuple struct
struct Pair(i32, f32); //No named fields but has fields

//A struct with two fields
struct Point {
    x: f32,
    y: f32,
}

// Structs can be reused as fields of another struct. Below Point is used as datatype in Rectangle
struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

pub fn run() {

    //declare a variable of type Person and assign values.
    let person = Person {
        name: String::from("Peter"),
        age: 27,
    };
    println!("{:#?}", person); //{:#?} implies pretty debug print person. :? is for debug print and :#? is for pretty debug print

    // Instantiate a unit struct
    let _unit = Unit;//As simple as that. If Unit implements some trait, then _unit will demand those implementations

    //declare a Point
    let point = Point { x: 10.3, y: 0.4 };

    //Access the fields of the point
    println!("point coordinates: ({}, {})", point.x, point.y);

    // Make a new point by using struct update syntax to use the fields of our
    // other one
    let bottom_right = Point { x: 5.2, ..point };
    // `bottom_right.y` will be the same as `point.y` because we used that field
    // from `point`
    println!("second point: ({}, {})", bottom_right.x, bottom_right.y);

    // Destructure the point using a let binding.
    let Point {
        x: left_edge,//left_edge here will be declared. If you use x:f32 only, x will be declared.
        y: top_edge,//top_edge here will be declared. If you use y:f32 only, y will be declared.
    } = point;
    dbg!(&left_edge,&top_edge);


    let _rectangle = Rectangle { //I used _ with rectangle to silence warning knowing that the variable is not used.
        //struct instantiation is an expression too as used below in Point
        top_left: Point {
            x: left_edge,//left_edge is available, thanks to the destructuring above
            y: top_edge,//top_edge is available, thanks to the destructuring above
        },
        bottom_right,
    };

    //Instantiate a tuple struct
    let pair = Pair(1, 0.1);
    //Access the fields of a tuple struct
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    //Destructure a tuple struct
    let Pair(integer, decimal) = pair;
    println!("pair contains {:?} and {:?}", integer, decimal);
}

//Let's work on user-defined traits. Traits enable us achieve polymorphism.
//We are designing Shape below for the purpose of 
//specifying all expected functions and methods in any struct that implements Shape.
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
//The use of 'static lifetime above ensures that our
//compiler is clear about the availability of those values, as they are borrowed.
//static will be available throughout the lifetime of the program.

///Use Default to specify the lengthlability of default instance creation without values passed for property
#[derive(Default, Debug, Clone)]
struct Rect {
    length: i32,
    width: i32,
    name: &'static str,
}

impl Rect {
    //default default() function. Will override derived default if any. 
    fn default() -> Self {
        Rect {
            length: 1,
            width: 1,
            name: "Rectangle",
        }
    }
}

impl Shape for Rect {
    //Associated function used to create a new Shape
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
        (self.length * 2) + (self.width * 2)
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

//implement Partial Eq
impl PartialEq for Rect {
    fn eq(&self, other: &Self) -> bool {
        self.area() == other.area()
    }
fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

impl PartialOrd for Rect {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.area().partial_cmp(&other.area())
    }
    // Provided methods
    //fn lt(&self, other: &Rhs) -> bool { ... }
    //fn le(&self, other: &Rhs) -> bool { ... }
    //fn gt(&self, other: &Rhs) -> bool { ... }
    //fn ge(&self, other: &Rhs) -> bool { ... }
}


//A conversion implementation into String
//Expects a string slice with length, width, name, separated by commas
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

//A conversion implementation into String for Rect.
/*impl Into<String> for Rect {
    fn into(self) -> String {
        //Let's return a string template literal
        format! ("My name is {} and my area is {}.", self.name);

    }
}*/

struct Circle {
    radius:i32,
    name: &'static str,
}

impl Circle {
    fn new(radius: i32, name: &'static str) -> Self {
        Circle {
            radius,
            name,
        }
    }
}

impl Shape for Circle {
    //Associated function used to create a new Shape
    fn new(_length: i32, _width: i32, name: &'static str) -> Self {
        Circle {
            radius: 0,
            name,
        }
    }

    fn area(&self) -> i32 {
        let pie = 22/7;
        pie * self.radius *self.radius
    }

    fn perimeter(&self) -> i32 {
        let pie =22/7;
        2 * pie * self.radius
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

impl PartialEq for Circle {
    fn eq(&self, other: &Self) -> bool {
        if let (Some(area_cmp), Some(perimeter_cmp)) = (self.area().partial_cmp(&other.area()), self.perimeter().partial_cmp(&other.perimeter())) {
            area_cmp == Ordering::Equal && perimeter_cmp == Ordering::Equal
        }
        else {
            false
        }
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

impl PartialOrd for Circle {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self.area().partial_cmp(&other.area()), self.perimeter().partial_cmp(&other.perimeter())) {
            (Some(area_cmp), Some(perimeter_cmp)) => {
                if area_cmp == Ordering::Equal && perimeter_cmp == Ordering::Equal {
                    Some(Ordering::Equal)
                } else {
                    Some(area_cmp.then_with(|| perimeter_cmp))
                }
            }
            _ => None,
        }
    }
}

impl From<&'static str> for Circle {
    fn from(s: &'static str) -> Circle {
        let mut parts = s.split(',');
        let radius = match parts.next() {
            Some(val) => val.parse::<i32>().unwrap(),
            None => 0,
        };
        let name = match parts.next() {
            Some(val) => val,
            None => "",
        };
        Circle { radius, name: &name }
    }
}

struct Triangle {  // My Triangle is a Right-angled
    height:i32,
    base:i32,
    name: &'static str,
}

impl Triangle {
    fn new(base: i32, height: i32, name: &'static str) -> Self {
        Triangle {
            height,
            base,
            name,
        }
    }
}

impl Shape for Triangle {
    //Associated function used to create a new Shape
    fn new(_length: i32, _width: i32, name: &'static str) -> Self {
        Triangle {
            base: 0, 
            height: 0, 
            name,
        }
    }

    fn area(&self) -> i32 {
        (1/2) * self.base * self.height
    }

    fn perimeter(&self) -> i32 {
        let hypothenus = (self.base*self.base) + (self.height*self.height);
        let square_root: i32  = (hypothenus as f64).sqrt() as i32;
        square_root + self.base + self.height
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

impl PartialEq for Triangle {
    fn eq(&self, other: &Self) -> bool {
        if let (Some(area_cmp), Some(perimeter_cmp)) = (self.area().partial_cmp(&other.area()), self.perimeter().partial_cmp(&other.perimeter())) {
            area_cmp == Ordering::Equal && perimeter_cmp == Ordering::Equal
        }
        else {
            false
        }
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

impl PartialOrd for Triangle {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self.area().partial_cmp(&other.area()), self.perimeter().partial_cmp(&other.perimeter())) {
            (Some(area_cmp), Some(perimeter_cmp)) => {
                if area_cmp == Ordering::Equal && perimeter_cmp == Ordering::Equal {
                    Some(Ordering::Equal)
                } else {
                    Some(area_cmp.then_with(|| perimeter_cmp))
                }
            }
            _ => None,
        }
    }
}

impl From<&'static str> for Triangle {
    fn from(s: &'static str) -> Triangle {
        let mut parts = s.split(',');
        let height = match parts.next() {
            Some(val) => val.parse::<i32>().unwrap(),
            None => 0,
        };
        let base = match parts.next() {
            Some(val) => val.parse::<i32>().unwrap(),
            None => 0,
        };
        let name = match parts.next() {
            Some(val) => val,
            None => "",
        };
        Triangle { height, base, name: &name }
    }
}

/*struct Circles {
    radius: f32
}
impl Into<Circles> for Rect {
    fn into(self) -> Circles {
        //Let's return a Circle
        let radius: f32 = f32::sqrt(self: self.area() as f32/3.14);
        Circles(radius);
    }
}*/

pub fn run2() {

    //Instantiation for Rectangle
    let rectangle1 = Rect::default();
    
    println!("The length of this rectangle is {}", rectangle1.length);
    println!("The width of this rectangle is {}", rectangle1.width);
    println!("The name of this rectangle is {}", rectangle1.name);
    println!("Area of rectangle1: {}", rectangle1.area());
    println!("Perimeter of rectangle1: {}", rectangle1.perimeter());

    let rectangle2 = Rect::new(1, 3, "Rectangle2");
    let rectangle3 = Rect::from("4,5,Rectangle3");

    //Compare using PartialOrd
    let result1 = rectangle1.partial_cmp(&rectangle2);
    println!(" result1 = {:?}", result1);

    let result2 = rectangle1.le(&rectangle2);
    println!("result2 = {:?}", result2);

    //Compare using PartialEq
    let result3 = rectangle2.eq(&rectangle3);
    println!("result3 = {:?}", result3);

    let result4 = rectangle2.ne(&rectangle3);
    println!("result4 = {:?}", result4);

    // Instantiation for Circle
    let circle1 = Circle::new(2, "Circle1");
    println!("The radius of this circle is {}", circle1.radius);
    println!("The name of this circle is {}", circle1.name);
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

    // Instantiation for Triangle
    let triangle1 = Triangle::new(3, 4, "Triangle1");
    println!("The height of this triangle is {}", triangle1.height);
    println!("The base of this triangle is {}", triangle1.base);
    println!("The name of this triangle is {}", triangle1.name);
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

