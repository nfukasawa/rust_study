pub fn hello_world1() -> String {
    "Hello, World!".to_string()
}

pub const fn hello_world2() -> &'static str {
    "Hello, World!"
}

#[test]
fn test_hello_world() {
    assert_eq!("Hello, World!", hello_world1());
    assert_eq!("Hello, World!", hello_world2());
}
