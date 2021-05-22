#[derive(Serialize, Deserialize, Debug)]
pub struct Alleles {
    pub h1_dqa1: String,
    pub h1_dqb1: String,
    pub h2_dqa1: String,
    pub h2_dqb1: String,
    pub other_alleles: Vec<RsAlleles>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RsAlleles {
    pub locus: String,
    pub variants: Vec<String>
}
