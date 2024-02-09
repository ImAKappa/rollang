use std::io::{self, Write};

fn main() {
    println!("Rollang v0.1.0");

    loop {
        print!(">>> ");
        io::stdout().flush().expect("Issue flushing stdout buffer");

        let mut command = String::new();
        io::stdin()
            .read_line(&mut command)
            .expect("Failed to read line");
        let command = command.trim();

        if command == "end" {
            println!("~~~ End of Session ~~~");
            break;
        } else {
            println!("{command}");
        }
    }
}
