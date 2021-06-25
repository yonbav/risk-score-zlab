#[derive(Serialize, Deserialize, Debug)]
pub struct Alleles {
    pub h1_dqa1: String,
    pub h1_dqb1: String,
    pub h2_dqa1: String,
    pub h2_dqb1: String,
    pub non_hla_alleles: Vec<RsAlleles>,
    pub main_hla_alleles: Vec<RsAlleles>,
    pub other_hla_alleles: Vec<RsAlleles>
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash)]
pub struct RsAlleles {
    pub locus: String,
    pub variants: Vec<String>
}
