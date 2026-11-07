use super::*;

fn format(code: String, max_width: usize) -> String {
    let formatted: Vec<String> = parser::parse(code)
        .unwrap()
        .iter()
        .map(|v| v.format(max_width, 4))
        .collect();
    formatted.join("\n")
}

#[test]
fn test_flat() {
    let code = "Tag(b: 2, a: 1).m(v) // comment".to_string();
    let expected = "Tag(a: 1, b: 2).m(v)";

    assert_eq!(format(code, 20), expected);
}

#[test]
fn test_broken_options() {
    let code = "Tag(option1: value1, option2: value2) { Child }".to_string();
    let expected = "\
Tag(
    option1: value1,
    option2: value2,
) {
    Child
}";

    assert_eq!(format(code, 20), expected);
}

#[test]
fn test_broken_option() {
    let code = "Tag(option: valueeeeeeeeeeeeeeeeeeee)".to_string();
    let expected = "\
Tag(
    option:
        valueeeeeeeeeeeeeeeeeeee,
)";

    assert_eq!(format(code, 20), expected);
}

#[test]
fn test_quote() {
    let code = r"'Tag 1'(key: 'a b\n\'\\c')".to_string();
    let expected = "Tag 1(key: a b\n'\\c)";

    assert_eq!(format(code, 100), expected);
}

#[test]
fn test_long_token() {
    let code = "Tagggggggggggggggggggggg".to_string();
    let expected = "Tagggggggggggggggggggggg";

    assert_eq!(format(code, 20), expected);
}

#[test]
fn test_broken_modifier() {
    let code = "Tag.modifierrrrrrrrrrrrrrrrrrrrr(value)".to_string();
    let expected = "\
Tag
    .modifierrrrrrrrrrrrrrrrrrrrr(
        value
    )";

    assert_eq!(format(code, 20), expected);
}

#[test]
fn test_modifier_after_children() {
    let code = "Tag { Child }.m(v)".to_string();
    let expected = "\
Tag {
    Child
}
.m(v)";

    assert_eq!(format(code, 20), expected);
}

#[test]
fn test_overflowing_modifier() {
    let code = "Tag(aaaaaaaaaa: bbbbbbbbb).m(v)".to_string();
    let expected = "\
Tag(aaaaaaaaaa: bbbbbbbbb)
    .m(v)";

    assert_eq!(format(code, 30), expected);
}

#[test]
fn test_multiple_modifiers() {
    let code = "Tag.m(v).m(v)".to_string();
    let expected = "\
Tag
    .m(v)
    .m(v)";

    assert_eq!(format(code, 100), expected);
}

#[test]
fn test_nested() {
    let code = "A { B { C.m(v) } D }".to_string();
    let expected = "\
A {
    B {
        C.m(v)
    }
    D
}";

    assert_eq!(format(code, 100), expected);
}
