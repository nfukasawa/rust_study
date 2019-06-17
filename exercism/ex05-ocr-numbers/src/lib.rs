#[derive(Debug, PartialEq)]
pub enum Error {
    InvalidRowCount(usize),
    InvalidColumnCount(usize),
}

pub fn convert(input: &str) -> Result<String, Error> {
    let rows: Vec<&str> = input.split('\n').collect();

    if rows.len() % 4 != 0 {
        return Err(Error::InvalidRowCount(rows.len()));
    }
    if let Some(row) = rows.iter().find(|&row| row.len() % 3 != 0) {
        return Err(Error::InvalidColumnCount(row.len()));
    }

    let mut ret = Vec::<String>::new();
    for chunk in rows.chunks(4) {
        ret.push(chunk[0].char_indices().step_by(3).map(|(i, _)| 
            ocr((&chunk[0][i..i + 3], &chunk[1][i..i + 3], &chunk[2][i..i + 3]))
        ).collect::<String>());
    }
    Ok(ret.join(","))
}

#[rustfmt::skip]
fn ocr(rows: (&str, &str, &str)) -> char {
    match rows {
        ( " _ ",
          "| |",
          "|_|" ) => '0',
        ( "   ",
          "  |",
          "  |" ) => '1',
        ( " _ ",
          " _|",
          "|_ " ) => '2',
        ( " _ ",
          " _|",
          " _|" ) => '3',
        ( "   ",
          "|_|",
          "  |" ) => '4',
        ( " _ ",
          "|_ ",
          " _|" ) => '5',
        ( " _ ",
          "|_ ",
          "|_|" ) => '6',
        ( " _ ",
          "  |",
          "  |" ) => '7',
        ( " _ ",
          "|_|",
          "|_|" ) => '8',
        ( " _ ",
          "|_|",
          " _|" ) => '9',
        _ => '?',
    }
}
