use lifegame::{Cell, LifeGame};

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

    for (field, next) in cases.iter() {
        let mut g = game(field);
        assert_eq!(game(next).get_field(), g.next().get_field());
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
