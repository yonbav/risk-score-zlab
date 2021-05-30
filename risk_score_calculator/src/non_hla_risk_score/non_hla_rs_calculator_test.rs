#[cfg(test)]
mod non_hla_rs_calculator_test {
    use crate::non_hla_rs_calculator;
    use crate::program_args_parsers::input_structures::RsAlleles;

    #[test]
    fn calculate_risk_score_test() {
        let alleles = vec![
            RsAlleles { 
                locus: String::from("rs540653847"), 
                variants: vec![String::from("GC")]
            },
            RsAlleles { 
                locus: String::from("rs231"), 
                variants: vec![String::from("A"), String::from("G")]
            }
        ];
        let result = non_hla_rs_calculator::calculate_risk_score(&alleles);
        assert!((result - 1.78).abs() < 0.0001)
    }
}