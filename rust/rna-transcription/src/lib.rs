#[derive(Debug, PartialEq, Eq)]
enum DnaNucleotide {
    G,
    C,
    T,
    A,
}

impl DnaNucleotide {
    fn complement(&self) -> RnaNucleotide {
        match self {
            DnaNucleotide::G => RnaNucleotide::C,
            DnaNucleotide::C => RnaNucleotide::G,
            DnaNucleotide::T => RnaNucleotide::A,
            DnaNucleotide::A => RnaNucleotide::U,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Dna {
    nucleotides: Vec<DnaNucleotide>,
}

#[derive(Debug, PartialEq, Eq)]
enum RnaNucleotide {
    A,
    C,
    G,
    U,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Rna {
    nucleotides: Vec<RnaNucleotide>,
}

impl Dna {
    pub fn new(dna: &str) -> Result<Dna, usize> {
        let mut nucleotides = Vec::new();
        for (i, c) in dna.chars().enumerate() {
            nucleotides.push(match c {
                'G' => DnaNucleotide::G,
                'C' => DnaNucleotide::C,
                'T' => DnaNucleotide::T,
                'A' => DnaNucleotide::A,
                _ => return Err(i),
            });
        }
        Ok(Self { nucleotides })
    }

    pub fn into_rna(self) -> Rna {
        Rna::from(self)
    }
}

impl Rna {
    pub fn new(rna: &str) -> Result<Rna, usize> {
        let mut nucleotides = Vec::new();
        for (i, c) in rna.chars().enumerate() {
            nucleotides.push(match c {
                'A' => RnaNucleotide::A,
                'C' => RnaNucleotide::C,
                'G' => RnaNucleotide::G,
                'U' => RnaNucleotide::U,
                _ => return Err(i),
            });
        }
        Ok(Self { nucleotides })
    }
}

impl From<Dna> for Rna {
    fn from(dna: Dna) -> Self {
        Self {
            nucleotides: dna
                .nucleotides
                .into_iter()
                .map(|n| n.complement())
                .collect(),
        }
    }
}
