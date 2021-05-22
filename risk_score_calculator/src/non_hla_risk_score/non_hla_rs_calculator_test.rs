#[cfg(test)]
mod non_hla_rs_calculator_test {
    use crate::non_hla_rs_calculator;

    #[test]
    fn calculate_risk_score_test() {
        let result = non_hla_rs_calculator::calculate_risk_score(1, 2);
        assert_eq!(result, Ok(3));
    }
}