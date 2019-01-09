use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "sigma.pest"]
struct SigmaParser;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let input = "test {{ test: u32 }}";
        let output = SigmaParser::parse(Rule::sigma, &input)
            .unwrap()
            .next()
            .unwrap();
        println!("{}", output);
        assert_eq!(2 + 2, 4);
    }
}
