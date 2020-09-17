mod entities;

use core::convert::TryFrom;
use std::collections::BTreeMap;

#[cfg(test)]
mod tests;

enum Parse {
    NonEscaped,
    Escaped(usize),
    Named(usize),
    Numeric(usize),
    Decimal(usize),
    Hex(usize),
}

/// Leaves text as is:
/// ```
/// let input = String::from("A character entity reference refers to the content of a named entity.");
/// let result = String::from("A character entity reference refers to the content of a named entity.");
/// assert_eq!(cr_unescape::unescape(&input), result);
/// ```
///
/// Converts named, decimal and hex escaped characters:
/// ```
/// let input = String::from("&reg; &#177; &#x192;");
/// let result = String::from("® ± ƒ");
/// assert_eq!(cr_unescape::unescape(&input), result);
/// ```
///
/// Leaves broken/invalid escaped characters as is:
/// ```
/// let input = String::from("&r; &#; &#x19");
/// let result = String::from("&r; &#; &#x19");
/// assert_eq!(cr_unescape::unescape(&input), result);
/// ```
pub fn unescape(text: &String) -> String {
    let characters: BTreeMap<String, String> = entities::load().unwrap();
    let mut result_buffer = String::with_capacity(text.len());

    let mut step = Parse::NonEscaped;

    for (i, symbol) in text.char_indices() {
        match step {
            Parse::NonEscaped => {
                if symbol == '&' {
                    step = Parse::Escaped(i)
                } else {
                    result_buffer.push(symbol)
                }
            }
            Parse::Escaped(escape_pos) => match symbol {
                'a'..='z' | 'A'..='Z' => {
                    step = Parse::Named(escape_pos);
                }
                '#' => {
                    step = Parse::Numeric(escape_pos);
                }
                '&' => {
                    result_buffer.push_str(&text[escape_pos..i]);
                    step = Parse::Escaped(i);
                }
                _ => {
                    result_buffer.push_str(&text[escape_pos..i]);
                    step = Parse::NonEscaped;
                }
            },
            Parse::Named(escape_pos) => match symbol {
                '&' => {
                    result_buffer.push_str(&text[escape_pos..i]);
                    step = Parse::Escaped(i);
                }
                ';' => {
                    if i > escape_pos + 2 {
                        let char_reference = &text[(escape_pos + 1)..i];
                        match characters.get(char_reference) {
                            Some(character) => result_buffer.push_str(character),
                            None => result_buffer.push_str(&text[escape_pos..=i]),
                        }
                    } else {
                        result_buffer.push_str(&text[escape_pos..=i]);
                    }
                    step = Parse::NonEscaped;
                }
                'a'..='z' | 'A'..='Z' | '0'..='9' => (),
                _ => {
                    result_buffer.push_str(&text[escape_pos..=i]);
                    step = Parse::NonEscaped;
                }
            },
            Parse::Numeric(escape_pos) => match symbol {
                '&' => {
                    result_buffer.push_str(&text[escape_pos..i]);
                    step = Parse::Escaped(i);
                }
                '0'..='9' => step = Parse::Decimal(escape_pos),
                'x' => step = Parse::Hex(escape_pos),
                _ => {
                    result_buffer.push_str(&text[escape_pos..=i]);
                    step = Parse::NonEscaped;
                }
            },
            Parse::Decimal(escape_pos) => match symbol {
                '&' => {
                    result_buffer.push_str(&text[escape_pos..i]);
                    step = Parse::Escaped(i);
                }
                '0'..='9' => (),
                ';' => {
                    if i >= escape_pos + 3 {
                        let char_reference = &text[(escape_pos + 2)..i];
                        match u32::from_str_radix(char_reference, 10) {
                            Ok(code) => match char::try_from(code) {
                                Ok(character) => result_buffer.push_str(&character.to_string()),
                                _error => result_buffer.push_str(&text[escape_pos..=i]),
                            },
                            _error => result_buffer.push_str(&text[escape_pos..=i]),
                        }
                    } else {
                        result_buffer.push_str(&text[escape_pos..=i]);
                    }
                    step = Parse::NonEscaped;
                }
                _ => {
                    result_buffer.push_str(&text[escape_pos..=i]);
                    step = Parse::NonEscaped;
                }
            },
            Parse::Hex(escape_pos) => match symbol {
                '&' => {
                    result_buffer.push_str(&text[escape_pos..i]);
                    step = Parse::Escaped(i);
                }
                '0'..='9' | 'a'..='f' | 'A'..='F' => (),
                ';' => {
                    if i >= escape_pos + 3 {
                        let char_reference = &text[(escape_pos + 3)..i];
                        match u32::from_str_radix(char_reference, 16) {
                            Ok(code) => match char::try_from(code) {
                                Ok(character) => result_buffer.push_str(&character.to_string()),
                                _error => result_buffer.push_str(&text[escape_pos..=i]),
                            },
                            _error => result_buffer.push_str(&text[escape_pos..=i]),
                        }
                    } else {
                        result_buffer.push_str(&text[escape_pos..=i]);
                    }
                    step = Parse::NonEscaped;
                }
                _ => {
                    result_buffer.push_str(&text[escape_pos..=i]);
                    step = Parse::NonEscaped;
                }
            },
        }
    }
    match step {
        Parse::Escaped(escape_pos) => result_buffer.push_str(&text[escape_pos..]),
        Parse::Named(escape_pos) => result_buffer.push_str(&text[escape_pos..]),
        Parse::Numeric(escape_pos) => result_buffer.push_str(&text[escape_pos..]),
        Parse::Decimal(escape_pos) => result_buffer.push_str(&text[escape_pos..]),
        Parse::Hex(escape_pos) => result_buffer.push_str(&text[escape_pos..]),
        Parse::NonEscaped => (),
    };
    result_buffer
}
