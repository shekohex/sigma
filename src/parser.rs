// this code is seprated in that file
// to allow missing docs lint.
#![allow(missing_docs)]
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "sigma.pest"]
pub(crate) struct SigmaParser;
