use rekoto::lexer::lexers;

fn main() {
    let mut lex = lexers::lex("let a = 1");
    let result = lex.run().get_tokens();
    println!("{:?}", result);
}
