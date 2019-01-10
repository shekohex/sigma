use sigma::Sigma;
fn main() -> Result<(), ()> {
  let result = Sigma::new()
    .parse("my username is {{ username: str | UPPERCASE }} WOW!")
    .map_err(|e| println!("{}", e))? // error handling..
    .register("username", "shekohex")
    .compile()
    .map_err(|e| println!("{}", e))?; // error handling..

  assert_eq!("my username is SHEKOHEX WOW!", result);
  Ok(())
}
