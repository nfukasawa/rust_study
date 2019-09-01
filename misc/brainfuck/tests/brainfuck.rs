use brainfuck::Interpreter;

#[test]
fn test_brainfuck() {
    // (code, input, output)
    let cases: Vec<(&str, &str, &str)> = vec![
        (hello_world, "", "Hello World!!"),
        (",.,.,.,.", "hoge", "hoge"),
    ];
    for case in cases {
        let mut output = Vec::new();
        let mut bf = Interpreter::new(case.1.as_bytes(), &mut output);
        bf.interpret(case.0.as_bytes());
        assert_eq!(case.2, String::from_utf8(output).unwrap());
    }
}

const hello_world: &'static str = r#"
    +++++++++[>++++++++>+++++++++++>+++++<<<-]>.>++.+++++++..+++.
    >-------------.<<+++++++++++++++.>.+++.------.--------.>+..
    "#;
