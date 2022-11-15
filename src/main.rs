// extern crate libc;
mod bindings;
mod tests;

#[link(name = "double")]
extern "C" {
    fn double_input(input: i32) -> i32;
}

fn main() {
    let input = 5;

    let output = unsafe { double_input(input) };
    println!("{} * 2 = {}", input, output);
}
