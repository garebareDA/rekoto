use rekoto::lexer::lexers;
use rekoto::parser::parsers;

fn main() {
    let mut lex = lexers::lex("let a = 1");
    let result = lex.run().get_tokens();
    println!("{:?}", result);
    let mut perse = parsers::Persers::new(result.to_vec());
    let result = perse.run();
    println!("{:?}", result);
}
