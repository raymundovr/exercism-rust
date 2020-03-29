#[derive(Debug)]
enum Direction {
    Right,
    Down,
    Left,
    Up,
}

fn get_direction(pos: (usize, usize), d: &Direction, min: usize, max: usize) -> Direction {
    match d {
        Direction::Right => {
            if pos.1 < max {
                return Direction::Right;
            }
            Direction::Down
        }
        Direction::Down => {
            if pos.0 < max {
                return Direction::Down;
            }
            Direction::Left
        }
        Direction::Left => {
            if pos.1 > min {
                return Direction::Left;
            }
            Direction::Up
        }
        Direction::Up => {
            if pos.0 > min {
                return Direction::Up;
            }
            Direction::Right
        }
    }
}

fn get_next_pos(current: (usize, usize), d: &Direction) -> (usize, usize) {
    match d {
        Direction::Right => (current.0, current.1 + 1),
        Direction::Down => (current.0 + 1, current.1),
        Direction::Left => (current.0, current.1 - 1),
        Direction::Up => (current.0 - 1, current.1),
    }
}

pub fn spiral_matrix(size: u32) -> Vec<Vec<u32>> {
    let mut matrix = vec![vec![0u32; size as usize]; size as usize];
    if size > 0 {
        let mut pos: (usize, usize) = (0, 0);
        let mut n = 1;
        let mut direction = Direction::Right;
        let mut min = 0;
        let mut max = (size as usize) - 1;

        while n <= (size * size) {
            matrix[pos.0][pos.1] = n;
            direction = get_direction(pos, &direction, min, max);
            pos = get_next_pos(pos, &direction);
            if pos == (max, min) {
                max -= 1;
                min += 1;
            }

            n += 1;
        }
    }

    matrix
}
