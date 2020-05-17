use std::collections::HashMap;

pub struct CodonsInfo<'a> {
    codons: HashMap<&'a str, &'a str>,
}

impl<'a> CodonsInfo<'a> {
    pub fn name_for(&self, codon: &str) -> Option<&'a str> {
        self.codons.get(codon).copied()
    }

    pub fn of_rna(&self, rna: &str) -> Option<Vec<&'a str>> {
        rna.chars()
            .collect::<Vec<char>>()
            .chunks(3)
            .map(|chunk| chunk.iter().collect::<String>())
            .map(|codon| self.name_for(&codon))
            .take_while(|&name| name != Some("stop codon"))
            .collect()
    }
}

pub fn parse<'a>(pairs: Vec<(&'a str, &'a str)>) -> CodonsInfo<'a> {
    let mut codons: HashMap<&'a str, &'a str> = HashMap::new();

    for (codon, name) in pairs {
        codons.insert(codon, name);
    }

    CodonsInfo { codons }
}
