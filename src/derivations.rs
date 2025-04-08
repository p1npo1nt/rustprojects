use std::io;

#[derive(PartialEq, Clone)]
struct Point<T> {
    x: i32,
    y: T,
}

fn main() {
    let mut p1 = Point{x: 5, y: 6};
    let mut p2 = Point{x:5, y:6};

    println!("{}", PartialEq(p1,p2));
}

