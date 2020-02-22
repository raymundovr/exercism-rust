const VALID_DNA: &str = &"ACGT";
const VALID_RNA: &str = &"ACGU";

#[derive(Debug, PartialEq)]
pub struct DNA {
    strand: String,
}

#[derive(Debug, PartialEq)]
pub struct RNA {
    strand: String,
}

mod util {
    pub fn validate_str(chain: &str, validation: &str) -> Result<String, usize> {
        let mut strand = String::new();

        for (i, c) in chain.chars().enumerate() {
            match validation.find(c) {
                Some(_) => strand.push(c),
                None => return Err(i),
            };
        }

        Ok(strand)
    }
}

impl DNA {
    pub fn new(dna: &str) -> Result<DNA, usize> {
        match util::validate_str(dna, VALID_DNA) {
            Ok(strand) => Ok(DNA { strand }),
            Err(i) => Err(i),
        }
    }

    pub fn into_rna(self) -> RNA {
        let strand: String = self
            .strand
            .chars()
            .map(|c| match c {
                'G' => 'C',
                'C' => 'G',
                'T' => 'A',
                'A' => 'U',
                _ => unreachable!(),
            })
            .collect();

        RNA::new(&strand).unwrap()
    }
}

impl RNA {
    pub fn new(rna: &str) -> Result<RNA, usize> {
        match util::validate_str(rna, VALID_RNA) {
            Ok(strand) => Ok(RNA { strand }),
            Err(i) => Err(i),
        }
    }
}
