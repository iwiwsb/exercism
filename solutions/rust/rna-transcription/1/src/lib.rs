use Nucleotide::*;

const DNA_NUCLEOTIDES: [Nucleotide; 4] = [Adenine, Cytosine, Guanine, Thymine];
const RNA_NUCLEOTIDES: [Nucleotide; 4] = [Adenine, Cytosine, Guanine, Uracil];

#[derive(Debug, PartialEq, Eq)]
enum Nucleotide {
    Adenine,
    Cytosine,
    Guanine,
    Thymine,
    Uracil,
}

impl TryFrom<char> for Nucleotide {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'A' => Ok(Adenine),
            'C' => Ok(Cytosine),
            'G' => Ok(Guanine),
            'T' => Ok(Thymine),
            'U' => Ok(Uracil),
            _ => Err(()),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Dna {
    nucl: Vec<Nucleotide>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Rna {
    nucl: Vec<Nucleotide>,
}

impl Dna {
    pub fn new(dna: &str) -> Result<Dna, usize> {
        let nucl = nucl_from_str(dna, &DNA_NUCLEOTIDES)?;
        Ok(Self { nucl })
    }

    pub fn into_rna(self) -> Rna {
        let nucl = self
            .nucl
            .iter()
            .map(|n| match n {
                Adenine => Uracil,
                Cytosine => Guanine,
                Guanine => Cytosine,
                Thymine => Adenine,
                Uracil => Uracil,
            })
            .collect();
        Rna { nucl }
    }
}

impl Rna {
    pub fn new(rna: &str) -> Result<Rna, usize> {
        let nucl = nucl_from_str(rna, &RNA_NUCLEOTIDES)?;
        Ok(Self { nucl })
    }
}

fn nucl_from_str(input: &str, nucleotides: &[Nucleotide]) -> Result<Vec<Nucleotide>, usize> {
    let mut nucl: Vec<Nucleotide> = Vec::new();
    for (i, c) in input.chars().enumerate() {
        match Nucleotide::try_from(c) {
            Ok(n) => {
                if nucleotides.contains(&n) {
                    nucl.push(n);
                } else {
                    return Err(i);
                }
            }
            Err(_) => return Err(i),
        }
    }
    Ok(nucl)
}
