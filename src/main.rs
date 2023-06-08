#![allow(dead_code)]
// use std::{mem};

// fn reverse(pair:(i32,bool)) -> (bool, i32){

//         let (int_param, bool_param) = pair;

//         (bool_param, int_param)
// }

// fn transpose(matrix:Matrix) -> Matrix{

//     let x1 = matrix.0.0;
//     let y1 = matrix.0.1;
//     let x2 = matrix.1.0;
//     let y2 = matrix.1.1;
//     Matrix((x1,x2), (y1,y2))
// }

// #[derive(Debug)]
// struct  Matrix((f32,f32),(f32,f32));

// impl fmt::Display for Matrix {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f,"({} {})\n({} {})",self.0.0, self.0.1, self.1.0, self.1.1)
//     }
// }

// #[derive(Debug)]
// struct Point2d(f32,f32);

// impl fmt::Display for Point2d {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f,"({} {})",self.0, self.1)
//     }
// }


pub mod structure;
mod tuple;

fn analyze_slice(slice: &[i32]) {
    println!("First element of array is {}",slice[0]);
    println!("Length of array is {}",slice.len());
    
}

fn main() {
    /* 
    println!("Hello, world!");
    // In general, the `{}` will be automatically replaced with any
    // arguments. These will be stringified.
    println!("{} days!",31);

    // Positional arguments can be used. Specifying an integer inside `{}`
    // determines which additional argument will be replaced. Arguments start
    // at 0 immediately after the format string.
    println!("{0} this is {1},{1} and this is {0}","Duke","Prakash");

    // As can named arguments.
    println!("name: {name}, age: {age} address: {address}",name="Prakash",age=23,address="Gorakhpur Uttar Pradesh");

    println!("{number:1>9}",number=2);

    // Different formatting can be invoked by specifying the format character
    // after a `:`.
    println!("Base 10 : {}",13);
    println!("Base 2 (binary) : {:b}",13);
    println!("Base 8 (octal): {:o}",13);
    println!("Base 16 (hexadecimal): {:x}",13);
    println!("Base 16 (hexadecimal): {:X}",13);

    #[allow(dead_code)]
    struct Structure(i32);

    println!("Hi prakash I'm here");
    let number: f64  = 1.2;
    let width: usize = 5;
    let pi: f64 = 3.141592;
    println!("{number:>width$}");
    println!("{pi:.9}");
     */
    
    // ----------------------------------Primitive----------------------------------------
    /* 
    let logical: bool = true;

    let a_float: f64 = 1.0;  // Regular annotation
    let an_integer   = 5i32; // Suffix annotation

    // Or a default will be used.
    let default_float   = 3.0; // `f64`
    let default_integer = 7;   // `i32`

    // A type can also be inferred from context.
    let mut inferred_type = 12; // Type i64 is inferred from another line.
    inferred_type = 4294967296i64;

    // A mutable variable's value can be changed.
    let mut mutable = 12; // Mutable `i32`
    mutable = 21;

    // Error! The type of a variable can't be changed.
    // mutable = true; 

    // Variables can be overwritten with shadowing.
    let mutable = true;
    */
    // integer addition
    /* 
    println!("1 + 2 = {}", 1u32 + 2);

    println!("1 - 2 = {}",1i32 - 2);

    println!("1e4 is {}, -2.5e-3 is {}", 1e4, -2.5e-3);

    // Short-circuiting boolean logic
    println!("true AND false is {}", true && false);
    println!("true OR false is {}", true || false);
    println!("NOT true is {}", !true);
    */
    // ------------------------------------------Tuple-----------------------------------------------------

    /*
    let pair: (i32, bool) = (1,true);

    println!("The pair is {:?}",reverse(pair));

    let tuple_of_tuples: ((u8, u16, u32), (u64, i8), i16) = ((1u8, 2u16, 2u32),(4u64, -1i8),-2i16);
    println!("tuple of tuples {:?}",tuple_of_tuples);

    let tuple: (i32, &str, bool, f64) = (1, "hello", false, 1.87);
    let (a, b, c, d) = tuple;
    println!("a: {:?}, b: {:?}, c: {:?}, d: {:?}",a,b,c,d);
     */

    
    /* 
    let a_point2d: Point2d = Point2d(1.0, 2.0);
    let b_point2d: Point2d = Point2d(3.0, 4.0);
    
    // println!("Matrix:\n{}\n{} ", a_point2d.0, b_point2d);
    
    let point_a = (1.0,2.0);
    let point_b = (3.0,4.0);
    
    */
    
    /*
    let matrix: Matrix = Matrix((1.0, 2.0),(3.0, 4.0));
    println!("Matrix: \n{}", matrix);
    println!("Transpose:\n{}", transpose(matrix));
     */


    // ---------------------------------------------Array and Slices-----------------------------------------------
    /* 
    // fixed size array
    let xs: [i32; 5] = [1,2,3,4,5];
    println!("Array {:?}",xs);

    // All elements initiated with same value
    let ys: [i32; 100] = [2;100];
    // indexing starts with 0
    println!("First element of Array {}",xs[0]);
    println!("Second element of Array {}",xs[1]);
    // `len` returns the length of array
    println!("Length of Array is {}",xs.len());

    // Array are stacked allocated
    println!("Array occupied {} bytes",mem::size_of_val(&xs));

    // Array can be automatically borrowed as slices
    println!("Borrow the whole array as slice");
    analyze_slice(&ys);

    // Slices can point to a section of array
    // they are the form of [starting_index..ending_index]
    // `starting_index` is the first index of slicing
    // `ending_index` is one more than the last position of slice
    println!("Borrow the some section of array as slice");
    analyze_slice(&ys[1..6]);

    // Example of empty slice
    let empty_array: [u32; 0] = [];
    assert_eq!(&empty_array, &[]);
    assert_eq!(&empty_array, &[][..]);
    println!("Empty Array: {:?}",empty_array);

    // Arrays can be safely accessed using `.get`, which returns an
    // `Option`. This can be matched as shown below, or used with
    // `.expect()` if you would like the program to exit with a nice
    // message instead of happily continue.
    for i in 0..xs.len() +1  {
        match xs.get(i) {
            Some(xval) => println!("{} {}",i,xval),
            None => println!("Slow down! {} is too far",i)    
        }
        
    }
    */
    // --------------------------------------Custom Type -------------------------------------------
    // ----------Structure------------


}

