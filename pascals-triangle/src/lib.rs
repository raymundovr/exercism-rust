pub struct PascalsTriangle {
    row_count: u32,
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        PascalsTriangle { row_count }
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        let mut rows: Vec<Vec<u32>> = Vec::new();
        if self.row_count == 0 {
            return rows;
        }

        //[[1]]
        //[[1], [1, 1]]
        //[[1], [1, 1], [1, 2, 1]]
        rows.push(vec![1]);

        for r in 1..self.row_count {
            let mut row: Vec<u32> = Vec::new();
            let last_row = &rows[(r - 1) as usize];
            for c in 0..=r {
                let mut left: u32 = 0;
                if c > 0 {
                    left = match last_row.get((c - 1) as usize) {
                        None => 0,
                        Some(v) => *v,
                    };
                }
                let right: u32 = match last_row.get(c as usize) {
                    None => 0,
                    Some(v) => *v,
                };

                row.push(left + right);
            }

            rows.push(row);
        }

        rows
    }
}
