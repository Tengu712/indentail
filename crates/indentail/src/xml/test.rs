use super::*;

fn xml(code: &str, mode: Mode) -> Result<String, IndentailError> {
    format(&parser::parse(code.to_string()).unwrap(), mode)
}

fn wrap(body: &str) -> String {
    format!(r#"<?xml version="1.1"?><indentail>{body}</indentail>"#)
}

#[test]
fn test_structural() {
    let code = "Tag(b: 2, a: 1) { C }.m1(v1).m2(v2) D";
    let expected = wrap(
        r#"<Tag a="1" b="2"><children><C></C></children><modifiers><m1>v1</m1><m2>v2</m2></modifiers></Tag><D></D>"#,
    );

    assert_eq!(xml(code, Mode::Structural).unwrap(), expected);
}

#[test]
fn test_aggressive() {
    let code = "Tag(k: v) { C }.m1(v1).m2(v2)";
    let expected = wrap(r#"<Tag k="v" m1="v1" m2="v2"><C></C></Tag>"#);

    assert_eq!(xml(code, Mode::Aggressive).unwrap(), expected);
}

#[test]
fn test_aggressive_duplicate() {
    assert!(matches!(
        xml("Tag(m: v1).m(v2)", Mode::Aggressive),
        Err(IndentailError::DuplicateAttribute(v)) if v == "m"
    ));
    assert!(matches!(
        xml("Tag.m(v1).m(v2)", Mode::Aggressive),
        Err(IndentailError::DuplicateAttribute(v)) if v == "m"
    ));
}

#[test]
fn test_semantic() {
    let code = "Tag(k: v) { C }.m1(v1).m2(v2)";
    let expected = wrap(r#"<m2 value="v2"><m1 value="v1"><Tag k="v"><C></C></Tag></m1></m2>"#);

    assert_eq!(xml(code, Mode::Semantic).unwrap(), expected);
}

#[test]
fn test_name() {
    let code = "'Tag 1'('1a': v, 'a:b': v, '_x20_': v, '-.': v) xmlns XmL x_y 😄";
    let expected = wrap(
        r#"<Tag_x20_1 _x2D_.="v" _x31_a="v" _x5F_x20_="v" a_x3A_b="v"></Tag_x20_1><_x78_mlns></_x78_mlns><_x58_mL></_x58_mL><x_y></x_y><😄></😄>"#,
    );

    assert_eq!(xml(code, Mode::Structural).unwrap(), expected);
}

#[test]
fn test_text() {
    let code = concat!(
        r#"Tag(k: '<>&"\t\r\n"#,
        "\u{1}\u{85}\u{2028}",
        r#"\'').m('<>&')"#
    );
    let expected = wrap(
        r#"<Tag k="&#x3C;&#x3E;&#x26;&#x22;&#x9;&#xD;&#xA;&#x1;&#x85;&#x2028;'"><modifiers><m>&#x3C;&#x3E;&#x26;</m></modifiers></Tag>"#,
    );

    assert_eq!(xml(code, Mode::Structural).unwrap(), expected);
}
