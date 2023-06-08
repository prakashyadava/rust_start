#![allow(dead_code)]
use std::fmt;

#[derive(Debug)]
struct Person{
    name: String,
    age: u8,
}

struct Pair(i32, f32);
#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}
#[derive(Debug)]
struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

impl fmt::Display for Rectangle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,"({}, {}), ({}, {})", self.top_left.x, self.top_left.y, self.bottom_right.x, self.bottom_right.y)
    }
}

fn main(){
    
    let name = String::from("Prakash");
    let age = 23;
    let prakash = Person {name, age};
    println!("Name: {} age: {}", prakash.name, prakash.age);

    let point = Point { x:2.1, y: 3.4 };
    println!("Point coordinates: ({}, {})", point.x, point.y);

    let bottom_right = Point { x: 10.7, ..point};
    println!("Second Point: ({}, {})",bottom_right.x, bottom_right.y);

    let Point { x: left_edge, y: top_edge} = point;

    let rectangle = Rectangle {
        top_left: Point { x: left_edge, y: top_edge },
        bottom_right: bottom_right
    };
    println!("Rectangle matrix: {}", rectangle);


}