#[cfg(test)]
mod main_hla_rs_calculator_test {
    use crate::haplotype_risk_calculator;
    use crate::program_args_parsers::input_structures::Alleles;

    #[test]
    fn calculate_risk_score_test() {
        let alleles = Alleles{
            h1_dqa1: String::from(""),
            h1_dqb1: String::from(""),
            h2_dqa1: String::from(""),
            h2_dqb1: String::from(""),
            non_hla_alleles: Vec::new(),
            other_hla_alleles: Vec::new(),
        };
        let result = haplotype_risk_calculator::calculate_risk_score(&alleles);
        assert_eq!(result, 0.1);
    }
}