use std::collections::HashMap;

static NUCLEOTIDES: [char; 4] = ['A', 'C', 'G', 'T'];

fn is_valid(c: &char) -> bool {
    NUCLEOTIDES.contains(c)
}

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    if !is_valid(&nucleotide) {
        return Err(nucleotide);
    }

    let mut count: usize = 0;
    for d in dna.chars() {
        if !is_valid(&d) {
            return Err(d);
        }
        if d == nucleotide {
            count += 1;
        }
    }

    Ok(count)
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let mut counts: HashMap<char, usize> = HashMap::new();
    for n in NUCLEOTIDES.iter() {
        counts.insert(*n, 0);
    }

    for c in dna.chars() {
        if !is_valid(&c) {
            return Err(c);
        }
        *counts.entry(c).or_insert(0) += 1;
    }

    Ok(counts)
}
