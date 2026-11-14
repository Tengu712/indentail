use super::*;
use std::iter;

#[cfg(test)]
mod test;

impl Node {
    pub fn format(&self, max_width: usize, indent_spaces: usize) -> String {
        let mut formatter = Formatter {
            out: Default::default(),
            max_width,
            indent_spaces,
        };
        formatter.node(self, 0);
        formatter.out
    }
}

struct Formatter {
    out: String,
    max_width: usize,
    indent_spaces: usize,
}

impl Formatter {
    fn node(&mut self, node: &Node, indent: usize) {
        // push tag
        let tag = [node.tag.as_str()];
        self.push(&tag);

        // option list
        let mut options: Vec<[&str; 4]> = node
            .options
            .iter()
            .map(|(k, v)| [k.as_str(), ":", " ", v])
            .collect();
        options.sort();
        let mut flat_options = vec![];
        if !options.is_empty() {
            flat_options.push("(");
            flat_options.extend(options.join(&", "));
            flat_options.push(")");
        }

        // modifiers
        let modifiers: Vec<[&str; 5]> = node
            .modifiers
            .iter()
            .map(|(k, v)| [".", k, "(", v, ")"])
            .collect();

        // push oneline
        // .modifier(value)
        // (options).modifier(value)
        if let [modifier] = modifiers.as_slice()
            && node.children.is_empty()
            && self.fits(indent, &[&tag[..], &flat_options, modifier].concat())
        {
            self.push(&flat_options);
            self.push(modifier);
            return;
        }

        // children list left brace
        let opener = if node.children.is_empty() { "" } else { " {" };

        // push head
        //
        if flat_options.is_empty() {
        }
        // (options)
        else if self.fits(indent, &[&tag[..], &flat_options, &[opener]].concat()) {
            self.push(&flat_options);
        }
        // (
        //   options,
        // )
        else {
            self.push(&["("]);
            let option_indent = indent + self.indent_spaces;
            for option in &options {
                self.newline(option_indent);
                if self.fits(option_indent, &[&option[..], &[","]].concat()) {
                    self.push(option);
                    self.push(&[","]);
                } else {
                    let [k, colon, _, v] = option;
                    self.push(&[k, colon]);
                    self.newline(option_indent + self.indent_spaces);
                    self.push(&[v, ","]);
                }
            }
            self.newline(indent);
            self.push(&[")"]);
        }

        // push children
        // {
        //   children
        // }
        if !node.children.is_empty() {
            self.push(&[opener]);
            for child in &node.children {
                self.newline(indent + self.indent_spaces);
                self.node(child, indent + self.indent_spaces);
            }
            self.newline(indent);
            self.push(&["}"]);
        }

        // push modifiers
        let modifier_indent = if node.children.is_empty() {
            indent + self.indent_spaces
        } else {
            indent
        };
        for modifier in &modifiers {
            self.newline(modifier_indent);
            // .modifier(value)
            if self.fits(modifier_indent, modifier) {
                self.push(modifier);
            }
            // .modifier(
            //   value
            // )
            else {
                let [dot, name, open, value, close] = modifier;
                self.push(&[dot, name, open]);
                self.newline(modifier_indent + self.indent_spaces);
                self.push(&[value]);
                self.newline(modifier_indent);
                self.push(&[close]);
            }
        }
    }

    fn fits(&self, indent: usize, pieces: &[&str]) -> bool {
        indent + pieces.iter().map(|v| v.chars().count()).sum::<usize>() <= self.max_width
    }

    fn push(&mut self, pieces: &[&str]) {
        self.out.extend(pieces.iter().copied());
    }

    fn newline(&mut self, indent: usize) {
        self.out.push('\n');
        self.out.extend(iter::repeat_n(' ', indent));
    }
}
