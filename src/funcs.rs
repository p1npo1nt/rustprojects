use std::io;
//functions and ctrl flow

fn main() {
    hi();
    println!("{}", greater(4, 6));
}

fn hi() {
    println!("Hello world");
}

fn add(x: i32, y:i32) -> i32 {
    x+y
}

fn greater(x: i32, y:i32) -> i32 {
    let mut r: i32;
    if x==y {
        return 0;
    }
    if x>y {
        return x;
    }
    return y;
}