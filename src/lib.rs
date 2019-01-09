#![warn(missing_docs)]
#![deny(missing_debug_implementations)]
#![doc(html_no_source)]

//! Sigma

use pest::{error::Error, iterators::Pair, Parser};
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "sigma.pest"]
struct SigmaParser;

enum SigmaError {
  ParserError,
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum Expression<'e> {
  Variable(Variable<'e>),
  Function(Function<'e>),
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum DataType {
  U8,
  I8,
  U16,
  I16,
  U32,
  I32,
  U64,
  I64,
  Bool,
  Str,
}

impl From<&str> for DataType {
  fn from(val: &str) -> DataType {
    use self::DataType::*;
    match val {
      "u8" => U8,
      "i8" => I8,
      "u16" => U16,
      "i16" => I16,
      "u32" => U32,
      "i32" => I32,
      "u64" => U64,
      "i64" => I64,
      "bool" => Bool,
      "str" => Str,
      _ => unreachable!(),
    }
  }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
struct Variable<'a> {
  pub name: &'a str,
  pub nullable: bool,
  pub data_type: Option<DataType>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
struct Function<'a> {
  pub name: &'a str,
  pub arg_count: u8,
}

fn parse(input: &str) -> Result<Vec<Expression>, Error<Rule>> {
  let mut result = Vec::new();
  for sigma in SigmaParser::parse(Rule::sigma, input)? {
    if sigma.as_rule() == Rule::var_pair {
      let var = parse_var_pair(sigma);
      let expr = Expression::Variable(var);
      result.push(expr);
    }
  }
  Ok(result)
}

fn parse_var_pair(pair: Pair<Rule>) -> Variable {
  let mut inner_rules = pair.into_inner();
  let _open_pairs = inner_rules.next().unwrap();
  let var = inner_rules.next().unwrap();
  let mut var_inner = var.into_inner();
  let var_name = var_inner.next().unwrap();
  let mut variable = Variable::default();
  variable.name = var_name.as_str();
  if let Some(nullable_or_datatype) = var_inner.next() {
    match nullable_or_datatype.as_rule() {
      Rule::nullable => {
        variable.nullable = true;
      },
      Rule::data_type => {
        let data_type = nullable_or_datatype.as_str();
        variable.data_type = Some(data_type.into());
      },
      _ => {},
    };
  }
  variable
}
#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn it_works() {
    let input = "test {{ optinal? }} and {{ without_type }} and even with pipelines {{ username: u32 | FN_NAME }}";
    let output = parse(input).unwrap();
    let expected = [
      Expression::Variable(Variable {
        name: "optinal",
        nullable: true,
        data_type: None,
      }),
      Expression::Variable(Variable {
        name: "without_type",
        nullable: false,
        data_type: None,
      }),
      Expression::Variable(Variable {
        name: "username",
        nullable: false,
        data_type: Some(DataType::U32),
      }),
    ];
    assert_eq!(output, expected.to_vec());
  }
}
