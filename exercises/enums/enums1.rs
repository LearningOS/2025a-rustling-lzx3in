// enums1.rs
//
// No hints this time! ;)

#[derive(Debug)]
enum Message {
    Quit,
    Echo(String),
    Move{a: i32, b: bool},
    ChangeColor(i32, i32),
}

fn main() {
    println!("{:?}", Message::Quit);
    println!("{:?}", Message::Echo("I am Echo".to_string()));
    println!("{:?}", Message::Move{b:true, a:1});
    println!("{:?}", Message::ChangeColor(1, 2));
}
