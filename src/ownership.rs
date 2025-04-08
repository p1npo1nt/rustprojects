use std::io;
//for example String is stored on the heap, a String thing has 3 data fields
//a pointer to heap memory, a length, and a capacity
//so for example s1 = String::from("hello"); s2 = s1; would make s1 invalid since s1 went out of scope and got "moved" to s2

fn main() {
    let s = String::from("hello"); //s comes into scope
    takes_ownership(s); //s "moves" into the take_ownership function
    //now if we try to use s after the take_ownership() call, Rust would through an error 

    let s1 = String::from("Vasu");
    let len = calculate_length(&s1); //since we use a reference instead of s1 itself, we can continue using s1 after this function call
    
    let mut s2 = String::from("I am "); 
    append_name(&mut s2); //using a mutable reference so we can actually change the string name
    
    let danling_pointer = danlge(); //danling pointer
}

fn takes_ownership(some_string: String) { //some_string comes into scope
    println!("{some_string}");
} //some_string goes out of scope

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn append_name(s: &String) {
    s.push_str(" Steve");
}

fn dangle() -> &String { //returns a dangling pointer
    let s = String::from("Hello"); //s comes into scope
    &s //return pointer to s
} //s goes out of scope so when we call dangle() in main we get a dangling pointer