use rand::Rng;
use std::collections::HashSet;

pub struct Robot {
    name: String,
    previous_names: HashSet<String>,
}

impl Robot {
    pub fn new() -> Self {
        Robot {
            name: Self::generate_name(),
            previous_names: HashSet::new(),
        }
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn reset_name(&mut self) {
        let mut generated = Self::generate_name();
        while self.previous_names.contains(&generated) {
            generated = Self::generate_name();
        }
        self.name = generated;
    }

    fn generate_name() -> String {
        let letters: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
        let digits: &[u8] = b"0123456789";

        let mut name = Self::gen_substring(2, letters);
        name.push_str(&Self::gen_substring(3, digits));
        name
    }

    fn gen_substring(amount: u8, sample: &[u8]) -> String {
        let mut rng = rand::thread_rng();
        (0..amount)
            .map(|_| {
                let idx = rng.gen_range(0, sample.len());
                sample[idx] as char
            })
            .collect()
    }
}
