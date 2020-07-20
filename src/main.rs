mod parser;
use nom::error::VerboseError;
pub fn main() {
    let _ = parser::parse_mod::<VerboseError<&str>>("").unwrap();
}
