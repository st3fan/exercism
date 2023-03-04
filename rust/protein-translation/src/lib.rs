use std::collections::HashMap;

pub struct CodonsInfo<'a> {
    codons: HashMap<&'a str, &'a str>,
}

impl<'a> CodonsInfo<'a> {
    pub fn name_for(&self, codon: &str) -> Option<&'a str> {
        if let Some(protein) = self.codons.get(codon) {
            return Some(*protein);
        }
        None
    }

    pub fn of_rna(&self, rna: &str) -> Option<Vec<&'a str>> {
        let mut result: Vec<&str> = Vec::new();

        // My kingdom for a String::chunk ?
        for codon in rna
            .chars()
            .collect::<Vec<char>>()
            .chunks(3)
            .map(|v| v.iter().collect::<String>())
        {
            if codon.len() != 3 {
                return None;
            }

            let Some(protein) = self.name_for(&codon) else {
                return None;
            };

            if protein == "stop codon" {
                return Some(result);
            }

            result.push(protein);
        }

        Some(result)
    }
}

pub fn parse<'a>(pairs: Vec<(&'a str, &'a str)>) -> CodonsInfo<'a> {
    let mut codons = HashMap::new();
    for (codon, protein) in pairs {
        codons.insert(codon, protein);
    }
    CodonsInfo { codons }
}
