extern "C" {
    fn abs(input: i32) -> i32;
}

fn main() {
    let a = unsafe { abs(-3) };
    println!("Absolute of -3 is {a}")
}
