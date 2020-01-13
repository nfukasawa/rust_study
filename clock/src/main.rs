use chrono::Local;
use std::time::{Duration, Instant};
use tokio::prelude::*;
use tokio::timer::Interval;

use console::Term;

fn main() {
    let term = Term::stdout();

    tokio::run(
        Interval::new(Instant::now(), Duration::new(1, 0))
            .for_each(move |_| {
                let now = Local::now();
                term.clear_screen().unwrap();
                term.write_line(now.format("%F (%a) %T").to_string().as_str())
                    .unwrap();
                Ok(())
            })
            .map_err(|e| panic!("error: {:?}", e)),
    );
}
