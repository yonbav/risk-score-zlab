#[cfg(test)]
mod non_hla_rs_calculator_test {
    use crate::non_hla_rs_calculator;
    use crate::program_args_parsers::input_structures::RsAlleles;

    #[test]
    fn calculate_risk_score_test_when_locus_fit_in_one_allele() {
        let alleles = vec![
            RsAlleles { 
                locus: String::from("rs540653847"), 
                variants: vec![String::from("GC")]
            }
        ];
        let result = non_hla_rs_calculator::calculate_risk_score(&alleles);
        let expected = 1.78;
        assert!((result - expected).abs() < 0.0001)
    }

    #[test]
    fn calculate_risk_score_test_when_no_locus_fit() {
        let alleles = vec![
            RsAlleles { 
                locus: String::from("rs540653847"), 
                variants: vec![String::from("G")]
            },
            RsAlleles { 
                locus: String::from("rs9271346"), 
                variants: vec![String::from("A"), String::from("G")]
            }
        ];
        let result = non_hla_rs_calculator::calculate_risk_score(&alleles);
        let expected = 0.0;
        assert!((result - expected).abs() < 0.0001)
    }

    #[test]
    fn calculate_risk_score_test_when_locus_fit_in_both_alleles() {
        let alleles = vec![
            RsAlleles { 
                locus: String::from("rs540653847"), 
                variants: vec![String::from("GC"), String::from("GC")]
            }
        ];
        let result = non_hla_rs_calculator::calculate_risk_score(&alleles);
        let expected = 3.56;
        assert!((result - expected).abs() < 0.0001)
    }
}