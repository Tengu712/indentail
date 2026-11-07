use super::*;

#[test]
fn test_only_comment() {
    let code = "// Hello, world!".to_string();
    let expected = vec![];

    assert_eq!(parse(code), Ok(expected));
}

#[test]
fn test_only_tag() {
    let code = "Tag".to_string();
    let expected = vec![Node {
        tag: "Tag".to_string(),
        ..Default::default()
    }];

    assert_eq!(parse(code), Ok(expected));
}

#[test]
fn test_contiguous_tags() {
    let code = "Tag1 Tag2".to_string();
    let expected = vec![
        Node {
            tag: "Tag1".to_string(),
            ..Default::default()
        },
        Node {
            tag: "Tag2".to_string(),
            ..Default::default()
        },
    ];

    assert_eq!(parse(code), Ok(expected));
}

#[test]
fn test_empty_option() {
    let code = "Tag()".to_string();
    let expected = vec![Node {
        tag: "Tag".to_string(),
        ..Default::default()
    }];

    assert_eq!(parse(code), Ok(expected));
}

#[test]
fn test_optioned() {
    let code = "Tag(opt1: val1, opt2: val2)".to_string();
    let expected = vec![Node {
        tag: "Tag".to_string(),
        options: HashMap::from([
            ("opt1".to_string(), "val1".to_string()),
            ("opt2".to_string(), "val2".to_string()),
        ]),
        ..Default::default()
    }];

    assert_eq!(parse(code), Ok(expected));
}

#[test]
fn test_trailing_comma() {
    let code = "Tag(opt1: val1, opt2: val2,)".to_string();
    let expected = vec![Node {
        tag: "Tag".to_string(),
        options: HashMap::from([
            ("opt1".to_string(), "val1".to_string()),
            ("opt2".to_string(), "val2".to_string()),
        ]),
        ..Default::default()
    }];

    assert_eq!(parse(code), Ok(expected));
}

#[test]
fn test_empty_children() {
    let code = "Tag {}".to_string();
    let expected = vec![Node {
        tag: "Tag".to_string(),
        ..Default::default()
    }];

    assert_eq!(parse(code), Ok(expected));
}

#[test]
fn test_nested() {
    let code = "
Tag1(opt11: val11, opt12: val12) {
    Tag2()
    Tag3 {
        Tag4(opt41: val41) {}
    }
}
Tag5{}
"
    .to_string();
    let expected = vec![
        Node {
            tag: "Tag1".to_string(),
            options: HashMap::from([
                ("opt11".to_string(), "val11".to_string()),
                ("opt12".to_string(), "val12".to_string()),
            ]),
            children: vec![
                Node {
                    tag: "Tag2".to_string(),
                    ..Default::default()
                },
                Node {
                    tag: "Tag3".to_string(),
                    children: vec![Node {
                        tag: "Tag4".to_string(),
                        options: HashMap::from([("opt41".to_string(), "val41".to_string())]),
                        ..Default::default()
                    }],
                    ..Default::default()
                },
            ],
            ..Default::default()
        },
        Node {
            tag: "Tag5".to_string(),
            ..Default::default()
        },
    ];

    assert_eq!(parse(code), Ok(expected));
}

#[test]
fn test_quote() {
    let code = r#"'Tag 1'('key
1': '\\\t\r\n\'')"#
        .to_string();
    let expected = vec![Node {
        tag: "Tag 1".to_string(),
        options: HashMap::from([("key\n1".to_string(), "\\\t\r\n'".to_string())]),
        ..Default::default()
    }];

    assert_eq!(parse(code), Ok(expected));
}

#[test]
fn test_empty_quote() {
    let code = "''".to_string();
    let expected = vec![];

    assert_eq!(parse(code), Ok(expected));
}

#[test]
fn test_quote_in_the_middle() {
    let code = "ab'c'de".to_string();
    let expected = vec![
        Node {
            tag: "ab".to_string(),
            ..Default::default()
        },
        Node {
            tag: "c".to_string(),
            ..Default::default()
        },
        Node {
            tag: "de".to_string(),
            ..Default::default()
        },
    ];

    assert_eq!(parse(code), Ok(expected));
}

