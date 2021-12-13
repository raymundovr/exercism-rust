use std::collections::HashSet;
use std::cmp;

/**
 * [/-1, -1][/-1, 0][/-1, 1]
 * [/0, -1][0,0][/0, 1]
 * [/1, -1][1,0][/1,1]
**/

fn get_neighbours(
    row_index: &usize,
    minefield_length: &usize,
    ch_index: &usize,
    row_length: &usize,
) -> HashSet<(usize, usize)> {
    let mut set: HashSet<(usize, usize)> = HashSet::new();
    let min_row = match *row_index > 0 { 
        true => row_index - 1,
        false => *row_index
    };
    let min_col = match *ch_index > 0 {
        true => ch_index -1,
        false => *ch_index,
    };
    let max_row = cmp::min(minefield_length - 1, *row_index + 1);
    let max_col = cmp::min(row_length - 1, *ch_index + 1);
    
    set.insert((min_row, min_col));
    set.insert((min_row, *ch_index));
    set.insert((min_row, max_col));
    set.insert((*row_index, max_col));
    set.insert((max_row, max_col));
    set.insert((max_row, *ch_index));
    set.insert((max_row, min_col));
    set.insert((*row_index, min_col));

    set
}

fn count_mines_neighbours(row_index: &usize, ch_index: &usize, minefield: &[&str]) -> char {
    let count = get_neighbours(
        row_index,
        &minefield.len(),
        ch_index,
        &minefield[*row_index].len()
    )
    .iter()
    .map(|(r, c)|{
        match minefield[*r].chars().nth(*c).unwrap() {
            '*' => 1,
            _ => 0,
        }
    })
    .sum::<u32>();
    
    match count {
        0 => ' ',
        _ => char::from_digit(count, 10).unwrap(),
    }
}

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    minefield
        .iter()
        .enumerate()
        .map(|(row_index, row)| {
            row.chars()
                .enumerate()
                .map(|(ch_index, ch)| match ch {
                    '*' => '*',
                    _ => count_mines_neighbours(&row_index, &ch_index, minefield),
                })
                .collect::<String>()
        })
        .collect::<Vec<_>>()
}
