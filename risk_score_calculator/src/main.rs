#[macro_use] extern crate serde_derive;
mod haplotype_risk_score;
mod rs_risk_score;
mod program_args_parsers;

use crate::program_args_parsers::input_structures::RsAlleles;
use std::array::IntoIter;
use std::iter::FromIterator;
use std::collections::HashMap;
use haplotype_risk_score::haplotype_risk_calculator;
use rs_risk_score::rs_risk_calculator;
use program_args_parsers::file_path_parser;
use std::env;

fn main() {
    let args = env::args().collect();
    let alleles = file_path_parser::get_alleles_from_args(args);
    let other_hla_result = rs_risk_calculator::calculate_risk_score(&alleles.non_hla_alleles, get_non_hla_scores());
    let non_hla_result = rs_risk_calculator::calculate_risk_score(&alleles.other_hla_alleles, get_non_hla_scores());
    let main_hla_result = haplotype_risk_calculator::calculate_risk_score(&alleles);
    print_result_message(main_hla_result, other_hla_result, non_hla_result);
}

fn print_result_message(main_hla_result: f32, other_hla_result: f32, non_hla_result: f32) {
    let message = format!(r#"
    genetic risk calculator
    ========================
    main_hla: {},
    other_hla_result: {}, 
    non_hla_result:: {}. "# ,
    main_hla_result, other_hla_result, non_hla_result);
    println!("{}", message);
}

fn get_non_hla_scores() -> HashMap<RsAlleles, f32> {
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
    return non_hla_score;
}
