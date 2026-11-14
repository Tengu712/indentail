use super::*;

#[cfg(test)]
mod test;

pub fn format(nodes: &[Node]) -> String {
    let mut out = String::new();
    format_nodes(nodes, false, &mut out);

    out
}

fn format_nodes(nodes: &[Node], force_arround: bool, out: &mut String) {
    let should_arround = force_arround || nodes.len() > 1;

    if should_arround {
        out.push('[');
    }

    for (i, node) in nodes.iter().enumerate() {
        if i > 0 {
            out.push(',');
        }
        format_node(node, out);
    }

    if should_arround {
        out.push(']');
    }
}

fn format_node(node: &Node, out: &mut String) {
    out.push_str("{\"tag\":");
    format_string(&node.tag, out);

    if !node.options.is_empty() {
        let mut options: Vec<(&String, &String)> = node.options.iter().collect();
        options.sort();

        out.push_str(",\"options\":{");

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
        out.push_str(",\"children\":");
        format_nodes(&node.children, true, out);
    }

    if !node.modifiers.is_empty() {
        out.push_str(",\"modifiers\":[");

        for (i, (n, v)) in node.modifiers.iter().enumerate() {
            if i > 0 {
                out.push(',');
            }
            out.push_str("{\"name\":");
            format_string(n, out);
            out.push_str(",\"value\":");
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
