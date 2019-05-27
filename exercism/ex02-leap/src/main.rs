mod leap;

fn main() {
    for y in 2000..2100 {
        println!("{} -> {}", y, leap::is_leap(y));
    }
}
