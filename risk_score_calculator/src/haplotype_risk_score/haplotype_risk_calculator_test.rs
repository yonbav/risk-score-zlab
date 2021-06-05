#[cfg(test)]
mod main_hla_rs_calculator_test {
    use crate::haplotype_risk_calculator;
    use crate::program_args_parsers::input_structures::Alleles;
    use std::collections::HashMap;
    use std::array::IntoIter;
    use std::iter::FromIterator;

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
        let scores = get_hla_haplotype_scores();
        let result = haplotype_risk_calculator::calculate_risk_score(&alleles, scores);
        assert_eq!(result, 0.1);
    }

    fn get_hla_haplotype_scores() -> HashMap<String, f32> {
        return HashMap::<_, _>::from_iter(
            IntoIter::new(
                [
                    (String::from("03:0X-03:02") , 2.08),
                    (String::from("01:03-06:01") , -2.41)
                ]
            )
        ); 
    }
}