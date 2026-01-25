use std::collections::HashMap;

pub struct CodonsInfo<'a> {
    codons_info: HashMap<&'a str, &'a str>,
}

impl<'a> CodonsInfo<'a> {
    pub fn name_for(&self, codon: &str) -> Option<&'a str> {
        self.codons_info.get(codon).copied()
    }

    pub fn of_rna(&self, rna: &str) -> Option<Vec<&'a str>> {
        let mut codons: Vec<&'a str> = Vec::new();
        let chars: Vec<char> = rna.chars().collect();
        let chunks = chars.chunks(3);
        for chunk in chunks {
            let codon: String = chunk.into_iter().collect();
            let name = self.name_for(codon.as_str());
            match name {
                Some("stop codon") => break,
                Some(value) => codons.push(value),
                None => return None,
            }
        }
        Some(codons)
    }
}

pub fn parse<'a>(pairs: Vec<(&'a str, &'a str)>) -> CodonsInfo<'a> {
    let codons_info: HashMap<&'a str, &'a str> = pairs.into_iter().collect();
    CodonsInfo { codons_info }
}
