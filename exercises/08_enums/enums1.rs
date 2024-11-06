#[derive(Debug)]
enum Message {
    Resize{w: u32, h: u32},
    Move{x : i32 , y: i32 },
    Echo {String},
    ChangeColor{r: u8, g: u8, b: u8 },
    Quit,
    
    // TODO: Define a few types of messages as used below.
}
fn main() {
    println!("{:?}", Message::Resize { width: 800, height: 600 });
    println!("{:?}", Message::Move { x: 10, y: 20 });
    println!("{:?}", Message::Echo(String::from("Hello, world!")));
    println!("{:?}", Message::ChangeColor { r: 255, g: 165, b: 0 });
    println!("{:?}", Message::Quit);
}
