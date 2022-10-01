fn main() {
    let mut s = String::from("Hello");
    s.push_str(", worlds!");
    println!("{}", s);

    let s1 = String::from("Hello");
    let s2 = s1;
    // println!("{}, world!", s1);
    println!("{}, world!", s2);

    let s1 = String::from("Hello");
    let s2 = s1.clone();
    println!("s1 = {s1}, s2 = {s2}");

    let s = String::from("Hello");
    takes_ownership(s);
    // s is deleted
    let x = 5;
    makes_copy(x);

    let s1 = gives_ownership();
    let s2 = String::from("Hello");
    let s3 = takes_and_gives_back(s2);
    // println!("{}", s2);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn gives_ownership() -> String {
    let some_string = String::from("yours");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}
