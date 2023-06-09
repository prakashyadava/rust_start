fn main(){
    let x = 2;
    let x = x * 3;

    {
        let x = x * 9;
        println!("The value of x in inner scope is {x}"); // 54
    }

    println!("The value of x in outer scope is {x}"); // 6

}