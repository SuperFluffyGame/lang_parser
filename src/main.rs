fn main() {
    let s = "2 + 3 * 3";

    let tokens = math::lexer::tokenize(s);

    println!("{:?}", tokens);

    let expr = math::parser::parse(tokens);

    println!("{:?}", expr);
}
