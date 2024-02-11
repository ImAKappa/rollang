use rollang::parser::Parser;
use std::io::{self, Write};

fn main() -> io::Result<()> {
    println!("Rollang v0.1.0");

    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut stderr = io::stderr();

    let mut input = String::new();
    // let mut env = rollang::Env::default();

    loop {
        write!(stdout, ">> ")?;
        stdout.flush()?;

        stdin.read_line(&mut input)?;
        let parse = Parser::new(&input).parse();
        println!("{}", parse.debug_tree());

        // match run(input.trim(), &mut env) {
        //     Ok(Some(val)) => writeln!(stdout, "{}", val)?,
        //     Ok(None) => {}
        //     Err(msg) => writeln!(stderr, "{}", msg)?,
        // }

        input.clear();
    }
}

// fn run(input: &str, env: &mut rollang::Env) -> Result<Option<rollang::Val>, String> {
//     let parse = eldiro::parse(input).map_err(|msg| format!("Parse error: {}", msg))?;

//     let evaluated = parse
//         .eval(env)
//         .map_err(|msg| format!("Evaluation error: {}", msg))?;

//     if evaluated == rollang::Val::Unit {
//         Ok(None)
//     } else {
//         Ok(Some(evaluated))
//     }
// }
