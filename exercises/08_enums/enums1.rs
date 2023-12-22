// enums1.rs
// No hints this time! ;)


#[derive(Debug)]
enum Message {
    // TODO: define a few types of messages as used below
    Quit,
    Echo(String),
    Move { x:i32, y:i32 },
    ChangeColor { r:i32, g:i32, b:i32 },
}

fn main() {
    println!("{:?}", Message::Quit);
    println!("{:?}", Message::Echo(String::from("test")));
    println!("{:?}", Message::Move{x:1, y:2});
    println!("{:?}", Message::ChangeColor{r:123, g:321, b:1});
}
