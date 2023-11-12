fn my_string(s: &String) -> &String {
    s
}
pub fn f() {
    // borrow semantics
    let s1 = String::from("I am s1!");
    my_string(&s1);
    println!("{}", s1); // This will work because s1 is borrowed by my_string
}
