#[derive(Debug)]
enum Message {
    // TODO: Define a few types of messages as used below.
    Quit,
    Move {x: i32, y: i32},
    Echo(String),
    ChangeColor(u8, u8, u8),
    Resize(u32, u32),
}

fn main() {
    println!("{:?}", Message::Resize(1000, 4080));
    println!("{:?}", Message::Move {x: 10, y: 20});
    println!("{:?}", Message::Echo("Hello!" .into()));
    println!("{:?}", Message::ChangeColor(255, 0, 0));
    println!("{:?}", Message::Quit);
}
