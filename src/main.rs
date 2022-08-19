fn main() {
    let s = "let a = 1";

    let tokens = math::lexer::tokenize(s);

    println!("{:?}", tokens);

    let expr = math::parser::parse(tokens);

    println!("{:?}", expr);
}
