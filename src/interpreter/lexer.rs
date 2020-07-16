#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Token {
    Number(f64),
    Punctuator(char),
}

#[derive(Debug)]
pub enum LexerError {
    IllegalCharacter(char),
}

pub fn lex(code: String) -> Result<Vec<Token>, LexerError> {
    let mut tokens = Vec::<Token>::new();
    let mut code = code.chars().peekable();

    while let Some(&ch) = code.peek() {
        match ch {
            ch if ch.is_ascii_digit() => {
                let mut number = String::new();

                while let Some(&ch) = code.peek() {
                    if ch.is_ascii_digit() || ch == '.' && !number.contains('.') {
                        number.push(code.next().unwrap());
                    } else {
                        break;
                    }
                }

                let number: f64 = number.parse().unwrap();
                tokens.push(Token::Number(number));
            }
            '+' | '-' | '*' | '/' | '%' | '^' | '(' | ')' => {
                let ch = code.next().unwrap();
                tokens.push(Token::Punctuator(ch));
            }
            ch => {
                if !ch.is_ascii_whitespace() {
                    return Err(LexerError::IllegalCharacter(ch));
                }
                code.next();
            }
        }
    }

    Ok(tokens)
}
