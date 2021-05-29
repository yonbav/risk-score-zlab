extern crate serde;
extern crate serde_json;

use crate::program_args_parsers::input_structures::Alleles;
use crate::program_args_parsers::errors::ArgsError;
use std::fs;

pub fn get_alleles_from_args(args: Vec<String>) -> Alleles {
    let file_path = match get_file_path(args) {
       Err(err) => panic!("{}", err),
       Ok(file_path) => file_path 
    };
    return get_alleles_from_file(file_path);
}


fn get_file_path(args: Vec<String>) -> Result<String, ArgsError> {
    if args.len() <= 1 {
        return Err(ArgsError::new(&format!("received argument are invalid.")));
    }
    return Ok(args[1].clone());
}

fn get_alleles_from_file(file_path: String) -> Alleles {
    let alleles_json = match fs::read_to_string(file_path) {
        Err(err) => panic!("{}", err),
        Ok(alleles_json) => alleles_json
    };
    let alleles: Alleles = serde_json::from_str(&alleles_json).unwrap();
    return alleles;
}
