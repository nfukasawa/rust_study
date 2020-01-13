use chrono::{Datelike, Local, Timelike};
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
                term.write_line(
                    format!(
                        "{}-{:02}-{:02} ({}) {:02}:{:02}:{:02}",
                        now.year(),
                        now.month(),
                        now.day(),
                        now.weekday(),
                        now.hour(),
                        now.minute(),
                        now.second()
                    )
                    .as_str(),
                )
                .unwrap();
                Ok(())
            })
            .map_err(|e| panic!("error: {:?}", e)),
    );
}
