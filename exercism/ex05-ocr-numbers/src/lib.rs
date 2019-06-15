#[derive(Debug, PartialEq)]
pub enum Error {
    InvalidRowCount(usize),
    InvalidColumnCount(usize),
}

pub fn convert(input: &str) -> Result<String, Error> {
    let raws: Vec<&str> = input.split('\n').collect();

    if raws.len() % 4 != 0 {
        return Err(Error::InvalidRowCount(raws.len()));
    }
    if let Some(raw) = raws.iter().find(|&raw| raw.len() % 3 != 0) {
        return Err(Error::InvalidColumnCount(raw.len()));
    }

    let mut ret = Vec::<String>::new();
    for chunk in raws.chunks(4) {
        ret.push(chunk[0].char_indices().step_by(3).map(|(i, _)| 
            ocr((&chunk[0][i..i + 3], &chunk[1][i..i + 3], &chunk[2][i..i + 3]))
        ).collect::<String>());
    }
    Ok(ret.join(","))
}

#[rustfmt::skip]
fn ocr(raws: (&str, &str, &str)) -> char {
    match raws {
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
