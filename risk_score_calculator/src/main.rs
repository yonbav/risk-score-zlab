mod other_hla_risk_score;
mod main_hla_risk_score;
mod non_hla_risk_score;

use other_hla_risk_score::other_hla_rs_calculator;
use main_hla_risk_score::main_hla_rs_calculator;
use non_hla_risk_score::non_hla_rs_calculator;

fn main() {
    let main_hla_result = main_hla_rs_calculator::calculate_risk_score(1, 2);
    let other_hla_result = other_hla_rs_calculator::calculate_risk_score(3,4);
    let non_hla_result = non_hla_rs_calculator::calculate_risk_score(5,6);
    let message = format!("genetic risk calculator\n================================\nmain_hla: {}, other_hla_result: {}, non_hla_result:: {}. ", main_hla_result.unwrap(), other_hla_result.unwrap(), non_hla_result.unwrap());
    println!("{}", message);
}
