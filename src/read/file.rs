use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use crate::error::result;
use crate::interpreters::interpreter;
use crate::lexer::lexers;
use crate::parser::ast::ast;
use crate::parser::parsers;
use crate::wasm_compiler;

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

        let mut interpreter = interpreter::Interpreter::new(
            filename.to_str().unwrap(),
            filename.file_stem().unwrap().to_str().unwrap(),
        );
        return interpreter.run(read(filename)?);
    }

    if query == "compile" {
        let filename = Path::new(&args[2]);
        if filename == Path::new("") {
            return Err(result::Error::FileReadError("file is empty".to_string()));
        }

        let mut compiler = wasm_compiler::compiler::Compiler::new();

        let mut file = File::create("rekoto/html/main.wasm").expect("file create error");
        file.write_all(&compiler.compile()?).expect("write error");
        file.flush().expect("flush error");

        return Ok(());
    }

    return Err(result::Error::FileReadError(format!(
        "{} command is not found",
        query
    )));
}

fn read(filename: &Path) -> Result<ast::RootAST, result::Error> {
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

    return Ok(result);
}
