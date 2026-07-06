pub fn parse_input(line: &str) -> Option<crate::types::command::ParsedCommand> {
    let mut tokens = Vec::new();
    let mut current_token = String::new();
    let mut in_double_quotes = false;
    let mut in_single_quotes = false;
    let mut escaped = false;
    let mut has_added_token = false;
    for c in line.chars() {
        if escaped {
            current_token.push(c);
            escaped = false;
            has_added_token = true;
        } else if c == '\\' && !in_single_quotes {
            escaped = true;
        } else if c == '"' && !in_single_quotes {
            in_double_quotes = !in_double_quotes;
            has_added_token = true;
        } else if c == '\'' && !in_double_quotes {
            in_single_quotes = !in_single_quotes;
            has_added_token = true;
        } else if c.is_whitespace() && !in_double_quotes && !in_single_quotes {
            if !current_token.is_empty() || has_added_token {
                tokens.push(current_token.clone());
                current_token.clear();
                has_added_token = false;
            }
        } else {
            current_token.push(c);
        }
    }
    if !current_token.is_empty() || has_added_token {
        tokens.push(current_token);
    }
    if tokens.is_empty() {
        return None;
    }
    let name = tokens.remove(0);
    Some(crate::types::command::ParsedCommand { name, args: tokens })
}
