use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
#[logos(skip r"[ \t\n]+")]
pub enum Token {
    #[token("false", |_| false)]
    #[token("true", |_| true)]
    Bool(bool),

    #[token("MVI")]
    #[token("MOV")]
    #[token("ADD")]
    #[token("SUB")]
    OpCode,

    #[regex(r"[ABCDEHL]")]
    Register,

    #[regex(r"[0-9]+", |lex| lex.slice().parse::<u8>().unwrap())]
    Number(u8),
}

#[cfg(test)]
mod tests {
    use super::Token;
    use logos::Logos;

    #[test]
    fn first() {
        let mut lex = Token::lexer(
            r#"MVI A
               MOV B
            "#,
        );
        assert_eq!(lex.next(), Some(Ok(Token::OpCode)));
        assert_eq!(lex.slice(), "MVI");
        assert_eq!(lex.next(), Some(Ok(Token::Register)));
        assert_eq!(lex.slice(), "A");

        assert_eq!(lex.next(), Some(Ok(Token::OpCode)));
        assert_eq!(lex.slice(), "MOV");
        assert_eq!(lex.next(), Some(Ok(Token::Register)));
        assert_eq!(lex.slice(), "B");
    }
}

