use rekoto::lexer::lexers;
use rekoto::parser::parsers;

fn main() {
    let mut lex = lexers::lex("let a = a(b);");
    let result = lex.run().get_tokens();
    println!("{:?}", result);

    let mut parse = parsers::Parsers::new(result.to_vec());
    let result = parse.run();
    println!("{:?}", result);
}
