use crate::unescape;

#[test]
fn test_unescaped() {
    let input = "A character entity reference refers to the content of a named entity.";
    let result = "A character entity reference refers to the content of a named entity.";
    assert_eq!(unescape(&input).unwrap(), result);
}

#[test]
fn test_named() {
    let input = "\
        To use one of these character entity references in an HTML or XML document, enter an \
        ampersand followed by the entity name and a semicolon, e.g., enter &copy; for the \
        copyright symbol (©).";
    let result = "\
        To use one of these character entity references in an HTML or XML document, enter an \
        ampersand followed by the entity name and a semicolon, e.g., enter © for the copyright \
        symbol (©).";
    assert_eq!(unescape(&input).unwrap(), result);
}

#[test]
fn test_decimal() {
    let input = "For example, to display the copyright symbol ©, enter &#0169;";
    let result = "For example, to display the copyright symbol ©, enter ©";
    assert_eq!(unescape(&input).unwrap(), result);
}

#[test]
fn test_hex() {
    let input = "For example, to display the copyright symbol © enter &#x00A9; or &#xA9;.";
    let result = "For example, to display the copyright symbol © enter © or ©.";
    assert_eq!(unescape(&input).unwrap(), result);
}

#[test]
fn test_invalid_char_ref() {
    // invalid char reference
    assert_eq!(
        unescape("&invalidreference;").unwrap(),
        "&invalidreference;"
    );
}

#[test]
fn test_invalid_utf8_out_of_range() {
    // utf-8 code out of range
    assert_eq!(unescape("&#x110000;").unwrap(), "&#x110000;");
}
#[test]
fn test_invalid_broken_ref() {
    // missed trailing semicolon
    assert_eq!(unescape("&#x192; &#x192").unwrap(), "ƒ &#x192");
}
