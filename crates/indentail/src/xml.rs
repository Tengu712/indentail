use super::*;

#[cfg(test)]
mod test;

#[derive(Debug, Clone)]
pub enum Mode {
    Structural,
    Aggressive,
    Semantic,
}

pub fn format(nodes: &[Node], mode: Mode) -> Result<String, IndentailError> {
    let mut formatter = Formatter {
        out: "<?xml version=\"1.1\"?><indentail>".to_string(),
        mode,
    };

    for node in nodes {
        formatter.node(node)?;
    }
    formatter.out.push_str("</indentail>");

    Ok(formatter.out)
}

struct Formatter {
    out: String,
    mode: Mode,
}

impl Formatter {
    fn node(&mut self, node: &Node) -> Result<(), IndentailError> {
        let mut attributes: Vec<(&str, &str)> = node
            .options
            .iter()
            .map(|(k, v)| (k.as_str(), v.as_str()))
            .collect();
        attributes.sort();

        match self.mode {
            Mode::Structural => {
                self.open(&node.tag, &attributes);

                if !node.children.is_empty() {
                    self.out.push_str("<children>");
                    for child in &node.children {
                        self.node(child)?;
                    }
                    self.out.push_str("</children>");
                }

                if !node.modifiers.is_empty() {
                    self.out.push_str("<modifiers>");
                    for (name, value) in &node.modifiers {
                        self.open(name, &[]);
                        self.text(value);
                        self.close(name);
                    }
                    self.out.push_str("</modifiers>");
                }

                self.close(&node.tag);
            }
            Mode::Aggressive => {
                for (name, value) in &node.modifiers {
                    if attributes.iter().any(|(k, _)| k == name) {
                        return Err(IndentailError::DuplicateAttribute(name.clone()));
                    }
                    attributes.push((name, value));
                }
                self.element(node, &attributes)?;
            }
            Mode::Semantic => {
                for (name, value) in node.modifiers.iter().rev() {
                    self.open(name, &[("value", value)]);
                }

                self.element(node, &attributes)?;

                for (name, _) in &node.modifiers {
                    self.close(name);
                }
            }
        }

        Ok(())
    }

    fn element(&mut self, node: &Node, attributes: &[(&str, &str)]) -> Result<(), IndentailError> {
        self.open(&node.tag, attributes);

        for child in &node.children {
            self.node(child)?;
        }

        self.close(&node.tag);
        Ok(())
    }

    fn open(&mut self, name: &str, attributes: &[(&str, &str)]) {
        self.out.push('<');
        self.name(name);

        for (k, v) in attributes {
            self.out.push(' ');
            self.name(k);
            self.out.push_str("=\"");
            self.text(v);
            self.out.push('"');
        }

        self.out.push('>');
    }

    fn close(&mut self, name: &str) {
        self.out.push_str("</");
        self.name(name);
        self.out.push('>');
    }

    fn name(&mut self, name: &str) {
        for (i, c) in name.char_indices() {
            let valid = match c {
                // leading `xml` is invalid
                // NOTE: Since it is case-insensitive, it needs to be written like this.
                _ if i == 0 && name.get(..3).is_some_and(|v| v.eq_ignore_ascii_case("xml")) => {
                    false
                }
                // `_x` is invalid
                '_' => !name[i + 1..].starts_with('x'),
                // leading digits and characters in certain ranges are invalid
                '-'
                | '.'
                | '0'..='9'
                | '\u{B7}'
                | '\u{300}'..='\u{36F}'
                | '\u{203F}'..='\u{2040}' => i > 0,
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
                self.out.push(c);
            } else {
                self.out.push_str(&format!("_x{:X}_", c as u32));
            }
        }
    }

    fn text(&mut self, text: &str) {
        for c in text.chars() {
            if c.is_control() || matches!(c, '<' | '>' | '&' | '"' | '\u{2028}') {
                self.out.push_str(&format!("&#x{:X};", c as u32));
            } else {
                self.out.push(c);
            }
        }
    }
}
