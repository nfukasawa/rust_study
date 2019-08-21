mod error;
mod macros;
mod parser;
mod reader;
mod value;

pub use error::Err;
pub use value::Value;

use parser::Parser;
use reader::BytesReader;

use std::str::FromStr;
impl FromStr for Value {
    type Err = error::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Parser::new(BytesReader::new(s.as_bytes())).parse()
    }
}
