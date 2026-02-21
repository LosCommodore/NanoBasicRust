use super::*;

#[test]
fn test_match_token() {
    let params = [
        (
            r"rem hallo",
            Token {
                kind: TokenType::Comment,
                position: Position {
                    line_num: 0,
                    col_start: 0,
                    col_end: 9,
                },
            },
        ),
        (
            r"REM HaLLo",
            Token {
                kind: TokenType::Comment,
                position: Position {
                    line_num: 0,
                    col_start: 0,
                    col_end: 9,
                },
            },
        ),
        (
            r"goto",
            Token {
                kind: TokenType::Goto,
                position: Position {
                    line_num: 0,
                    col_start: 0,
                    col_end: 4,
                },
            },
        ),
        (
            r")",
            Token {
                kind: TokenType::CloseParen,
                position: Position {
                    line_num: 0,
                    col_start: 0,
                    col_end: 1,
                },
            },
        ),
        (
            r"ABC",
            Token {
                kind: TokenType::Variable("ABC".to_string()),
                position: Position {
                    line_num: 0,
                    col_start: 0,
                    col_end: 3,
                },
            },
        ),
    ];

    for (text, result) in &params {
        println!("Using regex: {}", *text);
        assert_eq!(match_token(*text, 0, 0).unwrap(), *result);
    }
}

#[test]
fn test_match_line() {
    let param = r"a = 3";

    let expected = [
        Token {
            kind: TokenType::Variable("a".to_string()),
            position: Position {
                line_num: 0,
                col_start: 0,
                col_end: 1,
            },
        },
        Token {
            kind: TokenType::Equal,
            position: Position {
                line_num: 0,
                col_start: 2,
                col_end: 3,
            },
        },
        Token {
            kind: TokenType::Number(3),
            position: Position {
                line_num: 0,
                col_start: 4,
                col_end: 5,
            },
        },
    ];

    println!("Using regex: {}", param);
    let m = tokenize_line(param, 0).unwrap();
    print!("{:#?}", m);
    assert_eq!(expected, *m);
}
