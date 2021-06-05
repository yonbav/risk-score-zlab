#[cfg(test)]
mod rs_risk_calculator_test {
    use crate::rs_risk_calculator;
    use crate::program_args_parsers::input_structures::RsAlleles;
    use std::array::IntoIter;
    use std::iter::FromIterator;
    use std::collections::HashMap;

    #[test]
    fn calculate_risk_score_test_when_locus_fit_in_one_allele() {
        let alleles = vec![
            RsAlleles { 
                locus: String::from("rs540653847"), 
                variants: vec![String::from("GC")]
            }
        ];
        let scores = get_rs_scores();
        let result = rs_risk_calculator::calculate_risk_score(&alleles, scores);
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
        let scores = get_rs_scores();
        let result = rs_risk_calculator::calculate_risk_score(&alleles, scores);
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
        let scores = get_rs_scores();
        let result = rs_risk_calculator::calculate_risk_score(&alleles, scores);
        let expected = 3.56;
        assert!((result - expected).abs() < 0.0001)
    }

    fn get_rs_scores() -> HashMap<RsAlleles, f32> {
        return HashMap::<_, _>::from_iter(
            IntoIter::new(
                [
                    (RsAlleles { locus: String::from("rs540653847"), variants: vec![String::from("GC")] }, 1.78),
                    (RsAlleles { locus: String::from("rs9271346"), variants: vec![String::from("T")] }, 1.69)
                ]
            )
        );
    }
}