use super::*;

fn syntax_error(code: &str) -> (usize, usize, SyntaxErrorDetail) {
    match parse(code.to_string()) {
        Err(TailortailError::Syntax {
            line,
            column,
            detail,
        }) => (line, column, detail),
        r => panic!("{r:?}"),
    }
}

#[test]
fn test_only_comment() {
    let code = "// Hello, world!".to_string();
    let expected = vec![];

    assert_eq!(parse(code).unwrap(), expected);
}

#[test]
fn test_only_tag() {
    let code = "Tag".to_string();
    let expected = vec![Node {
        tag: "Tag".to_string(),
        ..Default::default()
    }];

    assert_eq!(parse(code).unwrap(), expected);
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

    assert_eq!(parse(code).unwrap(), expected);
}

#[test]
fn test_empty_option() {
    let code = "Tag()".to_string();
    let expected = vec![Node {
        tag: "Tag".to_string(),
        ..Default::default()
    }];

    assert_eq!(parse(code).unwrap(), expected);
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

    assert_eq!(parse(code).unwrap(), expected);
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

    assert_eq!(parse(code).unwrap(), expected);
}

#[test]
fn test_empty_children() {
    let code = "Tag {}".to_string();
    let expected = vec![Node {
        tag: "Tag".to_string(),
        ..Default::default()
    }];

    assert_eq!(parse(code).unwrap(), expected);
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

    assert_eq!(parse(code).unwrap(), expected);
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

    assert_eq!(parse(code).unwrap(), expected);
}

#[test]
fn test_empty_quote() {
    let code = "''".to_string();
    let expected = vec![];

    assert_eq!(parse(code).unwrap(), expected);
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

    assert_eq!(parse(code).unwrap(), expected);
}

#[test]
fn test_modifiers() {
    let code = "
Tag1.m1(v1)
Tag2(a: b)
    .'m 2'('v 2')
    .'.m3'(v3)
Tag3 { Tag4 }.m4(v4)
"
    .to_string();
    let expected = vec![
        Node {
            tag: "Tag1".to_string(),
            modifiers: vec![("m1".to_string(), "v1".to_string())],
            ..Default::default()
        },
        Node {
            tag: "Tag2".to_string(),
            options: HashMap::from([("a".to_string(), "b".to_string())]),
            modifiers: vec![
                ("m 2".to_string(), "v 2".to_string()),
                (".m3".to_string(), "v3".to_string()),
            ],
            ..Default::default()
        },
        Node {
            tag: "Tag3".to_string(),
            modifiers: vec![("m4".to_string(), "v4".to_string())],
            children: vec![Node {
                tag: "Tag4".to_string(),
                ..Default::default()
            }],
            ..Default::default()
        },
    ];

    assert_eq!(parse(code).unwrap(), expected);
}

#[test]
fn test_sign_out_of_paren() {
    let code = "Tag:1 Tag,2".to_string();
    let expected = vec![
        Node {
            tag: "Tag:1".to_string(),
            ..Default::default()
        },
        Node {
            tag: "Tag,2".to_string(),
            ..Default::default()
        },
    ];

    assert_eq!(parse(code).unwrap(), expected);
}

#[test]
fn test_dot_in_paren() {
    let code = "Tag(a.b: c.d).m(v.w)".to_string();
    let expected = vec![Node {
        tag: "Tag".to_string(),
        options: HashMap::from([("a.b".to_string(), "c.d".to_string())]),
        modifiers: vec![("m".to_string(), "v.w".to_string())],
        ..Default::default()
    }];

    assert_eq!(parse(code).unwrap(), expected);
}

#[test]
fn test_spaces_in_value() {
    let code = "Tag(a:  b c \t d \t, e: f).m( g  h )".to_string();
    let expected = vec![Node {
        tag: "Tag".to_string(),
        options: HashMap::from([
            ("a".to_string(), "b c \t d".to_string()),
            ("e".to_string(), "f".to_string()),
        ]),
        modifiers: vec![("m".to_string(), "g  h".to_string())],
        ..Default::default()
    }];

    assert_eq!(parse(code).unwrap(), expected);
}

#[test]
fn test_empty_quote_value() {
    let code = "Tag(a: '').m('')".to_string();
    let expected = vec![Node {
        tag: "Tag".to_string(),
        options: HashMap::from([("a".to_string(), "".to_string())]),
        modifiers: vec![("m".to_string(), "".to_string())],
        ..Default::default()
    }];

    assert_eq!(parse(code).unwrap(), expected);
}

#[test]
fn test_slum() {
    let code = "
//s 日本語のあるcomment
	Tag/\\-あ(
😄: a🤛💥  ,  	 ){//
    !@#$%^&*: -+12345 6.7(8) ~ {, (<  :>)}
    // Tag(opt: val)
}
.😇😎
(
 あ)"
    .to_string();
    let expected = vec![Node {
        tag: "Tag/\\-あ".to_string(),
        options: HashMap::from([("😄".to_string(), "a🤛💥".to_string())]),
        modifiers: vec![("😇😎".to_string(), "あ".to_string())],
        children: vec![
            Node {
                tag: "!@#$%^&*:".to_string(),
                ..Default::default()
            },
            Node {
                tag: "-+12345".to_string(),
                ..Default::default()
            },
            Node {
                tag: "6".to_string(),
                modifiers: vec![("7".to_string(), "8".to_string())],
                ..Default::default()
            },
            Node {
                tag: "~".to_string(),
                children: vec![Node {
                    tag: ",".to_string(),
                    options: HashMap::from([("<".to_string(), ">".to_string())]),
                    ..Default::default()
                }],
                ..Default::default()
            },
        ],
    }];

    assert_eq!(parse(code).unwrap(), expected);
}

