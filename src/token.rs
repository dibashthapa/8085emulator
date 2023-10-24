use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
#[logos(skip r"[ \t\n,]+")]
pub enum Token {
    #[token("false", |_| false)]
    #[token("true", |_| true)]
    Bool(bool),

    #[token("MVI")]
    #[token("MOV")]
    #[token("INR")]
    #[token("DCR")]
    #[token("LDA")]
    #[token("STA")]
    #[token("LXI")]
    #[token("ADD")]
    #[token("ADI")]
    #[token("SUB")]
    #[token("INX")]
    #[token("HLT")]
    OpCode,

    #[regex(r"[ABCDEHL]")]
    Register,

    #[regex(r";.*", logos::skip)]
    Comment,

    #[regex(r"[0-9]{2}", |lex| lex.slice().parse::<u8>().unwrap())]
    Number(u8),

    #[regex(r"[0-9]{4}", |lex| lex.slice().parse::<u16>().unwrap())]
    Address(u16),
}