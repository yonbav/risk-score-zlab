use crate::program_args_parsers::input_structures::RsAlleles;
use std::collections::HashMap;
use std::array::IntoIter;
use std::iter::FromIterator;


pub fn calculate_risk_score(alleles: &Vec<RsAlleles>) -> f32 {
    return alleles.into_iter().map(|allele| get_allele_beta(allele)).sum::<f32>();
}

fn get_allele_beta(allele: &RsAlleles) -> f32 {
    let non_hla_score : HashMap<RsAlleles, f32> = HashMap::<_, _>::from_iter(
        IntoIter::new(
            [
                (RsAlleles { locus: String::from("rs540653847"), variants: vec![String::from("GC")] }, 1.78),
                (RsAlleles { locus: String::from("rs9271346"), variants: vec![String::from("T")] }, 1.69),
                (RsAlleles { locus: String::from("rs116522341"), variants: vec![String::from("C")] }, 1.24),
                (RsAlleles { locus: String::from("rs1281934"), variants: vec![String::from("G")] }, 0.9),
                (RsAlleles { locus: String::from("rs2567287"), variants: vec![String::from("A")] }, 0.84),
                (RsAlleles { locus: String::from("rs75658393"), variants: vec![String::from("T")] }, 0.81),
                (RsAlleles { locus: String::from("rs72848653"), variants: vec![String::from("T")] }, 0.78),
                (RsAlleles { locus: String::from("rs144530872"), variants: vec![String::from("A")] }, 0.74),
                (RsAlleles { locus: String::from("rs9269173"), variants: vec![String::from("A")] }, 0.67),
                (RsAlleles { locus: String::from("rs9500974"), variants: vec![String::from("T")] }, 0.63),
                (RsAlleles { locus: String::from("rs12189871"), variants: vec![String::from("T")] }, 0.45),
                (RsAlleles { locus: String::from("rs12153924"), variants: vec![String::from("A")] }, 0.44),
                (RsAlleles { locus: String::from("rs371250843"), variants: vec![String::from("T")] }, 0.39),
                (RsAlleles { locus: String::from("rs9259118"), variants: vec![String::from("T")] }, 0.31),
                (RsAlleles { locus: String::from("rs559242105"), variants: vec![String::from("CTA")] }, 0.24),
                (RsAlleles { locus: String::from("rs17214657"), variants: vec![String::from("C")] }, -0.19),
                (RsAlleles { locus: String::from("rs9378176"), variants: vec![String::from("G")] }, -0.49),
                (RsAlleles { locus: String::from("rs2524277"), variants: vec![String::from("A")] }, -0.6),
                (RsAlleles { locus: String::from("rs6934289"), variants: vec![String::from("C")] }, -0.68),
                (RsAlleles { locus: String::from("rs16899379"), variants: vec![String::from("A")] }, -0.83),
                (RsAlleles { locus: String::from("rs149663102"), variants: vec![String::from("T")] }, -0.94)
            ]
        )
    );
    return if non_hla_score.contains_key(allele) { non_hla_score[allele] } else  { 0.0 }
}
