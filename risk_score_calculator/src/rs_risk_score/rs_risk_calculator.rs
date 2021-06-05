use crate::program_args_parsers::input_structures::RsAlleles;
use std::collections::HashMap;


pub fn calculate_risk_score(alleles: &Vec<RsAlleles>, scores: HashMap<RsAlleles, f32>) -> f32 {
    let alleles_per_haplotype = get_alleles_per_haplotype(alleles);
    return alleles_per_haplotype.into_iter().map(|allele| get_allele_beta(&allele, &scores)).sum::<f32>();
}

fn get_alleles_per_haplotype(alleles: &Vec<RsAlleles>) -> Vec<RsAlleles> {
    return alleles.iter().flat_map(|allele| get_allele_per_haplotype(allele)).collect();
}

fn get_allele_per_haplotype(allele: &RsAlleles) -> Vec<RsAlleles> {
    return allele.variants.iter().map(|current_variant| RsAlleles{ locus: allele.locus.clone(), variants: vec![String::from(current_variant)] }).collect()
}

fn get_allele_beta(allele: &RsAlleles, scores: &HashMap<RsAlleles, f32>) -> f32 {
    return if scores.contains_key(allele) { scores[allele] } else { 0.0 }
}
