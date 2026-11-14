use super::*;

#[cfg(test)]
mod test;

#[derive(Debug, Clone, Copy)]
pub enum Mode {
    Structural,
    Aggressive,
    Semantic,
}

pub fn format(nodes: &[Node], mode: Mode) -> Result<String, TailortailError> {
    let mut out = r#"<?xml version="1.1"?><tailortail>"#.to_string();
    for node in nodes {
        format_node(node, mode, &mut out)?;
    }
    out.push_str("</tailortail>");

    Ok(out)
}

fn format_node(node: &Node, mode: Mode, out: &mut String) -> Result<(), TailortailError> {
    let mut attributes: Vec<(&str, &str)> = node
        .options
        .iter()
        .map(|(k, v)| (k.as_str(), v.as_str()))
        .collect();
    attributes.sort();

    match mode {
        Mode::Structural => {
            format_open(&node.tag, &attributes, out);

            if !node.children.is_empty() {
                out.push_str("<children>");
                for child in &node.children {
                    format_node(child, mode, out)?;
                }
                out.push_str("</children>");
            }

            if !node.modifiers.is_empty() {
                out.push_str("<modifiers>");
                for (name, value) in &node.modifiers {
                    format_open(name, &[], out);
                    format_text(value, out);
                    format_close(name, out);
                }
                out.push_str("</modifiers>");
            }

            format_close(&node.tag, out);
        }
        Mode::Aggressive => {
            for (name, value) in &node.modifiers {
                if attributes.iter().any(|(k, _)| k == name) {
                    return Err(TailortailError::DuplicateAttribute(name.clone()));
                }
                attributes.push((name, value));
            }

            format_element(node, &attributes, mode, out)?;
        }
        Mode::Semantic => {
            for (name, value) in node.modifiers.iter().rev() {
                format_open(name, &[("value", value)], out);
            }

            format_element(node, &attributes, mode, out)?;

            for (name, _) in &node.modifiers {
                format_close(name, out);
            }
        }
    }

    Ok(())
}

fn format_element(
    node: &Node,
    attributes: &[(&str, &str)],
    mode: Mode,
    out: &mut String,
) -> Result<(), TailortailError> {
    format_open(&node.tag, attributes, out);

    for child in &node.children {
        format_node(child, mode, out)?;
    }

    format_close(&node.tag, out);
    Ok(())
}

fn format_open(name: &str, attributes: &[(&str, &str)], out: &mut String) {
    out.push('<');
    format_name(name, out);

    for (k, v) in attributes {
        out.push(' ');
        format_name(k, out);
        out.push_str("=\"");
        format_text(v, out);
        out.push('"');
    }

    out.push('>');
}

fn format_close(name: &str, out: &mut String) {
    out.push_str("</");
    format_name(name, out);
    out.push('>');
}

fn format_name(name: &str, out: &mut String) {
    for (i, c) in name.char_indices() {
        let valid = match c {
            // leading `xml` is invalid
            // NOTE: Since it is case-insensitive, it needs to be written like this.
            _ if i == 0 && name.get(..3).is_some_and(|v| v.eq_ignore_ascii_case("xml")) => false,
            // `_x` is invalid
            '_' => !name[i + 1..].starts_with('x'),
            // leading digits and characters in certain ranges are invalid
            '-' | '.' | '0'..='9' | '\u{B7}' | '\u{300}'..='\u{36F}' | '\u{203F}'..='\u{2040}' => {
                i > 0
            }
            // valid characters
            'A'..='Z'
            | 'a'..='z'
            | '\u{C0}'..='\u{D6}'
            | '\u{D8}'..='\u{F6}'
            | '\u{F8}'..='\u{2FF}'
            | '\u{370}'..='\u{37D}'
            | '\u{37F}'..='\u{1FFF}'
            | '\u{200C}'..='\u{200D}'
            | '\u{2070}'..='\u{218F}'
            | '\u{2C00}'..='\u{2FEF}'
            | '\u{3001}'..='\u{D7FF}'
            | '\u{F900}'..='\u{FDCF}'
            | '\u{FDF0}'..='\u{FFFD}'
            | '\u{10000}'..='\u{EFFFF}' => true,
            // others are invalid
            _ => false,
        };
        if valid {
            out.push(c);
        } else {
            out.push_str(&format!("_x{:X}_", c as u32));
        }
    }
}

fn format_text(text: &str, out: &mut String) {
    for c in text.chars() {
        if c.is_control() || matches!(c, '<' | '>' | '&' | '"' | '\u{2028}') {
            out.push_str(&format!("&#x{:X};", c as u32));
        } else {
            out.push(c);
        }
    }
}
