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
//! ```ignore
//! --> 1:49
//!   |
//! 1 | my username is {{ username: str |> UPPERCASE |> NO_FUN }} WOW!
//!   |                                                 ^----^
//!   |
//!   = undefined function: NO_FUN
//! ```
//!
//! what if you forgot to bind for some variable in your template ?
//!
//! ```ignore
//!  --> 1:19
//!   |
//! 1 | my username is {{ username: str |> UPPERCASE |> NO_FUN }} WOW!
//!   |                   ^------^
//!   |
//!   = undefined variable: username consider adding a bind for it
//! ```
//! do you need extra help ? we got your back ;)
//!
//! ```ignore
//! --> 1:35
//!   |
//! 1 | my username is {{ username: u32 | UPPERCAS }} WOW!
//!   |                                   ^------^
//!   |
//!   = undefined function: UPPERCAS did you mean: UPPERCASE ?
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
//! ```ignore
//! use sigma::Sigma;
//!
//! let result = Sigma::new("Hello {{ username }}") // using {{ ... }} for the template.
//!  .bind("username", "someone") // bind the vars with values
//!  .parse() // you must parse your template first
//!  .map_err(|e| eprintln!("{}", e))? // for pretty printing the error..
//!  .compile();
//! assert_eq!("Hello someone", result);
//! ```
//! * with optinal variables
//! ```ignore
//! use sigma::Sigma;
//!   
//! let result = Sigma::new("Hello {{ username? }}") // using `?` to tell the parser it maybe `null`.
//!  .parse()
//!  .map_err(|e| eprintln!("{}", e))? // for pretty printing the error..
//!  .compile();
//! assert_eq!("Hello ", result);
//! ```
//! * what about types ?
//!
//! ```ignore
//! use sigma::Sigma;
//!   
//! let result = Sigma::new("Hello {{ username: str }}") // u8, u32 ? a bool ?.
//!  .bind("username", "someone")
//!  .parse()
//!  .map_err(|e| eprintln!("{}", e))? // for pretty printing the error..
//!  .compile();
//! assert_eq!("Hello someone", result);
//! ```
//! * how about functions ?
//! ```ignore
//! use sigma::Sigma;
//!   
//! let result = Sigma::new("Hello {{ username: str | UPPERCASE }}") // functions uses the `|` operator or if you love `|>` you can use it too.
//!  .bind("username", "someone")
//!  .parse()
//!  .map_err(|e| eprintln!("{}", e))? // for pretty printing the error..
//!  .compile();
//! assert_eq!("Hello SOMEONE", result);
//! ```
mod parser;

use crate::parser::{Rule, SigmaParser};
use pest::{
  error::{Error as PestError, ErrorVariant},
  iterators::{Pair, Pairs},
  Parser, Span,
};
use regex::{NoExpand, Regex};
use std::collections::HashMap;

type SigmaResult<'a, T> = Result<T, PestError<Rule>>;

/// Primitive Data Types
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DataType {
  /// The 8-bit unsigned integer type.
  ///
  /// ex: `{{ data: u8 }}`
  U8,
  /// The 8-bit signed integer type.
  ///
  /// ex: `{{ data: i8 }}`
  I8,
  /// The 16-bit unsigned integer type.
  ///
  /// ex: `{{ data: u16 }}`
  U16,
  /// The 16-bit signed integer type.
  ///
  /// ex: `{{ data: i16 }}`
  I16,
  /// The 32-bit unsigned integer type.
  ///
  /// ex: `{{ data: u32 }}`
  U32,
  /// The 32-bit signed integer type.
  ///
  /// ex: `{{ data: i32 }}`
  I32,
  /// The 64-bit unsigned integer type.
  ///
  /// ex: `{{ data: u64 }}`
  U64,
  /// The 64-bit signed integer type.
  ///
  /// ex: `{{ data: i64 }}`
  I64,
  /// The 32-bit floating point type.
  ///
  /// ex: `{{ data: f32 }}`
  F32,
  /// The 64-bit floating point type.
  ///
  /// ex: `{{ data: f64 }}`
  F64,
  /// The boolean type.
  ///
  /// ex: `{{ data: bool }}`
  Bool,
  /// String.
  ///
  /// ex: `{{ data: str }}`
  Str,
}

#[doc(hidden)]
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Variable<'a> {
  pub name: &'a str,
  pub nullable: bool,
  pub typed: bool,
  pub data_type: Option<(DataType, Span<'a>)>,
  pub location: (usize, usize),
  pub functions: Vec<(&'a str, Span<'a>)>,
  pub name_span: Option<Span<'a>>,
  pub pair_str: &'a str,
}

#[doc(hidden)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Function {
  pub name: String,
  pub call: fn(String) -> String,
}

/// Sigma
///
/// TODO: Add examples here
#[derive(Debug, PartialEq, Eq)]
pub struct Sigma<'s> {
  vars: HashMap<&'s str, Variable<'s>>,
  registry: HashMap<String, String>,
  input: &'s str,
  is_parsed: bool,
  functions: HashMap<&'s str, Function>,
}

impl<'s> Sigma<'s> {
  /// Create new Sigma with some template
  pub fn new(input: &'s str) -> Self {
    let sigma = Self {
      input,
      vars: HashMap::new(),
      functions: HashMap::new(),
      is_parsed: false,
      registry: HashMap::new(),
    };

    let sigma = sigma.register_fn("UPPERCASE", |input| input.to_uppercase());
    sigma.register_fn("LOWERRCASE", |input| input.to_lowercase())
  }

