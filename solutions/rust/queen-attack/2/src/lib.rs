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
    pub fn new(rank: i32, file: i32) -> Option<Self> {
        match (rank, file) {
            (0..=7, 0..=7) => Some(Self { rank, file }),
            (_, _) => None,
        }
    }
}

impl Queen {
    pub fn new(position: ChessPosition) -> Self {
        Self { position }
    }

    pub fn can_attack(&self, other: &Queen) -> bool {
        self.on_same_row(other) || self.on_same_column(other) || self.on_same_diag(other)
    }

    fn row(&self) -> i32 {
        self.position.rank
    }

    fn column(&self) -> i32 {
        self.position.file
    }

    fn on_same_row(&self, other: &Queen) -> bool {
        self.row() == other.row()
    }

    fn on_same_column(&self, other: &Queen) -> bool {
        self.column() == other.column()
    }

    fn on_same_diag(&self, other: &Queen) -> bool {
        self.rank_distance(other) == self.file_distance(other)
    }

    fn rank_distance(&self, other: &Queen) -> i32 {
        if self.on_same_row(other) {
            0
        } else {
            (self.row() - other.row()).abs()
        }
    }

    fn file_distance(&self, other: &Queen) -> i32 {
        if self.on_same_column(other) {
            0
        } else {
            (self.column() - other.column()).abs()
        }
    }
}
