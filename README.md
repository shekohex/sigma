# Sigma σ is a Simple, Safe and Fast Template language

[![Latest version](https://img.shields.io/crates/v/sigma.svg)](https://crates.io/crates/sigma)
[![Documentation](https://docs.rs/sigma/badge.svg)](https://docs.rs/sigma)
![License](https://img.shields.io/crates/l/sigma.svg)

Hi {{ name }} i'm sigma :wave: !

##### Simple:
sigma is a very simple template language, it only tries to solve only one
problem. it also extendable, but with simple idea too (_Pure Functions_).
##### Safe:
sigma is also typed, that means that it has the idea of built-in validators
for your input. and for those how wanna play, it also could be untyped.
also it has a good error checking at parse time of your template.
the only error that could happen in runtime is that the input data fails to be parsed to your data types
in your templates.

Here is some error examples:
```
--> 1:49
  |
1 | my username is {{ username: str |> UPPERCASE |> NO_FUN }} WOW!
  |                                                 ^----^
  |
  = undefined function: NO_FUN
```
what if you forgot to bind for some variable in your template ?
```
 --> 1:19
  |
1 | my username is {{ username: str |> UPPERCASE |> NO_FUN }} WOW!
  |                   ^------^
  |
  = undefined variable: username consider adding a bind for it
```
do you need extra help ? we got your back ;)
```
--> 1:35
  |
1 | my username is {{ username: u32 | UPPERCAS }} WOW!
  |                                   ^------^
  |
  = undefined function: UPPERCAS did you mean: UPPERCASE ?
```

##### Fast:
sigma uses [`pest`](https://pest.rs/), The Elegant Parser under the hood to write it's grammar.
that means it will be exteramly fast in parsing your templete, also it uses regex crate to replace your
data in the template.

Here is some benchmacrk on my old pc running intel quad core q9650 processor and an Hard Disk Drive (HDD)
```
small_data_1kb_parse: 55.300 us
small_data_10kb_parse: 516.29 us
small_data_50kb_parse: 2.5659 ms
small_data_500kb_parse: 25.467 ms
small_data_1mb_parse: 48.668 ms
---
small_data_1kb_compile: 123.39 us
small_data_10kb_compile: 120.90 us
small_data_50kb_compile: 255.70 us
small_data_500kb_compile: 1.4492 ms
small_data_1mb_compile: 1.1464 ms
```

### Examples
here is a simple examples of how it works

* Simple:
```rust
use sigma::Sigma;
let result = Sigma::new("Hello {{ username }}") // using {{ ... }} for the template.
 .bind("username", "someone") // bind the vars with values
 .parse() // you must parse your template first
 .map_err(|e| eprintln!("{}", e))? // for pretty printing the error..
 .compile()?;
assert_eq!("Hello someone", result);
```
* with optinal variables
```rust
use sigma::Sigma;
  
let result = Sigma::new("Hello {{ username? }}") // using `?` to tell the parser it maybe `null`.
 .parse()
 .map_err(|e| eprintln!("{}", e))? // for pretty printing the error..
 .compile()?;
assert_eq!("Hello ", result);
```
* what about types ?
```rust
use sigma::Sigma;
  
let result = Sigma::new("Hello {{ username: str }}") // u8, u32 ? a bool ?.
 .bind("username", "someone")
 .parse()
 .map_err(|e| eprintln!("{}", e))? // for pretty printing the error..
 .compile()?;
assert_eq!("Hello someone", result);
```
* how about functions ?
```rust
use sigma::Sigma;
  
let result = Sigma::new("Hello {{ username: str | UPPERCASE }}") // functions uses the `|` operator or if you love `|>` you can use it too.
 .bind("username", "someone")
 .parse()
 .map_err(|e| eprintln!("{}", e))? // for pretty printing the error..
 .compile()?;
assert_eq!("Hello SOMEONE", result);
```
* love macros ?
```rust
use sigma::sigma;
let username = "someone";
let result = sigma!("Hello {{ username }}", username); // the macro return the result so you can check for compile erros.
assert_eq!("Hello someone", result.unwrap());
```

## Contributing

You are welcome to contribute to this project, just open a PR.

## Authors

* **Shady Khalifa** - _Initial work_

See also the list of [contributors](https://github.com/shekohex/sigma/contributors) who participated in this project.

## License

This project is licensed under the MIT License - see the [LICENSE.md](LICENSE.md) file for details.
