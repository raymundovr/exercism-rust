#[derive(Debug)]
pub struct ChessPosition {
    rank: i32,
    file: i32,
}

#[derive(Debug)]
pub struct Queen {
    position: ChessPosition,
}

impl ChessPosition {
    fn is_in_bounds(position: i32) -> bool {
        position >= 0 && position < 8 
    }

    pub fn new(rank: i32, file: i32) -> Option<Self> {
        if !Self::is_in_bounds(rank) || !Self::is_in_bounds(file) {
            return None;
        }

        Some(ChessPosition {
            rank,
            file
        })
    }
}

impl Queen {
    pub fn new(position: ChessPosition) -> Self {
        Queen {
            position
        }
    }

    pub fn can_attack(&self, other: &Queen) -> bool {       
        let rank_diff = (self.position.rank - other.position.rank).abs();
        let file_diff = (self.position.file - other.position.file).abs();

        if rank_diff == 0 || file_diff == 0 || rank_diff == file_diff {
            return true;
        }

        false
    }
}
