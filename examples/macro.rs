use sigma::sigma;
use std::io::{self, BufRead};

fn main() {
  println!("What is your name ?:");
  let mut name = String::new();
  let stdin = io::stdin();
  stdin
    .lock()
    .read_line(&mut name)
    .expect("Could not read line");
  let waving = "👋";
  let s = sigma!("Hello {{ name: str |> TRIM |> UPPERCASE }} ! I'm Sigma {{ waving }}", name, waving);

  match s {
    Ok(r) => println!("{}", r),
    Err(e) => eprintln!("Error\n{}", e),
  }
}
