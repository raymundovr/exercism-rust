fn rise(n: u32, p: u32) -> u32 {
    n.pow(p)
}

pub fn is_armstrong_number(num: u32) -> bool {
    let num_str = num.to_string();
    let potence: u32 = num_str.len() as u32;

    let calc = num
        .to_string()
        .chars()
        .rev()
        .map(|c| rise(c.to_digit(10).unwrap(), potence))
        .sum::<u32>();

    calc == num
}
