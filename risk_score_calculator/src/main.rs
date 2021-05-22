#[macro_use] extern crate serde_derive;

mod other_hla_risk_score;
mod main_hla_risk_score;
mod non_hla_risk_score;
mod program_args_parsers;

use other_hla_risk_score::other_hla_rs_calculator;
use main_hla_risk_score::main_hla_rs_calculator;
use non_hla_risk_score::non_hla_rs_calculator;
use program_args_parsers::file_path_parser;
use std::env;

fn main() {
    let args = env::args().collect();
    let alleles = file_path_parser::get_alleles_from_args(args);
    let main_hla_result = main_hla_rs_calculator::calculate_risk_score(1, 2);
    let other_hla_result = other_hla_rs_calculator::calculate_risk_score(3,4);
    let non_hla_result = non_hla_rs_calculator::calculate_risk_score(5,6);
    print_result_message(main_hla_result, other_hla_result, non_hla_result);
}

fn print_result_message(main_hla_result: Result<i32, String>, other_hla_result: Result<i32, String>, non_hla_result: Result<i32, String>) {
    let message = format!(r#"
    genetic risk calculator
    ========================
    main_hla: {},
    other_hla_result: {}, 
    non_hla_result:: {}. "# ,
    main_hla_result.unwrap(), other_hla_result.unwrap(), non_hla_result.unwrap());
    println!("{}", message);
}
