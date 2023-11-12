fn my_string(s: String) {
    println!("{}", s);
}
pub fn f() {
    // move semantics
    let s1 = String::from("I am s1!");
    my_string(s1);
    // println!("{}", s); This will error because s has been moved to my_string
}