#[test]
fn test_nul() {
    let code = "T\0ag(k: 'v\0')".to_string();
    let expected = vec![Node {
        tag: "Tag".to_string(),
        options: HashMap::from([("k".to_string(), "v".to_string())]),
        ..Default::default()
    }];

    assert_eq!(parse(code).unwrap(), expected);
}

#[test]
fn test_invalid_token() {
    assert_eq!(
        syntax_error("Tag(opt1 val1)"),
        (1, 10, SyntaxErrorDetail::InvalidToken("val1".to_string()))
    );
}

#[test]
fn test_invalid_colon() {
    assert_eq!(
        syntax_error("Tag(:)"),
        (1, 5, SyntaxErrorDetail::InvalidColon)
    );
}

#[test]
fn test_invalid_colon2() {
    assert_eq!(
        syntax_error("Tag(opt::value)"),
        (1, 9, SyntaxErrorDetail::InvalidColon)
    );
}

#[test]
fn test_invalid_colon3() {
    assert_eq!(
        syntax_error("Tag(opt: value:)"),
        (1, 15, SyntaxErrorDetail::InvalidColon)
    );
}

#[test]
fn test_invalid_comma() {
    assert_eq!(
        syntax_error("Tag(,)"),
        (1, 5, SyntaxErrorDetail::InvalidComma)
    );
}

#[test]
fn test_invalid_comma2() {
    assert_eq!(
        syntax_error("Tag(a,)"),
        (1, 6, SyntaxErrorDetail::InvalidComma)
    );
}

#[test]
fn test_invalid_comma3() {
    assert_eq!(
        syntax_error("Tag(a: b,,)"),
        (1, 10, SyntaxErrorDetail::InvalidComma)
    );
}

#[test]
fn test_invalid_left_paren() {
    assert_eq!(
        syntax_error("Tag {} (opt1: val1)"),
        (1, 8, SyntaxErrorDetail::InvalidLeftParen)
    );
}

#[test]
fn test_invalid_right_paren() {
    assert_eq!(
        syntax_error("Tag )"),
        (1, 5, SyntaxErrorDetail::InvalidRightParen)
    );
}

#[test]
fn test_unclosed_paren() {
    assert_eq!(
        syntax_error("Tag(a: b"),
        (1, 9, SyntaxErrorDetail::UnclosedParen)
    );
}

#[test]
fn test_invalid_left_brace() {
    assert_eq!(
        syntax_error("Tag(opt1: {})"),
        (1, 11, SyntaxErrorDetail::InvalidLeftBrace)
    );
}

#[test]
fn test_invalid_right_brace() {
    assert_eq!(
        syntax_error("Tag }"),
        (1, 5, SyntaxErrorDetail::InvalidRightBrace)
    );
}

#[test]
fn test_unclosed_brace() {
    assert_eq!(
        syntax_error("Tag { Tag2"),
        (1, 11, SyntaxErrorDetail::UnclosedBrace)
    );
}

#[test]
fn test_invalid_escape() {
    assert_eq!(
        syntax_error(r#"Tag('a\x')"#),
        (1, 7, SyntaxErrorDetail::InvalidEscape('x'))
    );
}

#[test]
fn test_unclosed_quote() {
    assert_eq!(
        syntax_error("Tag 'a\nb"),
        (1, 5, SyntaxErrorDetail::UnclosedQuote)
    );
}

#[test]
fn test_invalid_dot() {
    assert_eq!(
        syntax_error(".Tag(a: b)"),
        (1, 1, SyntaxErrorDetail::InvalidDot)
    );
}

#[test]
fn test_modifier_without_value() {
    assert_eq!(
        syntax_error("Tag.m()"),
        (1, 7, SyntaxErrorDetail::InvalidRightParen)
    );
}

#[test]
fn test_options_after_modifier() {
    assert_eq!(
        syntax_error("Tag.m(v)(a: b)"),
        (1, 9, SyntaxErrorDetail::InvalidLeftParen)
    );
}

#[test]
fn test_children_after_modifier() {
    assert_eq!(
        syntax_error("Tag.m(v) {}"),
        (1, 10, SyntaxErrorDetail::InvalidLeftBrace)
    );
}

#[test]
fn test_duplicate_option() {
    assert_eq!(
        syntax_error("Tag(k1: v1, k2: v2, k1: v3)"),
        (1, 21, SyntaxErrorDetail::DuplicateOption("k1".to_string()))
    );
}

#[test]
fn test_empty_quote_key() {
    assert_eq!(
        syntax_error("Tag('': v)"),
        (1, 7, SyntaxErrorDetail::InvalidColon)
    );
}

#[test]
fn test_empty_quote_modifier_name() {
    assert_eq!(
        syntax_error("Tag.''(v)"),
        (1, 7, SyntaxErrorDetail::InvalidLeftParen)
    );
}

#[test]
fn test_newline_in_value() {
    assert_eq!(
        syntax_error("Tag(a: b\nc)"),
        (2, 1, SyntaxErrorDetail::InvalidToken("c".to_string()))
    );
}
