const QUOTE_CHARS: [u8; 2] = [b'\'', b'\"'];

fn push_str(tokens: &mut Vec<String>, str: &mut String) {
    if !str.is_empty() {
        let tok = str //.trim_matches(|c| QUOTE_CHARS.contains(&c));
            .trim_start_matches(|c| c == '\'' || c == '"')
            .trim_end_matches(|c| c == '\'' || c == '"');
        tokens.push(tok.to_string());
        str.clear();
    }
}

/**
 * collect all strings, tokenize
 * if between quotes, don't break
 * otherwise break on whitespaces
 * ie. if within_quotes, continue collecting
 * when encountering close quote, push to tokens
 */
pub fn parse_command(command: String) -> Vec<String> {
    let mut tokens = Vec::new();
    let bytes = command.as_bytes();
    let mut idx = 0;
    let mut sb = String::new();
    let mut bracket_stack: Vec<u8> = Vec::new();
    while idx < command.len() {
        match bytes[idx] {
            c if c == QUOTE_CHARS[0] || c == QUOTE_CHARS[1] => {
                if bracket_stack.len() > 0 {
                    if c == *bracket_stack.last().unwrap() {
                        bracket_stack.pop();
                    } else {
                        sb.push(c as char);
                    }
                } else {
                    bracket_stack.push(c);
                }
            }
            b' ' => {
                // println!("space encountered, bracket_stack: {:?}", bracket_stack);
                if bracket_stack.len() > 0 {
                    // println!("bracket_stack: {:?}", bracket_stack);
                    sb.push(bytes[idx] as char);
                } else {
                    if sb.len() > 0 {
                        push_str(&mut tokens, &mut sb);
                    }
                }
            }
            _ => {
                sb.push(bytes[idx] as char);
            }
        }
        idx += 1;
    }
    push_str(&mut tokens, &mut sb);
    tokens
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_command() {
        let command = "echo 'world     test' 'example''hello'";
        let tokens = parse_command(command.to_string());
        assert_eq!(tokens, vec!["echo", "world     test", "example", "hello"]);
    }
}
