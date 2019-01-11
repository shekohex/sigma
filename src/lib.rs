#![warn(missing_docs)]
#![deny(missing_debug_implementations)]
#![doc(html_no_source)]

//! # Sigma: Simple, Safe and Fast Template language.
//!
//! ##### Simple:
//!
//! sigma is a very simple template language, it only tries to solve only one
//! problem. it also extendable, but with simple idea too (_Pure Functions_).
//!
//! ##### Safe:
//!
//! sigma is also typed, that means that it has the idea of built-in validators
//! for your input. and for those how wanna play, it also could be untyped.
//! also it has a good error checking at parse time of your template:
//! Here is some error examples:
//! ```
//! --> 1:49
//!   |
//! 1 | my username is {{ username: str |> UPPERCASE |> NO_FUN }} WOW!
//!   |                                                 ^----^
//!   |
//!   = undefined function: NO_FUN
//! ```
//!
//! what if you fogot to register for some variable in your template ?
//!
//! ```
//!  --> 1:19
//!   |
//! 1 | my username is {{ username: str |> UPPERCASE |> NO_FUN }} WOW!
//!   |                   ^------^
//!   |
//!   = undefined variable: username consider adding a register for it
//! ```
//!
//! ##### Fast:
//!
//! sigma uses [`pest`](https://pest.rs/), The Elegant Parser under the hood to write it's grammar.
//! that means it will be exteramly fast in parsing your templete.
//!
//! // TODO: Add some benchmarks here
//!
//! ### Examples
//!
//! here is a simple examples of how it works
//!
//! * Simple:
//!
//! ```rust
//! use sigma::Sigma;
//!   
//! let result = Sigma::new()
//!  .parse("Hello {{ username }}") // using {{ ... }} for the template.
//!  .map_err(|e| println!("{}", e))? // for pretty printing the error..
//!  .register("username", "someone")
//!  .compile()
//!  .map_err(|e| println!("{}", e))?;
//! assert_eq!("Hello someone", result);
//! ```
//! * with optinal variables
//! ```rust
//! use sigma::Sigma;
//!   
//! let result = Sigma::new()
//!  .parse("Hello {{ username? }}") // using `?` to tell the parser it maybe `null`.
//!  .map_err(|e| println!("{}", e))? // for pretty printing the error..
//!  .compile()
//!  .map_err(|e| println!("{}", e))?;
//! assert_eq!("Hello ", result);
//! ```
//! * what about types ?
//!
//! ```rust
//! use sigma::Sigma;
//!   
//! let result = Sigma::new()
//!  .parse("Hello {{ username: str }}") // u8, u32 ? bool ! use all ?.
//!  .map_err(|e| println!("{}", e))? // for pretty printing the error..
//!  .register("username", "someone")
//!  .compile()
//!  .map_err(|e| println!("{}", e))?;
//! assert_eq!("Hello someone", result);
//! ```
//! * how about functions ?
//! ```rust
//! use sigma::Sigma;
//!   
//! let result = Sigma::new()
//!  .parse("Hello {{ username: str | UPPERCASE }}") // functions uses the `|` operator or if you love `|>`.
//!  .map_err(|e| println!("{}", e))? // for pretty printing the error..
//!  .register("username", "someone")
//!  .compile()
//!  .map_err(|e| println!("{}", e))?;
//! assert_eq!("Hello SOMEONE", result);
//! ```
mod parser;

use crate::parser::{Rule, SigmaParser};
use pest::{
  error::{Error as PestError, ErrorVariant},
  iterators::{Pair, Pairs},
  Parser, Span,
};
use std::collections::HashMap;

type SigmaResult<'a, T> = Result<T, PestError<Rule>>;

