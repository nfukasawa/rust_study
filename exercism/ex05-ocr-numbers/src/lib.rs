// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

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
    if let Some(l) = raws.iter().find(|&l| l.len() % 3 != 0) {
        return Err(Error::InvalidColumnCount(l.len()));
    }

    let mut ret = Vec::<String>::new();
    for raw in raws.chunks(4) {
        ret.push(raw[0].char_indices().step_by(3).map(|(i, _)| 
            ocr((&raw[0][i..i + 3], &raw[1][i..i + 3], &raw[2][i..i + 3]))
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
