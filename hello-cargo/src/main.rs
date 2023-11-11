fn my_string(s: String) {
    println!("{}", s);
}

fn my_string2(s: &String) {
    println!("{}", s);
}

fn main() {
    // move semantics
    let s1 = String::from("I am s1!");
    my_string(s1);
    // println!("{}", s); This will error because s has been moved to my_string

    // borrow semantics
    let s2: String = String::from("I am s2!");
    my_string2(&s2);
    println!("{}", s2); // This will work because s2 is borrowed by my_string2
}
