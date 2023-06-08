#![allow(dead_code)]

use std::fmt;

#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

fn reverse(pair: (i32, bool)) -> (bool,i32){
    let (a,b) = pair;
    (b,a)
}

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,"{}, {}, {}, {}",self.0, self.1, self.2, self.3)
    }
}

fn main(){
    let _tup: (i32, &str, bool, f64) = (1, "Duke", false, 1.2);
    let list: [i32; 4]  = [1,2,3,4];
    let pair: (i32, bool) = (1, true);
    println!("Pair is {:?}", pair);
    println!("The reverse of pair is {:?}", reverse(pair));

    let matrix = Matrix(1.2, 2.3, 4.3, 4.4);

    println!("Elements of matrix are {}", matrix);

    for i in 0..list.len() + 1 {
        match list.get(i) {
            Some(lval) => println!("index: {}, value: {}",i, lval),
            None => println!("Slow down! {} is too far", i),
            
        }
    }
}