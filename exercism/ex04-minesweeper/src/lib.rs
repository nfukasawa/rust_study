pub fn annotate(minefield: &[&str]) -> Vec<String> {
    Board::new(minefield).annotate()
}

struct Board {
    cells: Vec<Vec<Cell>>,
}

impl Board {
    fn new(field: &[&str]) -> Board {
        let cells = field.iter().map(|&row| row.chars().collect()).collect();
        Board { cells: init_cells(&cells) }
    }

    fn annotate(&self) -> Vec<String> {
        self.cells.iter().map(|row| row.iter().map(|cell| cell.to_char()).collect()).collect()
    }
}

fn init_cells(cells: &Vec<Vec<char>>) -> Vec<Vec<Cell>> {
    (0..cells.len())
        .map(|y| {cells[y].iter().enumerate().map(|(x, &c)| match c {
            '*' => Cell::Mine,
            _ => Cell::Count(count_cell(cells, x, y)),
        }).collect()
    }).collect()
}

fn count_cell(cells: &Vec<Vec<char>>, x: usize, y: usize) -> u8 {
    return vec![
        is_mine(cells, x, y, -1, -1),
        is_mine(cells, x, y, -1, 0),
        is_mine(cells, x, y, 0, -1),
        is_mine(cells, x, y, -1, 1),
        is_mine(cells, x, y, 1, -1),
        is_mine(cells, x, y, 0, 1),
        is_mine(cells, x, y, 1, 0),
        is_mine(cells, x, y, 1, 1),
    ].iter().fold(0, |sum, m| match m { true => sum + 1, false => sum, })
}

fn is_mine(cells: &Vec<Vec<char>>, x: usize, y: usize, dx: i32, dy: i32) -> bool {
    if x == 0 && dx < 0 || y == 0 && dy < 0 {
        return false
    }
    let x1 = ((x as i32) + dx) as usize;
    let y1 = ((y as i32) + dy) as usize;
    if cells.len() <= y1 || cells[y1].len() <= x1 {
        false
    } else {
        cells[y1][x1] == '*'
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
