fn main() {
    let mut v: Vec<i32> = Vec::new(); // empty
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
    let v = vec![1, 2, 3];

    // Reading value
    let mut v = vec![1, 2, 3, 4, 5];
    let third = &v[2];
    println!("The third element is {}", third);

    let third = v.get(2);
    if let Some(value) = third {
        println!("The third element is {}", value);
    } else {
        println!("There is no third value");
    }

    // iterating
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }

    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }

    // enum
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    use SpreadsheetCell::*;

    let row = vec![Int(3), Text(String::from("blue")), Float(10.12)];
}
