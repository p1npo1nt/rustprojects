use std::io;

fn main() {
    let mut user1 = User { //mutable instance of User
        active: true,
        username: String::from("bob"),
        email: String::from("bob@gmail.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("bob2@gmail.com"); //change value of email field

    let mut user2 = User {
        email: String::from("sally@gmail.com"),
        ..user1 //copy the rest of the attributes from user1
    };

    let rect1 = Rectangle { //init rectangle
        width:30,
        height:50,
    };

    println!(rect1.area()); //like calling a Java class method
}

struct User { //like a class
    active: bool, //typed fields
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle { //implementation block, everything within this impl block will be associated with Rectangle
    fn area(&self) -> u32 {
        self.width*self.heightP
    }
}