fn main() {
    let s = "";

    let tokens = math::lexer::tokenize(s);

    println!("{:?}", tokens);

    let expr = math::parser::parse(tokens);

    println!("{:?}", expr);
}
