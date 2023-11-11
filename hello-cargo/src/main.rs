mod borrow_semantics;
mod move_semantics;
fn main() {
    move_semantics::f();
    borrow_semantics::f();
}
