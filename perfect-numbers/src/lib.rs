#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient,
}

pub fn classify(num: u64) -> Option<Classification> {
    if num == 0 {
        return None;
    }

    let sum: u64 = (1..=(num / 2)).filter(|i| num % i == 0).sum();

    if sum < num {
        return Some(Classification::Deficient);
    } else if sum == num {
        return Some(Classification::Perfect);
    } else {
        return Some(Classification::Abundant);
    }
}
