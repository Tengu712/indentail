use super::*;

fn json(code: &str, mode: Mode) -> Result<String, TailortailError> {
    format(&parser::parse(code.to_string()).unwrap(), mode)
}

#[test]
fn test_single() {
    let code = "Tag(b: 2, a: 1) { C }.m1(v1).m2(v2).m1(v3)";
    let expected = r#"{"tag":"Tag","options":{"a":"1","b":"2"},"children":[{"tag":"C"}],"modifiers":[{"name":"m1","value":"v1"},{"name":"m2","value":"v2"},{"name":"m1","value":"v3"}]}"#;

    assert_eq!(json(code, Mode::Single).unwrap(), expected);
}

#[test]
fn test_single_empty() {
    assert_eq!(json("", Mode::Single).unwrap(), "{}");
}

#[test]
fn test_single_multiple() {
    assert!(matches!(
        json("A B", Mode::Single),
        Err(TailortailError::MultipleTopLevelNodes)
    ));
}

#[test]
fn test_multiple() {
    let code = "A { B } C D";
    let expected = r#"[{"tag":"A","children":[{"tag":"B"}]},{"tag":"C"},{"tag":"D"}]"#;

    assert_eq!(json(code, Mode::Multiple).unwrap(), expected);
}

#[test]
fn test_multiple_single() {
    assert_eq!(json("A", Mode::Multiple).unwrap(), r#"[{"tag":"A"}]"#);
}

#[test]
fn test_multiple_empty() {
    assert_eq!(json("", Mode::Multiple).unwrap(), "[]");
}

#[test]
fn test_string() {
    let code = concat!(r#"'"\\'(k: '\t\r\n"#, "\u{1}\u{1F}\u{7F}<>&😄", "')");
    let expected = r#"{"tag":"\"\\","options":{"k":"\u0009\u000D\u000A\u0001\u001F"#.to_string()
        + "\u{7F}<>&😄\"}}";

    assert_eq!(json(code, Mode::Single).unwrap(), expected);
}
