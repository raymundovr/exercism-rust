use std::collections::HashSet;

pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {    
    let mut marked: HashSet<u64> = HashSet::new();
    
    (2..=upper_bound)
        .filter(|i| {
            marked.extend(((i+i)..=upper_bound).step_by(*i as usize));
            !marked.contains(i)
        })
        .collect()
    
}
