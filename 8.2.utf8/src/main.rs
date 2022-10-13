fn main() {
    let s = "Hello".to_string();

    let s1 = String::from("Hello, ");
    let s2 = String::from("World");
    let s3 = s1 + &s2;
    // println!("{}", s1);

    let s1 = String::from("Hello, ");
    let s = format!("{}-{}-{}", s1, s2, s3);
}
