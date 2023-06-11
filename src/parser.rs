use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammar.pest"] // relative to src
struct MyParser;

#[cfg(test)]
mod tests {
    use pest::Parser;

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_parser() {
        let parsed = MyParser::parse(
            Rule::block_body,
            "[std.dev]={}[hello]={}let test = \"jel;lo\";",
        );
        assert!(parsed.is_ok());

        let pairs = parsed.unwrap();

        println!("parsed: {pairs}");
        for pair in pairs {
            let tokens: Vec<_> = pair.tokens().collect();

            for token in tokens {
                println!("token: {token:?}");
            }
        }
    }
}