  /// bind some key in the template for some value
  pub fn bind(mut self, key: &'s str, value: &'s str) -> Self {
    self.registry.insert(key.to_owned(), value.to_owned());
    self
  }

  /// bind one or more keys with values in one run.
  pub fn bind_map(mut self, map: HashMap<String, String>) -> Self {
    self.registry.extend(map);
    self
  }

  /// remove all the previous binded keys, and use that one
  pub fn override_bind(mut self, map: HashMap<String, String>) -> Self {
    self.registry = map;
    self
  }

  /// register a helper function
  pub fn register_fn(
    mut self,
    func_name: &'s str,
    func: fn(String) -> String,
  ) -> Self {
    self.functions.insert(
      func_name,
      Function {
        name: func_name.to_uppercase(),
        call: func,
      },
    );
    self
  }

  /// Parse the template before compiling it to ensure no runtime erros.
  pub fn parse(mut self) -> SigmaResult<'s, Self> {
    for sigma in SigmaParser::parse(Rule::sigma, &self.input)? {
      if sigma.as_rule() == Rule::var_pair {
        self.parse_var_pair(sigma)?;
      }
    }
    self.is_parsed = true;
    Ok(self)
  }

  /// Compile the template with the binded values
  ///
  /// ## Panics
  /// this will panic if the current template
  /// not parsed yet.
  pub fn compile(self) -> SigmaResult<'s, String> {
    assert!(self.is_parsed, "The template must be parsed first");
    let mut output = self.input.to_owned(); // copy the input
    for var in self.vars.values() {
      let var_regex = Regex::new(&regex::escape(var.pair_str)).unwrap();
      if let Some(value) = self.registry.get(var.name) {
        let mut current_data = (*value).to_owned();
        for function in &var.functions {
          let f = &self.functions[&function.0]; // we are sure it will be there.
          current_data = (f.call)(current_data);
        }
        self.validate_data_type(&var, &current_data)?;
        output = var_regex
          .replace_all(&output, NoExpand(&current_data))
          .to_string();
      } else if var.nullable {
        // it must be nullable then
        output = var_regex.replace_all(&output, NoExpand("")).to_string();
      }
    }
    Ok(output)
  }

  // TODO: Refactor this function
  fn parse_var_pair(&mut self, pair: Pair<'s, Rule>) -> SigmaResult<()> {
    let mut variable = Variable::default();
    variable.pair_str = pair.as_str();
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
    // check if we have a back value for this variable ?
    if !self.registry.contains_key(variable.name) && !variable.nullable {
      let extra_help;
      if let Some(matches) =
        parser::did_you_mean(variable.name, self.registry.keys())
      {
        extra_help = format!("did you mean: `{}` ?", matches);
      } else {
        extra_help = "consider adding a bind for it".to_owned();
      }
      return Err(PestError::new_from_span(
        ErrorVariant::CustomError {
          message: format!(
            "undefined variable: `{}` {}",
            variable.name, extra_help
          ),
        },
        variable.name_span.clone().unwrap(),
      ));
    }
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
          extra_help = format!("did you mean: `{}` ?", matches);
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
            if let Some(matches) = parser::did_you_mean(
              function_name.as_str(),
              self.functions.keys(),
            ) {
              extra_help = format!("did you mean: `{}` ?", matches);
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

  fn validate_data_type(
    &self,
    var: &Variable,
    data: &str,
  ) -> SigmaResult<'s, ()> {
    if let Some(data_type) = &var.data_type {
      use self::DataType::*;
      let data_type_error = {
        let extra = if data.len() > 15 { "..." } else { "" };
        PestError::<Rule>::new_from_span(
          ErrorVariant::CustomError {
            message: format!(
              "cannot parse input `{}{}` into `{:?}` for var `{}` !",
              data.chars().take(15).collect::<String>(),
              extra,
              data_type.0, var.name
            ),
          },
          data_type.1.clone(),
        )
      };
      match data_type.0 {
        U8 => {
          data.parse::<u8>().map_err(|_| data_type_error)?;
        },
        I8 => {
          data.parse::<i8>().map_err(|_| data_type_error)?;
        },
        U16 => {
          data.parse::<u16>().map_err(|_| data_type_error)?;
        },
        I16 => {
          data.parse::<i16>().map_err(|_| data_type_error)?;
        },
        U32 => {
          data.parse::<u32>().map_err(|_| data_type_error)?;
        },
        I32 => {
          data.parse::<i32>().map_err(|_| data_type_error)?;
        },
        U64 => {
          data.parse::<u64>().map_err(|_| data_type_error)?;
        },
        I64 => {
          data.parse::<i64>().map_err(|_| data_type_error)?;
        },
        F32 => {
          data.parse::<f32>().map_err(|_| data_type_error)?;
        },
        F64 => {
          data.parse::<f64>().map_err(|_| data_type_error)?;
        },
        Bool => {
          data.parse::<bool>().map_err(|_| data_type_error)?;
        },
        _ => {
          // it must be a string then
        },
      };
      return Ok(());
    }
    Ok(())
  }
}

// TODO: Add more tests here.
#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  #[should_panic]
  fn missing_data_type() {
    let input = "{{ username: }}";
    let _ = Sigma::new(input)
      .bind("username", "test")
      .parse()
      .unwrap()
      .compile();
  }

  #[test]
  #[should_panic]
  fn unknown_data_type() {
    let input = "{{ username: unknown }}";
    let output = Sigma::new(input)
      .bind("username", "test")
      .parse()
      .unwrap()
      .compile();
    println!("{:?}", output);
  }
}
