use std::collections::HashMap;

pub struct CodonsInfo<'a> {
    codons: HashMap<&'a str, &'a str>,
}

impl<'a> CodonsInfo<'a> {
    pub fn name_for(&self, codon: &str) -> Option<&'a str> {
        self.codons.get(codon).map(|&x| x)
    }

    pub fn of_rna(&self, rna: &str) -> Option<Vec<&'a str>> {
        rna.chars()
            .collect::<Vec<char>>()
            .chunks(3)
            .map(|chars| chars.iter().collect::<String>())
            .take_while(|rna| match rna.as_str() {
                "UAA" => false,
                "UAG" => false,
                "UGA" => false,
                _ => true,
            })
            .map(|rna| self.name_for(rna.as_str()))
            .collect()
    }
}

pub fn parse<'a>(pairs: Vec<(&'a str, &'a str)>) -> CodonsInfo<'a> {
    CodonsInfo {
        codons: pairs.into_iter().collect(),
    }
}
