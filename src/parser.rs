const QUOTE_CHARS: [char; 2] = ['\'', '\"'];

fn push_str(tokens: &mut Vec<String>, buf: &mut String) {
    if !buf.is_empty() {
        let tok = buf
            .trim_start_matches(|c| QUOTE_CHARS.contains(&c))
            .trim_end_matches(|c| QUOTE_CHARS.contains(&c));
        tokens.push(tok.to_string());
        buf.clear();
    }
}

/**
 * collect all strings, tokenize
 * if between quotes, don't break
 * otherwise break on whitespaces
 * ie. if within_quotes, continue collecting
 * when encountering close quote, push to tokens
 */
pub fn parse_command(command: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut sb = String::new();
    let mut bracket_stack: Vec<char> = Vec::new();
    for c in command.chars() {
        match c {
            ch if QUOTE_CHARS.contains(&ch) => {
                if bracket_stack.len() > 0 {
                    if c == *bracket_stack.last().unwrap() {
                        bracket_stack.pop();
                    } else {
                        sb.push(c);
                    }
                } else {
                    bracket_stack.push(c);
                }
            }
            ' ' => {
                if bracket_stack.len() > 0 {
                    sb.push(c);
                } else {
                    if sb.len() > 0 {
                        push_str(&mut tokens, &mut sb);
                    }
                }
            }
            _ => {
                sb.push(c);
            }
        }
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
        let tokens = parse_command(command);
        assert_eq!(tokens, vec!["echo", "world     test", "example", "hello"]);
    }
}
