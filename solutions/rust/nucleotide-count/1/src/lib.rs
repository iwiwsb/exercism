use std::collections::HashMap;

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    if !['A', 'C', 'G', 'T'].contains(&nucleotide) {
        return Err(nucleotide);
    }

    let mut count = 0;
    for n in dna.chars() {
        if !['A', 'C', 'G', 'T'].contains(&n) {
            return Err(n);
        }

        if n == nucleotide {
            count += 1;
        }
    }
    Ok(count)
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let mut nucleotide_counts: HashMap<char, usize> = HashMap::new();
    nucleotide_counts.insert('A', count('A', dna)?);
    nucleotide_counts.insert('C', count('C', dna)?);
    nucleotide_counts.insert('G', count('G', dna)?);
    nucleotide_counts.insert('T', count('T', dna)?);
    Ok(nucleotide_counts)
}
