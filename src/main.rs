use rekoto::lexer::lexers;
use rekoto::parser::parsers;
use rekoto::interpreters::interpreter;

fn main() {
    let mut lex = lexers::lex("for let i = 0; i < 5; i++; {}");
    let result = lex.run().get_tokens();
    println!("{:?}", result);

    let mut parse = parsers::Parsers::new(result.to_vec());
    let result = parse.run();
    println!("{:?}", result);
    match result {
        Ok(result) => {
            let mut interpreter = interpreter::Interpreter::new();
            match interpreter.run(result) {
                Ok(_) => {}
                Err(e) => {
                    eprintln!("{}", e);
                }
            }
        }

        Err(e) => {
            eprintln!("{}", e);
        }
    }
}
