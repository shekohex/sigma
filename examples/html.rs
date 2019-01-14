use sigma::Sigma;
use std::{collections::HashMap, fs::File, io::Read};
fn main() -> Result<(), ()> {
  let mut html_file =
    File::open("./examples/data/hello.html").expect("hello.html not found !");
  let mut input = String::new();
  html_file
    .read_to_string(&mut input)
    .expect("unable to read the file");
  let mut data = HashMap::new();
  data.insert("name", "someone");
  data.insert("id", "100");
  data.insert("title", "Home Page");
  let result = Sigma::new(&input)
    .bind_map(data)
    .parse()
    .map_err(|e| eprintln!("Parse Error:\n{}", e))? // error handling..
    .compile()
    .map_err(|e| eprintln!("Compile Error:\n{}", e))?; // error handling..

  println!("{}", result);
  Ok(())
}
