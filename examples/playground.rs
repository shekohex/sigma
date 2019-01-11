use sigma::Sigma;
fn main() -> Result<(), ()> {
  let result = Sigma::new("my username is {{ username: u32 | UPPERCASE }} WOW!")
      .bind("username", "shekohex")
      .parse()
      .map_err(|e| eprintln!("Parse Error:\n{}", e))? // error handling..
      .compile()
      .map_err(|e| eprintln!("Compile Error:\n{}", e))?; // error handling..
  assert_eq!("my username is SHEKOHEX WOW!", result);
  Ok(())
}
