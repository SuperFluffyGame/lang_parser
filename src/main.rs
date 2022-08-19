fn main() {
    let s = "(-1) ^ 2 ";

    let tokens = math::lexer::tokenize(s);

    println!("{:?}", tokens);

    let expr = math::parser::parse(tokens);

    println!("{:?}", expr);
}
