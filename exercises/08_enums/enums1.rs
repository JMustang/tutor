#[derive(Debug)]
struct Point(i32, i32);

#[derive(Debug)]
enum Message {
    // TODO: Defina alguns tipos de mensagens, conforme usado abaixo.
    Resize { width: u64, height: u64 },
    Move(Point),
    Echo(String),
    ChangeColor(u8, u8, u8),
    Quit,
}

fn main() {
    println!(
        "{:?}",
        Message::Resize {
            width: 800,
            height: 600
        }
    );
    println!("{:?}", Message::Move(Point(10, 20)));
    println!("{:?}", Message::Echo("Rust".to_string()));
    println!("{:?}", Message::ChangeColor(126, 64, 32));
    println!("{:?}", Message::Quit);
}
