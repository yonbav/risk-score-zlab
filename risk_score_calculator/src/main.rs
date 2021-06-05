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
    let other_hla_result = rs_risk_calculator::calculate_risk_score(&alleles.non_hla_alleles, get_other_hla_scores());
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

fn get_other_hla_scores() -> HashMap<RsAlleles, f32> {
    let other_hla_score: HashMap<RsAlleles, f32> = HashMap::<_, _>::from_iter(
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
    return other_hla_score;
}

fn get_non_hla_scores() -> HashMap<RsAlleles, f32> {
    let non_hla_score: HashMap<RsAlleles, f32> = HashMap::<_, _>::from_iter(
        IntoIter::new(
            [
                (RsAlleles { locus: String::from("rs3842753"), variants: vec![String::from("G")] }, 0.83),
                (RsAlleles { locus: String::from("rs2476601"), variants: vec![String::from("A")] }, 0.64),
                (RsAlleles { locus: String::from("rs2289702"), variants: vec![String::from("C")] }, 0.28),
                (RsAlleles { locus: String::from("rs653178"), variants: vec![String::from("C")] }, 0.26),
                (RsAlleles { locus: String::from("rs4948088"), variants: vec![String::from("C")] }, 0.26),
                (RsAlleles { locus: String::from("rs9924471"), variants: vec![String::from("A")] }, 0.22),
                (RsAlleles { locus: String::from("rs4759229"), variants: vec![String::from("A")] }, 0.22),
                (RsAlleles { locus: String::from("rs1893217"), variants: vec![String::from("G")] }, 0.19),
                (RsAlleles { locus: String::from("rs60888743"), variants: vec![String::from("A")] }, 0.18),
                (RsAlleles { locus: String::from("rs11170466"), variants: vec![String::from("A")] }, 0.18),
                (RsAlleles { locus: String::from("rs9981624"), variants: vec![String::from("T")] }, 0.17),
                (RsAlleles { locus: String::from("rs9388489"), variants: vec![String::from("C")] }, 0.17),
                (RsAlleles { locus: String::from("rs5763779"), variants: vec![String::from("A")] }, 0.16),
                (RsAlleles { locus: String::from("rs425105"), variants: vec![String::from("A")] }, 0.15),
                (RsAlleles { locus: String::from("rs72727394"), variants: vec![String::from("T")] }, 0.15),
                (RsAlleles { locus: String::from("rs17388568"), variants: vec![String::from("T")] }, 0.14),
                (RsAlleles { locus: String::from("rs1615504"), variants: vec![String::from("A")] }, 0.12),
                (RsAlleles { locus: String::from("rs6476839"), variants: vec![String::from("T")] }, 0.11),
                (RsAlleles { locus: String::from("rs9585056"), variants: vec![String::from("C")] }, 0.11),
                (RsAlleles { locus: String::from("rs229541"), variants: vec![String::from("A")] }, 0.10),
                (RsAlleles { locus: String::from("rs2281808"), variants: vec![String::from("C")] }, 0.10),
                (RsAlleles { locus: String::from("rs1738074"), variants: vec![String::from("T")] }, -0.08),
                (RsAlleles { locus: String::from("rs56994090"), variants: vec![String::from("C")] }, -0.13),
                (RsAlleles { locus: String::from("rs10492166"), variants: vec![String::from("A")] }, -0.14),
                (RsAlleles { locus: String::from("rs3024505"), variants: vec![String::from("A")] }, -0.15),
                (RsAlleles { locus: String::from("rs2111485"), variants: vec![String::from("A")] }, -0.16),
                (RsAlleles { locus: String::from("rs3087243"), variants: vec![String::from("A")] }, -0.17),
                (RsAlleles { locus: String::from("rs12708716"), variants: vec![String::from("G")] }, -0.19),
                (RsAlleles { locus: String::from("rs144309607"), variants: vec![String::from("T")] }, -0.40),
                (RsAlleles { locus: String::from("rs61839660"), variants: vec![String::from("T")] }, -0.48),
                (RsAlleles { locus: String::from("rs41295121"), variants: vec![String::from("T")] }, -0.71)
            ]
        )
    );
    return non_hla_score;
}
