use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Cell {
    Alive = 1u8,
    Dead = 0u8,
}

#[wasm_bindgen]
#[derive(Debug, PartialEq)]
pub struct LifeGame {
    width: usize,
    height: usize,
    field: Vec<Cell>,
}

#[wasm_bindgen]
impl LifeGame {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            field: vec![Cell::Dead; width * height],
        }
    }

    pub fn fill_cells(&self, buf: &mut [u8]) -> bool {
        if buf.len() < self.width * self.height {
            return false;
        }
        self.field
            .iter()
            .enumerate()
            .for_each(|(i, cell)| buf[i] = if *cell == Cell::Alive { 1 } else { 0 });
        true
    }

    pub fn set_cell(&mut self, x: usize, y: usize, val: Cell) -> bool {
        match self.cell_mut(x, y) {
            Some(cell) => {
                *cell = val.clone();
                true
            }
            None => false,
        }
    }

    pub fn next(&mut self) {
        self.field = self.convert_field(|x, y, cell| match (cell, self.num_neighbor_alive(x, y)) {
            (Cell::Alive, 2) | (Cell::Alive, 3) | (Cell::Dead, 3) => Cell::Alive,
            _ => Cell::Dead,
        });
    }

    fn num_neighbor_alive(&self, x: usize, y: usize) -> usize {
        vec![
            (x.checked_sub(1), y.checked_sub(1)),
            (x.checked_sub(1), Some(y)),
            (Some(x), y.checked_sub(1)),
            (Some(x + 1), y.checked_sub(1)),
            (x.checked_sub(1), Some(y + 1)),
            (Some(x), Some(y + 1)),
            (Some(x + 1), Some(y)),
            (Some(x + 1), Some(y + 1)),
        ]
        .iter()
        .map(|(x, y)| {
            if let (Some(x), Some(y)) = (x, y) {
                match self.cell(*x, *y) {
                    Some(cell) => match cell {
                        Cell::Alive => 1,
                        Cell::Dead => 0,
                    },
                    None => 0,
                }
            } else {
                0
            }
        })
        .sum()
    }

    fn convert_field<F>(&self, callback: F) -> Vec<Cell>
    where
        F: Fn(usize, usize, &Cell) -> Cell,
    {
        self.field
            .iter()
            .enumerate()
            .map(|(pos, cell)| callback(pos % self.width, pos / self.width, cell))
            .collect()
    }

    fn cell(&self, x: usize, y: usize) -> Option<&Cell> {
        if x >= self.width || y >= self.height {
            None
        } else {
            self.field.get(x + self.width * y)
        }
    }

    fn cell_mut(&mut self, x: usize, y: usize) -> Option<&mut Cell> {
        if x >= self.width || y >= self.height {
            None
        } else {
            self.field.get_mut(x + self.width * y)
        }
    }
}

impl LifeGame {
    pub fn with_field(field: Vec<Vec<Cell>>) -> Self {
        Self {
            width: field.get(0).unwrap().len(),
            height: field.len(),
            field: field.concat(),
        }
    }

    pub fn get_field(&self) -> Vec<Vec<Cell>> {
        self.field
            .chunks(self.width)
            .map(|row| row.to_vec())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lifegame() {
        let cases = vec![
            ("---\n---\n---", "---\n---\n---"),
            ("*--\n---\n---", "---\n---\n---"),
            ("-*-\n---\n---", "---\n---\n---"),
            ("**-\n---\n---", "---\n---\n---"),
            ("*-*\n---\n---", "---\n---\n---"),
            ("*--\n-*-\n---", "---\n---\n---"),
            ("*--\n--*\n---", "---\n---\n---"),
            ("*--\n---\n--*", "---\n---\n---"),
            ("***\n---\n---", "-*-\n-*-\n---"),
            ("**-\n*--\n---", "**-\n**-\n---"),
            ("**-\n-*-\n---", "**-\n**-\n---"),
            ("**-\n--*\n---", "-*-\n-*-\n---"),
            ("**-\n---\n*--", "---\n**-\n---"),
            ("**-\n---\n--*", "---\n-*-\n---"),
            ("*-*\n-*-\n---", "-*-\n-*-\n---"),
            ("*-*\n---\n-*-", "---\n-*-\n---"),
            ("*--\n-*-\n--*", "---\n-*-\n---"),
            ("***\n*--\n---", "**-\n*--\n---"),
            ("***\n-*-\n---", "***\n***\n---"),
            ("***\n---\n*--", "-*-\n*--\n---"),
            ("***\n---\n-*-", "-*-\n*-*\n---"),
            ("**-\n**-\n---", "**-\n**-\n---"),
            ("**-\n*-*\n---", "**-\n*--\n---"),
            ("**-\n-**\n---", "***\n***\n---"),
            ("**-\n*--\n--*", "**-\n*--\n---"),
            ("**-\n-*-\n--*", "**-\n***\n---"),
            ("**-\n--*\n*--", "-*-\n*--\n---"),
            ("**-\n--*\n-*-", "-*-\n*-*\n---"),
            ("**-\n--*\n--*", "-*-\n--*\n---"),
            // TODO: and more patterns...
        ];

        for (i, (field, next)) in cases.iter().enumerate() {
            println!("{}", i);
            let mut g = game(field);
            g.next();
            assert_eq!(game(next).get_field(), g.get_field());
        }
    }

    fn game(s: &str) -> LifeGame {
        let mut field = Vec::new();
        for line in s.to_string().lines() {
            let mut row = Vec::new();
            for ch in line.chars() {
                match ch {
                    '*' => row.push(Cell::Alive),
                    '-' => row.push(Cell::Dead),
                    _ => (),
                }
            }
            if row.len() > 0 {
                field.push(row);
            }
        }
        LifeGame::with_field(field)
    }
}
