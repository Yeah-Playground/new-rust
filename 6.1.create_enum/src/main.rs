enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(u8, u8, u8),
}

impl Message {
    fn call(&self) {}
}

// enum Option<T> {
//     None,
//     Some(T),
// }

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky Penny");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quater from {:?}!", state);
            1
        }
    }
}

fn main() {
    let m = Message::Write(String::from("Hello"));
    m.call();

    let x: i8 = 5;
    let _y: Option<i8> = Some(x);
    let _absent_number: Option<i32> = None;

    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The max value is {}", max);
    }
}
