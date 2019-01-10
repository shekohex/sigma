#![warn(missing_docs)]
#![deny(missing_debug_implementations)]
#![doc(html_no_source)]

//! Sigma

mod parser;

use crate::parser::{Rule, SigmaParser};
use pest::{
  error::{Error as PestError, ErrorVariant},
  iterators::Pair,
  Parser, Position, Span,
};
use std::collections::HashMap;

type SigmaResult<'a, T> = Result<T, PestError<Rule>>;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DataType {
  U8,
  I8,
  U16,
  I16,
  U32,
  I32,
  U64,
  I64,
  F32,
  F64,
  Bool,
  Str,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Variable<'a> {
  pub name: &'a str,
  pub nullable: bool,
  pub typed: bool,
  pub data_type: Option<(DataType, Span<'a>)>,
  pub location: (usize, usize),
  pub functions: Vec<(&'a str, Span<'a>)>,
  pub current_value: String,
  pub name_span: Option<Span<'a>>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Function<'b> {
  pub name: &'b str,
  pub pure: bool,
  pub call: fn(String) -> String,
}

#[derive(Debug, PartialEq, Eq, Default)]
pub struct Sigma<'s> {
  vars: HashMap<&'s str, Variable<'s>>,
  registry: HashMap<&'s str, &'s str>,
  input: String,
  functions: HashMap<&'s str, Function<'s>>,
}

impl<'s> Sigma<'s> {
  pub fn new() -> Self {
    let mut functions = HashMap::new();

    let upper_case_fn = Function {
      name: "UPPERCASE",
      pure: true,
      call: |input| input.to_uppercase(),
    };
    functions.insert("UPPERCASE", upper_case_fn);
    Self {
      input: String::new(),
      vars: HashMap::new(),
      functions,
      registry: HashMap::new(),
    }
  }

  /// Parse the input and return expressions
  pub fn parse(mut self, input: &'s str) -> SigmaResult<'s, Self> {
    for sigma in SigmaParser::parse(Rule::sigma, &input)? {
      if sigma.as_rule() == Rule::var_pair {
        let var = parse_var_pair(sigma)?;
        self.vars.insert(var.name, var);
      }
    }
    self.input = input.into();
    Ok(self)
  }

  pub fn register(mut self, key: &'s str, value: &'s str) -> Self {
    self.registry.insert(key, value);
    self
  }

  pub fn compile(mut self) -> SigmaResult<'s, String> {
    for var in self.vars.values_mut() {
      if let Some(value) = self.registry.get(var.name) {
        let mut current_data = (*value).to_owned();
        for function in &var.functions {
          let f = self.functions.get(&function.0).ok_or_else(|| {
            PestError::new_from_span(
              ErrorVariant::CustomError {
                message: format!("undefined function: {}", function.0),
              },
              function.1.clone(),
            )
          })?;
          current_data = (f.call)(current_data);
        }
        var.current_value = current_data;
        self
          .input
          .replace_range(var.location.0..var.location.1, &var.current_value);
      } else if var.nullable {
        self.input.replace_range(var.location.0..var.location.1, "");
      } else {
        return Err(PestError::new_from_span(
          ErrorVariant::CustomError {
            message: format!(
              "undefined variable: {} consider adding a register for it",
              var.name
            ),
          },
          var.name_span.clone().unwrap(),
        ));
      }
    }
    Ok(self.input)
  }
}

// TODO: Refactor this function
// TODO: Validate Datatypes
// TODO: Parse functions in seprate function
fn parse_var_pair(pair: Pair<Rule>) -> SigmaResult<Variable> {
  let mut variable = Variable::default();
  let mut inner_rules = pair.into_inner();
  let open_pairs = inner_rules.next().unwrap();
  let var = inner_rules.next().unwrap();
  let var_inner = var.into_inner();
  for var_rules in var_inner {
    match var_rules.as_rule() {
      Rule::nullable => {
        variable.nullable = true;
      },
      Rule::var_name => {
        variable.name = var_rules.as_str();
        variable.name_span = Some(var_rules.as_span());
      },
      Rule::data_type_sep => {
        // it must has data type then
        variable.typed = true;
      },
      Rule::data_type => {
        let data_type = var_rules;
        variable.data_type =
          Some((parse_data_type(&data_type)?, data_type.as_span()));
      },
      _ => {},
    };
  }

  let close_pairs_or_functions = inner_rules;
  for pair in close_pairs_or_functions {
    let rule = pair.as_rule();
    match rule {
      Rule::function => {
        if variable.data_type.is_none() || !variable.typed {
          return Err(PestError::new_from_span(
            ErrorVariant::ParsingError {
              positives: vec![Rule::data_type],
              negatives: vec![],
            },
            variable.name_span.unwrap(),
          ));
        }
        let mut function = pair.into_inner();
        let _sep = function.next().unwrap();
        let function_name = function.next().unwrap();
        variable
          .functions
          .push((function_name.as_str(), function_name.as_span()));
      },
      Rule::pair_close => {
        variable.location =
          (open_pairs.as_span().start(), pair.as_span().end());
        break;
      },
      _ => {},
    };
  }
  // data type check
  if variable.typed && variable.data_type.is_none() {
    return Err(PestError::new_from_span(
      ErrorVariant::ParsingError {
        positives: vec![Rule::data_type],
        negatives: vec![],
      },
      variable.name_span.clone().unwrap(),
    ));
  }
  Ok(variable)
}

fn parse_data_type<'b>(pair: &Pair<Rule>) -> SigmaResult<'b, DataType> {
  use self::DataType::*;
  let val = pair.as_str();
  let result = match val {
    "u8" => U8,
    "i8" => I8,
    "u16" => U16,
    "i16" => I16,
    "u32" => U32,
    "i32" => I32,
    "u64" => U64,
    "i64" => I64,
    "f32" => F32,
    "f64" => F64,
    "bool" => Bool,
    "str" => Str,
    _ => {
      return Err(PestError::new_from_span(
        ErrorVariant::CustomError {
          message: format!("Unknown Data type: {}", val),
        },
        pair.as_span(),
      ));
    },
  };
  Ok(result)
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  #[should_panic]
  fn missing_data_type() {
    let input = "{{ username: }}";
    let _ = Sigma::new()
      .parse(input)
      .unwrap()
      .register("username", "test")
      .compile()
      .unwrap();
  }

  #[test]
  #[should_panic]
  fn unknown_data_type() {
    let input = "{{ username: unknown }}";
    let output = Sigma::new()
      .parse(input)
      .unwrap()
      .register("username", "test")
      .compile()
      .unwrap();
    println!("{:?}", output);
  }
}
