use std::collections::HashMap;

fn is_valid_nucleotide(nucleotide: char) -> bool {
    nucleotide == 'A' || nucleotide == 'C' || nucleotide == 'G' || nucleotide == 'T'
}

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    if !is_valid_nucleotide(nucleotide) {
        return Err(nucleotide);
    }

    let mut count = 0;
    for n in dna.chars() {
        if !is_valid_nucleotide(n) {
            return Err(n);
        } else if n == nucleotide {
            count += 1;
        }
    }
    Ok(count)
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let mut a: usize = 0;
    let mut c: usize = 0;
    let mut g: usize = 0;
    let mut t: usize = 0;

    for n in dna.chars() {
        match n {
            'A' => a += 1,
            'C' => c += 1,
            'G' => g += 1,
            'T' => t += 1,
            _ => return Err(n),
        }
    }

    let mut counts = HashMap::new();
    counts.insert('A', a);
    counts.insert('C', c);
    counts.insert('G', g);
    counts.insert('T', t);
    Ok(counts)
}
