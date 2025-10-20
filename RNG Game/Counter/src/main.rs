use std::io;

fn main() {
    println!("Welcome to Solana Trader Simulator 🪙");
    println!("Type 'pump' to push price up, 'dump' to crash it, or 'rug' to exit.\n");

    let mut price:i32 = 0;
    println!("Initial price: {}", price);
    loop {
        let mut button:String = String::new();

        io::stdin()
            .read_line(&mut button)
            .expect("Unexpected Error happened while reading input");

        let input = button.trim();

        match input {
            "pump" => {
                price += 10;
                println!("Price pumped by 10$.");
            }
            "dump" => {
                price -= 1;
                println!("Dumped by 1$ coz it's hard to dump on Solana kek");
            }
            "rug" => {
                println!("Rugging coz I am SBF");
                break;
            }
            _ => {
                println!("Invalid input. Please type 'pump', 'dump' or 'rug'.");
                continue;
            }
        }
         println!("Current Price : {}", &price);

    }
}