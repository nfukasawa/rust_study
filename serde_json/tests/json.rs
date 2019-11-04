use serde_json::Deserialize;

#[derive(Deserialize, Debug)]
struct Hoge {
    num: i64,
    str: String,
    bool: bool,
    vec: Vec<String>,
    fuga: Fuga,
}

#[derive(Deserialize, Debug)]
struct Fuga {}

#[test]
fn test_hoge() {
    let hoge = Hoge {
        num: 0,
        str: "hello".to_string(),
        bool: false,
        vec: vec!["hello".to_string()],
        fuga: Fuga {},
    };
    hoge.foo();
}
