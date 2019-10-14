#[derive(Debug, Clone)]
pub enum Cell {
    Alive,
    Dead,
}

#[derive(Debug)]
pub struct LifeGame {
    field: Vec<Vec<Cell>>,
}

impl LifeGame {
    pub fn new(width: usize, heigh: usize) -> Self {
        Self {
            field: vec![vec![Cell::Dead; heigh]; width],
        }
    }

    pub fn with_field(field: Vec<Vec<Cell>>) -> Self {
        Self { field }
    }

    pub fn set(&mut self, x: usize, y: usize, val: Cell) -> Option<&Cell> {
        match self.cell_mut(x, y) {
            Some(cell) => {
                *cell = val;
                Some(cell)
            }
            None => None,
        }
    }

    pub fn get_field(&self) -> &Vec<Vec<Cell>> {
        &self.field
    }

    pub fn next(&mut self) -> &mut Self {
        self.field = self.convert_field(|x, y, cell| match (cell, self.num_neighbor_alive(x, y)) {
            (Cell::Alive, 2) | (Cell::Alive, 3) | (Cell::Dead, 3) => Cell::Alive,
            _ => Cell::Dead,
        });
        self
    }

    fn num_neighbor_alive(&self, x: usize, y: usize) -> usize {
        vec![
            (x - 1, y - 1),
            (x - 1, y),
            (x, y - 1),
            (x + 1, y - 1),
            (x - 1, y + 1),
            (x, y + 1),
            (x + 1, y),
            (x + 1, y + 1),
        ]
        .iter()
        .map(|(x, y)| match self.cell(*x, *y) {
            Some(cell) => match cell {
                Cell::Alive => 1,
                Cell::Dead => 0,
            },
            None => 0,
        })
        .sum()
    }

    fn convert_field<F>(&self, callback: F) -> Vec<Vec<Cell>>
    where
        F: Fn(usize, usize, &Cell) -> Cell,
    {
        self.field
            .iter()
            .enumerate()
            .map(|(y, row)| {
                row.iter()
                    .enumerate()
                    .map(|(x, cell)| callback(x, y, cell))
                    .collect()
            })
            .collect()
    }

    fn cell(&self, x: usize, y: usize) -> Option<&Cell> {
        match self.field.get(y) {
            Some(row) => row.get(x),
            None => None,
        }
    }

    fn cell_mut(&mut self, x: usize, y: usize) -> Option<&mut Cell> {
        match self.field.get_mut(y) {
            Some(row) => row.get_mut(x),
            None => None,
        }
    }
}
