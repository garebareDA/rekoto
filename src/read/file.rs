use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use crate::error::result;
use crate::interpreters::interpreter;
use crate::lexer::lexers;
use crate::parser::parsers;

pub fn read_file() -> Result<(), result::Error> {
    let args: Vec<String> = env::args().collect();
    if args.len() > 3 && 3 > args.len() {
        return Err(result::Error::FileReadError("too many args".to_string()));
    }

    let query = &args[1];
    if query == "run" {
        let filename = Path::new(&args[2]);
        if filename == Path::new("") {
            return Err(result::Error::FileReadError("file is empty".to_string()));
        }

        let mut f: File;
        match File::open(filename) {
            Ok(file) => {
                f = file;
            }

            Err(e) => {
                return Err(result::Error::FileReadError(e.to_string()));
            }
        }

        let mut contents = String::new();
        match f.read_to_string(&mut contents) {
            Ok(_) => {}
            Err(e) => {
                return Err(result::Error::FileReadError(e.to_string()));
            }
        }

        let mut lex = lexers::lex(&contents);
        let result = lex.run().get_tokens();
        println!("{:?}", result);

        let mut parse = parsers::Parsers::new(result.to_vec());
        let result = parse.run()?;
        println!("{:?}", result);

        let mut interpreter = interpreter::Interpreter::new(
            filename.to_str().unwrap(),
            filename.file_stem().unwrap().to_str().unwrap(),
        );
        return interpreter.run(result);
    }

    return Err(result::Error::FileReadError(format!(
        "{} command is not found",
        query
    )));
}
