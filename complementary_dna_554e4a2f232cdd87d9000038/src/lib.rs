fn _dna_strand(dna: &str) -> String {
    dna.chars().map(|c| {
        match c.to_ascii_uppercase() {
            'A' => 'T',
            'T' => 'A',
            'G' => 'C',
            'C' => 'G',
            _ => panic!("invalid DNA base"),
        }
    }).collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::_dna_strand;

    fn dotest(s: &str, expected: &str) {
        let actual = _dna_strand(s);
        assert_eq!(actual, expected,
                "With dna = \"{s}\"\nExpected \"{expected}\" but got \"{actual}\"")
    }

    #[test]
    fn fixed_tests() {
        dotest("AAAA","TTTT");
        dotest("ATTGC","TAACG");
        dotest("GTAT","CATA");
    }
}