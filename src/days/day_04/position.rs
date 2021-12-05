#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct GridPosition {
    column: usize,
    row: usize,
}



impl GridPosition {
    pub fn of(column: usize, row: usize) -> Self {
        GridPosition { column, row }
    }

    pub fn column(&self) -> usize {
        self.column
    }

    pub fn row(&self) -> usize {
        self.row
    }
}
