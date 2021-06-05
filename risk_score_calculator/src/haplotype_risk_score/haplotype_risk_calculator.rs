use crate::program_args_parsers::input_structures::Alleles;
use std::collections::HashMap;

pub fn calculate_risk_score(alleles: &Alleles, scores: HashMap<String, f32>) -> f32 {
    let first_haplotype_score = get_haplotype_score(&format!("{}-{}", alleles.h1_dqa1, alleles.h1_dqb1), &scores);
    let second_haplotype_score = get_haplotype_score(&format!("{}-{}", alleles.h2_dqa1, alleles.h2_dqb1), &scores);
    return first_haplotype_score + second_haplotype_score
}

fn get_haplotype_score(haplotype: &str, scores: &HashMap<String, f32>) -> f32 {
    return if scores.contains_key(haplotype) { scores[haplotype] } else { 0.0 };
}
