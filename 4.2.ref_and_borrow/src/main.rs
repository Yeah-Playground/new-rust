fn main() {
    let s1 = String::from("Hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}", s1, len);

    let mut s = String::from("hello");
    change(&mut s);

    {
        let mut s = String::from("hi");
        let _r1 = &mut s;
        // let r2 = &mut s;
        // println!("{}, {}", r1, r2);
        // Cannot borrow multiple mutable
        // Multiple immutable borrow is ok
        // But combine with mutable and immutable is not ok
    }

    let r1 = &s;
    let r2 = &s;
    println!("{} and {}", r1, r2);
    let r3 = &mut s;
    println!("{}", r3);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", World");
}

// Dangling Pointer
// fn dangle() -> &String {
//     let s = String::from("World");
//     &s
// }
