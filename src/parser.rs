fn push_str(tokens: &mut Vec<String>, str: &mut String) {
    if !str.is_empty() {
        tokens.push(str.clone());
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
    let mut within_quotes = false;
    let mut idx = 0;
    let mut sb = String::new();
    while idx < command.len() {
        match bytes[idx] {
            b'\'' => {
                within_quotes = !within_quotes;
            }
            b' ' => {
                if within_quotes {
                    sb.push(bytes[idx] as char);
                } else {
                    push_str(&mut tokens, &mut sb);
                }
            }
            _ => {
                sb.push(bytes[idx] as char);
            }
        }
        idx += 1;
    }
    if !sb.is_empty() {
        tokens.push(sb);
    }
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
