#[cfg(test)]
mod file_path_parser_test {
    use crate::program_args_parsers::file_path_parser;

    #[test]
    #[should_panic]
    fn get_alleles_from_args_argument_not_exist() {
        let args = Vec::new();
        file_path_parser::get_alleles_from_args(args);
    }

    #[test]
    fn get_alleles_from_args_file_exist() {
        let mut args = Vec::with_capacity(2);
        args.push(String::from("line"));
        args.push(String::from("./src/program_args_parsers/example.json"));
        let result = file_path_parser::get_alleles_from_args(args);
        assert_eq!(result.h1_dqa1, "05:01");
        assert_eq!(result.h1_dqb1, "02:01");
        assert_eq!(result.h2_dqa1, "03:0X");
        assert_eq!(result.h2_dqb1, "03:02");
        assert_eq!(result.non_hla_alleles[0].locus, "rs233");
        assert_eq!(result.non_hla_alleles[0].variants[0], "A");
        assert_eq!(result.non_hla_alleles[0].variants[1], "G");
        assert_eq!(result.other_hla_alleles[0].locus, "rs111");
        assert_eq!(result.other_hla_alleles[0].variants[0], "C");
        assert_eq!(result.other_hla_alleles[0].variants[1], "A");
    }
}