fn main() {
    // Constant vs Immutable variable
    // no mut keyword
    // type must be annotated
    // const can be daclard in any scope (including global scope)
    // const value must not be computed during runtime

    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("{THREE_HOURS_IN_SECONDS}");

    let x = 5;
    let x = x + 1;

    {
        let x = x + 2;
        println!("The value of x in scope is {x}");
    }
    println!("The value of x is {x}");

    let size: isize = 32;
    println!("{size}");

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (_, y, _) = tup;
    println!("The value of y i s {y}");
    println!("five hundred is {}", tup.0);

    let months = [
        "Jan", "Feb", "Mat", "Apr", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
    ];
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let b = [3; 5]; // [3, 3, 3, 3, 3]
}
