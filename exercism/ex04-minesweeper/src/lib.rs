pub fn annotate(minefield: &[&str]) -> Vec<String> {
    Board::new(minefield).annotate()
}

struct Board {
    cells: Vec<Vec<Cell>>,
}

impl Board {
    fn new(field: &[&str]) -> Board {
        let cells = field.iter().map(|&raw| raw.chars().map(|c| c).collect()).collect();
        Board { cells: init_cells(&cells) }
    }

    fn annotate(&self) -> Vec<String> {
        self.cells.iter().map(|raw| raw.iter().map(|cell| cell.to_char()).collect()).collect()
    }
}

fn init_cells(cells: &Vec<Vec<char>>) -> Vec<Vec<Cell>> {
    (0..cells.len())
        .map(|y| {cells[y].iter().enumerate().map(|(x, &ch)| match ch {
            '*' => Cell::Mine,
            _ => Cell::Count(count_cell(cells, x, y)),
        }).collect()
    }).collect()
}

fn count_cell(cells: &Vec<Vec<char>>, x: usize, y: usize) -> u8 {
    let mut bs = vec![
        is_mine(cells, x, y, -1, -1),
        is_mine(cells, x, y, -1, 0),
        is_mine(cells, x, y, 0, -1),
        is_mine(cells, x, y, -1, 1),
        is_mine(cells, x, y, 1, -1),
        is_mine(cells, x, y, 0, 1),
        is_mine(cells, x, y, 1, 0),
        is_mine(cells, x, y, 1, 1),
    ];
    bs.retain(|&b| b);
    bs.len() as u8
}

fn is_mine(cells: &Vec<Vec<char>>, x: usize, y: usize, xoffset: i32, yoffset: i32) -> bool {
    if x == 0 && xoffset < 0 || y == 0 && yoffset < 0 {
        false
    } else {
        match cells.get(((y as i32) + yoffset) as usize) {
            Some(raw) => match raw.get(((x as i32) + xoffset) as usize) {
                Some(cell) => (*cell == '*'),
                None => false,
            },
            None => false,
        }
    }
}

enum Cell {
    Mine,
    Count(u8),
}

impl Cell {
    fn to_char(&self) -> char {
        match self {
            Cell::Mine => '*',
            Cell::Count(n) => {
                if *n == (0 as u8) {
                    ' '
                } else {
                    (('0' as u8) + *n) as char
                }
            }
        }
    }
}
