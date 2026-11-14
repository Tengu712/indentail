use super::*;

#[cfg(test)]
mod test;

#[derive(Debug, Clone, Copy)]
pub enum Mode {
    Single,
    Multiple,
}

pub fn format(nodes: &[Node], mode: Mode) -> Result<String, TailortailError> {
    let mut out = String::new();

    match (mode, nodes) {
        (Mode::Single, []) => out.push_str("{}"),
        (Mode::Single, [node]) => format_node(node, &mut out),
        (Mode::Single, _) => return Err(TailortailError::MultipleTopLevelNodes),
        (Mode::Multiple, _) => format_nodes(nodes, &mut out),
    }

    Ok(out)
}

fn format_nodes(nodes: &[Node], out: &mut String) {
    out.push('[');

    for (i, node) in nodes.iter().enumerate() {
        if i > 0 {
            out.push(',');
        }
        format_node(node, out);
    }

    out.push(']');
}

fn format_node(node: &Node, out: &mut String) {
    out.push_str(r#"{"tag":"#);
    format_string(&node.tag, out);

    if !node.options.is_empty() {
        let mut options: Vec<(&String, &String)> = node.options.iter().collect();
        options.sort();

        out.push_str(r#","options":{"#);

        for (i, (k, v)) in options.into_iter().enumerate() {
            if i > 0 {
                out.push(',');
            }
            format_string(k, out);
            out.push(':');
            format_string(v, out);
        }

        out.push('}');
    }

    if !node.children.is_empty() {
        out.push_str(r#","children":"#);
        format_nodes(&node.children, out);
    }

    if !node.modifiers.is_empty() {
        out.push_str(r#","modifiers":["#);

        for (i, (n, v)) in node.modifiers.iter().enumerate() {
            if i > 0 {
                out.push(',');
            }
            out.push_str(r#"{"name":"#);
            format_string(n, out);
            out.push_str(r#","value":"#);
            format_string(v, out);
            out.push('}');
        }

        out.push(']');
    }

    out.push('}');
}

fn format_string(text: &str, out: &mut String) {
    out.push('"');

    for c in text.chars() {
        match c {
            '"' | '\\' => {
                out.push('\\');
                out.push(c);
            }
            '\u{0}'..='\u{1F}' => out.push_str(&format!("\\u{:04X}", c as u32)),
            _ => out.push(c),
        }
    }

    out.push('"');
}
