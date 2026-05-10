fn _dna_to_rna(dna: &str) -> String {
    dna.replace("T", "U")
}

#[cfg(test)]
mod tests {
    use super::_dna_to_rna;

    #[test]
    fn returns_expected() {
        assert_eq!(_dna_to_rna("TTTT"), "UUUU");
        assert_eq!(_dna_to_rna("GCAT"), "GCAU");
    }
}
