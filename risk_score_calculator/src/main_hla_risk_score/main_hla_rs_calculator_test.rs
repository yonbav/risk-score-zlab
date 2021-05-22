#[cfg(test)]
mod main_hla_rs_calculator_test {
    use crate::main_hla_rs_calculator;

    #[test]
    fn calculate_risk_score_test() {
        let result = main_hla_rs_calculator::calculate_risk_score(1, 2);
        assert_eq!(result, Ok(3));
    }
}