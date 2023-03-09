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
        assert!(MyParser::parse(Rule::block, "{[std.dev]={}}").is_ok())
    }
}
