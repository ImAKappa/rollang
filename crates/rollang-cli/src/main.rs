use rollang;
use std::io::{self, Write};

fn main() {
    println!("Rollang v0.1.0");

    'repl: loop {
        print!(">>> ");
        io::stdout().flush().expect("Issue flushing stdout buffer");

        let mut command = String::new();
        io::stdin()
            .read_line(&mut command)
            .expect("Failed to read line");
        let command = command.trim();

        let tokens = rollang::parse(command);

        for token in tokens {
            match token {
                Ok(token) => {
                    if token == rollang::lexer::SyntaxKind::End {
                        println!("~~~ End of Session ~~~");
                        break 'repl;
                    } else {
                        println!("{:?}", token);
                    }
                }
                Err(err) => println!("{:?}", err),
            }
        }
    }
}
