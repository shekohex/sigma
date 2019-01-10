# Sigma σ is a Simple, Safe and Fast Template language

Currently Under Development..

But here is some of it's gools.

---

# Sigma: Simple, Safe and Fast Template language.

##### Simple:

sigma is a very simple template language, it only tries to solve only one
problem. it also extendable, but with simple idea too (_Pure Functions_).

##### Safe:

sigma is also typed, that means that it has the idea of built-in validators
for your input. and for those how wanna play, it also could be untyped.
also it has a good error checking at parse time of your template:
Here is some error examples:

```
--> 1:49
  |
1 | my username is {{ username: str |> UPPERCASE |> NO_FUN }} WOW!
  |                                                 ^----^
  |
  = undefined function: NO_FUN
```

what if you fogot to register for some variable in your template ?

```
 --> 1:19
  |
1 | my username is {{ username: str |> UPPERCASE |> NO_FUN }} WOW!
  |                   ^------^
  |
  = undefined variable: username consider adding a register for it
```

##### Fast:

sigma uses [`pest`](https://pest.rs/), The Elegant Parser under the hood to write it's grammar.
that means it will be exteramly fast in parsing your templete.

// TODO: Add some benchmarks here

### Examples

here is a simple examples of how it works

- Simple:

```rust
use sigma::Sigma;

let result = Sigma::new()
 .parse("Hello {{ username }}") // using {{ ... }} for the template.
 .map_err(|e| println!("{}", e))? // for pretty printing the error..
 .register("username", "someone")
 .compile()
 .map_err(|e| println!("{}", e))?;
assert_eq!("Hello someone", result);
```

- with optinal variables

```rust
use sigma::Sigma;

let result = Sigma::new()
 .parse("Hello {{ username? }}") // using `?` to tell the parser it maybe `null`.
 .map_err(|e| println!("{}", e))? // for pretty printing the error..
 .compile()
 .map_err(|e| println!("{}", e))?;
assert_eq!("Hello ", result);
```

- what about types ?

```rust
use sigma::Sigma;

let result = Sigma::new()
 .parse("Hello {{ username: str }}") // u8, u32 ? bool ! use all ?.
 .map_err(|e| println!("{}", e))? // for pretty printing the error..
 .register("username", "someone")
 .compile()
 .map_err(|e| println!("{}", e))?;
assert_eq!("Hello someone", result);
```

- how about functions ?

```rust
use sigma::Sigma;

let result = Sigma::new()
 .parse("Hello {{ username: str | UPPERCASE }}") // functions uses the `|` operator or if you love `|>`.
 .map_err(|e| println!("{}", e))? // for pretty printing the error..
 .register("username", "someone")
 .compile()
 .map_err(|e| println!("{}", e))?;
assert_eq!("Hello SOMEONE", result);
```

## Contributing

You are welcome to contribute to this project, just open a PR.

## Authors

* **Shady Khalifa** - _Initial work_

See also the list of [contributors](contributors) who participated in this project.

## License

This project is licensed under the MIT License - see the [LICENSE.md](LICENSE.md) file for details.
