#[cfg(test)]
mod other_hla_rs_calculator_test {
    use crate::other_hla_rs_calculator;

    #[test]
    fn calculate_risk_score_test() {
        let result = other_hla_rs_calculator::calculate_risk_score(1, 2);
        assert_eq!(result, Ok(3));
    }
}