#[test]
fn test_slum() {
    let code = "
//s 日本語のあるcomment
	Tag/\\-あ(
😄: a🤛💥  ,  	 ){//
    !@#$%^&* -+12345 6 {? (<  :>)}
    // Tag(opt: val)
}"
    .to_string();
    let expected = vec![Node {
        tag: "Tag/\\-あ".to_string(),
        options: HashMap::from([("😄".to_string(), "a🤛💥".to_string())]),
        children: vec![
            Node {
                tag: "!@#$%^&*".to_string(),
                ..Default::default()
            },
            Node {
                tag: "-+12345".to_string(),
                ..Default::default()
            },
            Node {
                tag: "6".to_string(),
                children: vec![Node {
                    tag: "?".to_string(),
                    options: HashMap::from([("<".to_string(), ">".to_string())]),
                    ..Default::default()
                }],
                ..Default::default()
            },
        ],
        ..Default::default()
    }];

    assert_eq!(parse(code), Ok(expected));
}

#[test]
fn test_invalid_token() {
    let code = "Tag(opt1 val1)".to_string();
    let expected = IndentailError::Syntax {
        line: 1,
        column: 10,
        detail: SyntaxErrorDetail::InvalidToken("val1".to_string()),
    };

    assert_eq!(parse(code), Err(expected));
}

#[test]
fn test_invalid_colon() {
    let code = "Tag: opt1".to_string();
    let expected = IndentailError::Syntax {
        line: 1,
        column: 4,
        detail: SyntaxErrorDetail::InvalidColon,
    };

    assert_eq!(parse(code), Err(expected));
}

#[test]
fn test_invalid_comma() {
    let code = "Tag(,)".to_string();
    let expected = IndentailError::Syntax {
        line: 1,
        column: 5,
        detail: SyntaxErrorDetail::InvalidComma,
    };

    assert_eq!(parse(code), Err(expected));
}

#[test]
fn test_invalid_left_paren() {
    let code = "Tag {} (opt1: val1)".to_string();
    let expected = IndentailError::Syntax {
        line: 1,
        column: 8,
        detail: SyntaxErrorDetail::InvalidLeftParen,
    };

    assert_eq!(parse(code), Err(expected));
}

#[test]
fn test_invalid_right_paren() {
    let code = "Tag )".to_string();
    let expected = IndentailError::Syntax {
        line: 1,
        column: 5,
        detail: SyntaxErrorDetail::InvalidRightParen,
    };

    assert_eq!(parse(code), Err(expected));
}

#[test]
fn test_unclosed_paren() {
    let code = "Tag(a: b".to_string();
    let expected = IndentailError::Syntax {
        line: 1,
        column: 9,
        detail: SyntaxErrorDetail::UnclosedParen,
    };

    assert_eq!(parse(code), Err(expected));
}

#[test]
fn test_invalid_left_brace() {
    let code = "Tag(opt1: {})".to_string();
    let expected = IndentailError::Syntax {
        line: 1,
        column: 11,
        detail: SyntaxErrorDetail::InvalidLeftBrace,
    };

    assert_eq!(parse(code), Err(expected));
}

#[test]
fn test_invalid_right_brace() {
    let code = "Tag }".to_string();
    let expected = IndentailError::Syntax {
        line: 1,
        column: 5,
        detail: SyntaxErrorDetail::InvalidRightBrace,
    };

    assert_eq!(parse(code), Err(expected));
}

#[test]
fn test_unclosed_brace() {
    let code = "Tag { Tag2".to_string();
    let expected = IndentailError::Syntax {
        line: 1,
        column: 11,
        detail: SyntaxErrorDetail::UnclosedBrace,
    };

    assert_eq!(parse(code), Err(expected));
}

#[test]
fn test_invalid_escape() {
    let code = r#"Tag('a\x')"#.to_string();
    let expected = IndentailError::Syntax {
        line: 1,
        column: 7,
        detail: SyntaxErrorDetail::InvalidEscape('x'),
    };

    assert_eq!(parse(code), Err(expected));
}

#[test]
fn test_unclosed_quote() {
    let code = "Tag 'a\nb".to_string();
    let expected = IndentailError::Syntax {
        line: 1,
        column: 5,
        detail: SyntaxErrorDetail::UnclosedQuote,
    };

    assert_eq!(parse(code), Err(expected));
}
