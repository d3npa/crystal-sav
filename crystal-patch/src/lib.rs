use std::env;
use sav_tools::errors::ParserError;

pub struct Arguments {
    pub sav_file: String
}

impl Arguments {
    pub fn parse() -> Result<Arguments, ParserError> {
        let mut args: Vec<String> = env::args().collect();

        if args.len() != 2 {
            return Err(ParserError::InvalidArguments);
        }

        let sav_file = args.remove(1);
        Ok(Arguments { sav_file })
    }
}

