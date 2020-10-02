Character Reference Unescape
============================
This library is for unescaping character references in HTML and XML. Works on both numeric character references and character entity references.  
## Usage:
```rust
use cr_unescape::EscapeCharacters;

fn main() {
    let input = "\
        In SGML, HTML and XML documents, the logical constructs known as \
        character data and attribute values consist of sequences of \
        characters, in which each character can manifest directly \
        (representing itself), or can be represented by a series of \
        characters called a character reference, of which there are two \
        types: a numeric character reference and a character entity \
        reference.
        Numeric character reference:
        &#nnnn; or &#xhhhh;
        Example:
        &#177; &#x192;
        Character entity reference:
        &name;
        &reg;";
    let result = "\
        In SGML, HTML and XML documents, the logical constructs known as \
        character data and attribute values consist of sequences of \
        characters, in which each character can manifest directly \
        (representing itself), or can be represented by a series of \
        characters called a character reference, of which there are two \
        types: a numeric character reference and a character entity \
        reference.
        Numeric character reference:
        &#nnnn; or &#xhhhh;
        Example:
        ± ƒ
        Character entity reference:
        &name;
        ®";
    assert_eq!(input.unescape().unwrap(), result);
}
```