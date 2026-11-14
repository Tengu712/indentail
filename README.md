# Tailortail

## About

Tailortail is a data definition language inspired by SwiftUI.

```
Enemy(name: boss, hp: 500) {
    Sprite(image: boss.png)
    ForEach(index: i, count: 8) {
        Sprite(image: bit.png)
            .translateX(64)
            .rotate(i * 45)
    }
}
.scale(2)
.translateX(320)
.translateY(240)
```

## Grammar

```ebnf
document = { node } ;
node     = token , [ options ] , [ children ] , { modifier } ;
options  = "(" , [ option , { "," , option } , [ "," ] ] , ")" ;
option   = token , ":" , value ;
children = "{" , { node } , "}" ;
modifier = "." , token , "(" , value , ")" ;
token    = word | quoted ;
value    = word , { { " " | "\t" } , word } | quoted ;
```

- Tokens and symbols can be separated by whitespace.
- `word` is a sequence of characters other than whitespace, `(`, `)`, `{`, `}` and `'`.
  - Outside parentheses, `.` cannot be used.
  - Inside parentheses, `:` and `,` cannot be used.
- `quoted` is zero or more characters enclosed in `'`.
  - `\\`, `\t`, `\r`, `\n` and `\'` are escape sequences.
  - An empty `quoted` is ignored when used as a tag, and causes an error when used as an option key or a modifier name.
- `//` outside `quoted` starts a comment that continues to the end of the line.
- Option keys must be unique within a node.

## Format

### Rust

```rs
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Node {
    pub tag: String,
    pub options: HashMap<String, String>,
    pub modifiers: Vec<(String, String)>,
    pub children: Vec<Node>,
}
```

### JSON

```jsonc
/* single mode */
{
  "tag": "TAG",
  "options": { "KEY": "VALUE", /* ... */ },
  "children": [ /* NODE ... */ ],
  "modifiers": [ { "name": "NAME", "value": "VALUE" }, /* ... */ ]
}

/* multiple mode */
[ /* NODE ... */ ]
```

- In `single` mode:
  - `{}` is output if there are no nodes.
  - an error is returned if there are multiple nodes.
- In both modes, `options`, `children` and `modifiers` are omitted if empty.

### XML

```xml
<!-- structural mode -->
<?xml version="1.1"?>
<tailortail>
  <TAG KEY="VALUE" ...>
    <children>
      NODE
      ...
    </children>
    <modifiers>
      <NAME>VALUE</NAME>
      ...
    </modifiers>
  </TAG>
  ...
</tailortail>

<!-- aggressive mode -->
<?xml version="1.1"?>
<tailortail>
  <TAG KEY="VALUE" ... NAME="VALUE" ...>
    NODE
    ...
  </TAG>
  ...
</tailortail>

<!-- semantic mode -->
<?xml version="1.1"?>
<tailortail>
  <NAME value="VALUE">
    ...
      <TAG KEY="VALUE" ...>
        NODE
        ...
      </TAG>
    ...
  </NAME>
  ...
</tailortail>
```

- In `aggressive` mode, an error is returned if a modifier name duplicates an option key or another modifier name.
- In `semantic` mode, the last modifier is the outermost.
- In all modes, `<children>` and `<modifiers>` are omitted if empty.
