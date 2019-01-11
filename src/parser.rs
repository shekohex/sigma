// this code is seprated in that file
// to allow missing docs lint.
#![allow(missing_docs)]
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "sigma.pest"]
pub(crate) struct SigmaParser;

/// Produces a string from a given list of possible values which is similar to
/// the passed in value `v` with a certain confidence.
/// Thus in a list of possible values like ["foo", "bar"], the value "fop" will
/// yield `Some("foo")`, whereas "blark" would yield `None`.
/// see: https://github.com/clap-rs/clap/blob/master/src/suggestions.rs
pub(crate) fn did_you_mean<'a, T: ?Sized, I>(
  v: &str,
  possible_values: I,
) -> Option<&'a str>
where
  T: AsRef<str> + 'a,
  I: IntoIterator<Item = &'a T>,
{
  let mut candidate: Option<(f64, &str)> = None;
  for pv in possible_values {
    let confidence = strsim::jaro_winkler(v, pv.as_ref());
    if confidence > 0.8
      && (candidate.is_none() || (candidate.as_ref().unwrap().0 < confidence))
    {
      candidate = Some((confidence, pv.as_ref()));
    }
  }
  match candidate {
    None => None,
    Some((_, candidate)) => Some(candidate),
  }
}

#[cfg(test)]
mod test_did_you_mean {
  use super::*;

  #[test]
  fn possible_values_match() {
    let p_vals = ["test", "possible", "values"];
    assert_eq!(did_you_mean("tst", p_vals.iter()), Some("test"));
  }
}
