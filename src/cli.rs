use std::io::{self, Write};

pub fn start() {
    println!("🟢 Secure Chat Terminal");
    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_ok() {
            let input = input.trim();
            if input == "/quit" {
                break;
            }
            println!("Command received: {}", input);
            // TODO: parse and handle commands
        }
    }
}

