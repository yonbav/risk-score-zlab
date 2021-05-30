use crate::program_args_parsers::input_structures::RsAlleles;
use std::collections::HashMap;
use std::array::IntoIter;
use std::iter::FromIterator;



pub fn calculate_risk_score(alleles: &Vec<RsAlleles>) -> f32 {
    return alleles.into_iter().map(|allele| get_allele_beta(allele)).sum::<f32>();
}

fn get_allele_beta(allele: &RsAlleles) -> f32 {
    let other_hla_scores : HashMap<RsAlleles, f32> = HashMap::<_, _>::from_iter(
        IntoIter::new(
            [
                (RsAlleles { locus: String::from("rs540653847"), variants: vec![String::from("GC")] }, 1.78)
            ]
        )
    );
    return if other_hla_scores.contains_key(allele) { other_hla_scores[allele] } else  { 0.0 }
}
