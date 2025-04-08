use std::io;

fn main() {
    let mut s = String::from("Hello world");
    let word = first_word(&s); //returns 5 for end of first word
    s.clear(); //clears string, word will still work fine after this but we get a bug if we try to access the first word of s since its contents have changeed
    //NO ERROR ^^

    //let's try again as to avoid the bug:
    let mut s2 = String::from("Hello world");
    let word = first_word_modified(&s2);
    s2.clear(); 
    println!("The first word is: {word}"); //ERROR: A variable already borrowed as immutable was borrowed as mutable.
    //recall that if we have an immutable reference to something, we cannot take a mutable reference of it too
    //since .clear() truncates String s2, it NEEDS to get a mutable reference
    //however, since word is an immmutable reference to String s2, we have contradicted and rust throws an error
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes(); //convert s to an array of bytes

    for (i, &item) in bytes.iter().enumerate() { //create an iterator over the array of bytes, 
                                                 //enumerate wraps iter result and returns each element as a part of a tuple (index, element reference)
        if item == b' ' { //if we find a byte represting a space, we return position
            return i;
        }
    }

    s.len() //otherwise return length
}

fn first_word_modified(s: &String) -> &str { //we now modify first_word to return a string slice (&str) instead of indicies
    let bytes = s.as_bytes(); //convert s to an array of bytes

    for (i, &item) in bytes.iter().enumerate() { //create an iterator over the array of bytes, 
                                                 //enumerate wraps iter result and returns each element as a part of a tuple (index, element reference)
        if item == b' ' { 
            return &s[0..i];
        }
    }

    &s[..] //return entire string
}
