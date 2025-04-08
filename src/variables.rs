use std::io;

fn main() {
    //unsigned 32 bit integer
    let guess: u32 = "42".parse().expect("Not a number!");

    //array of u32 type with length 5
    let a: [u32; 5] = [1, 2, 3, 4, 5];

    let first=a[0];
    let last=a[4];

    //access element at index i
    println!("Enter an index to access in the array a[]: ");

    let mut index = String::new();

    io::stdin().read_line(&mut index).expect("Error with input");

    //parse string input to integer with proper unsigned bit type (usize)
    let index: usize=index.trim().parse().expect("NaN");
    let value = a[index];

    println!("At index {index} of a[] we get {value}");
}