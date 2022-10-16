use std::{
    fs::File,
    io::{self, ErrorKind, Read},
};

fn main() {
    let greeting_file = File::open("Hello.txt");
    let greeting_file = match greeting_file {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        },
    };

    let greeting_file =
        File::open("hello.txt").expect("Hello.txt should be included in this proeject");
}

fn read_username_from_file() -> Result<String, io::Error> {
    // let mut username_file = File::open("hello.txt")?;
    // let mut username = String::new();
    // username_file.read_to_string(&mut username)?;

    let mut username = String::new();
    File::open("hello.txt")?.read_to_string(&mut username)?;

    Ok(username)
}

fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}