/// Primitive Data Types
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DataType {
  /// The 8-bit unsigned integer type.
  U8,
  /// The 8-bit signed integer type.
  I8,
  /// The 16-bit unsigned integer type.
  U16,
  /// The 16-bit signed integer type.
  I16,
  /// The 32-bit unsigned integer type.
  U32,
  /// The 32-bit signed integer type.
  I32,
  /// The 64-bit unsigned integer type.
  U64,
  /// The 64-bit signed integer type.
  I64,
  /// The 32-bit floating point type.
  F32,
  /// The 64-bit floating point type.
  F64,
  /// The boolean type.
  Bool,
  /// String.
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

    let lower_case_fn = Function {
      name: "LOWERCASE",
      pure: true,
      call: |input| input.to_lowercase(),
    };
    functions.insert("UPPERCASE", upper_case_fn);
    functions.insert("LOWERCASE", lower_case_fn);
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
        self.parse_var_pair(sigma)?;
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
    for var in self.vars.values() {
      if let Some(value) = self.registry.get(var.name) {
        let mut current_data = (*value).to_owned();
        for function in &var.functions {
          let f = &self.functions[&function.0]; // we are sure it will be there.
          current_data = (f.call)(current_data);
        }
        self
          .input
          .replace_range(var.location.0..var.location.1, &current_data);
      } else if var.nullable {
        self.input.replace_range(var.location.0..var.location.1, "");
      } else {
        let mut extra_help = String::new();
        if let Some(matches) = parser::did_you_mean(var.name, self.vars.keys())
        {
          extra_help = format!("Did you mean: `{}` ?", matches);
        } else {
          extra_help = "consider adding a register for it".to_owned();
        }
        return Err(PestError::new_from_span(
          ErrorVariant::CustomError {
            message: format!("undefined variable: {} {}", var.name, extra_help),
          },
          var.name_span.clone().unwrap(),
        ));
      }
    }
    Ok(self.input)
  }

  // TODO: Refactor this function
  // TODO: Validate Datatypes
  // TODO: Parse functions in seprate function
  fn parse_var_pair(&mut self, pair: Pair<'s, Rule>) -> SigmaResult<()> {
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
            Some((self.parse_data_type(&data_type)?, data_type.as_span()));
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
    let mut variable = self.parse_function(inner_rules, variable)?;
    variable.location = (open_pairs.as_span().start(), variable.location.1);
    self.vars.insert(variable.name, variable);
    Ok(())
  }

  fn parse_data_type<'b>(
    &self,
    pair: &Pair<Rule>,
  ) -> SigmaResult<'b, DataType> {
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
        let p_vals = [
          "u8", "u16", "u32", "u64", "i8", "i16", "i32", "i64", "f32", "f64",
          "str", "bool",
        ];
        let mut extra_help = String::new();
        if let Some(matches) = parser::did_you_mean(val, p_vals.iter()) {
          extra_help = format!("Did you mean: `{}` ?", matches);
        }
        return Err(PestError::new_from_span(
          ErrorVariant::CustomError {
            message: format!("unknown Data type: `{}` {}", val, extra_help),
          },
          pair.as_span(),
        ));
      },
    };
    Ok(result)
  }

  fn parse_function<'f>(
    &self,
    pairs: Pairs<'f, Rule>,
    mut var: Variable<'f>,
  ) -> SigmaResult<'f, Variable<'f>> {
    for pair in pairs {
      let rule = pair.as_rule();
      match rule {
        Rule::function => {
          if var.data_type.is_none() || !var.typed {
            return Err(PestError::new_from_span(
              ErrorVariant::ParsingError {
                positives: vec![Rule::data_type],
                negatives: vec![],
              },
              var.name_span.unwrap(),
            ));
          }
          let mut function = pair.into_inner();
          let _sep = function.next().unwrap();
          let function_name = function.next().unwrap();
          if !self.functions.contains_key(function_name.as_str()) {
            let mut extra_help = String::new();
            if let Some(matches) =
              parser::did_you_mean(function_name.as_str(), self.functions.keys())
            {
              extra_help = format!("Did you mean: `{}` ?", matches);
            }
            return Err(PestError::new_from_span(
              ErrorVariant::CustomError {
                message: format!(
                  "undefined function: {} {}",
                  function_name.as_str(),
                  extra_help
                ),
              },
              function_name.as_span(),
            ));
          }
          var
            .functions
            .push((function_name.as_str(), function_name.as_span()));
        },
        Rule::pair_close => {
          var.location = (0, pair.as_span().end());
          break;
        },
        _ => {},
      };
    }
    Ok(var)
  }
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
