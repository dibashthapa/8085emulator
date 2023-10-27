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
    #[token("ADC")]
    #[token("SUB")]
    #[token("INX")]
    #[token("XCHG")]
    #[token("LHLD")]
    #[token("SHLD")]
    #[token("HLT")]
    OpCode,

    #[regex(r"[ABCDEHL]")]
    Register,

    #[regex(r"[;:].*", logos::skip)]
    Comment,

    #[regex(r"[0-9A-F]{2}", |lex| u8::from_str_radix(lex.slice(), 16).unwrap())]
    Number(u8),

    #[regex(r"[0-9]{4}", |lex| u16::from_str_radix(lex.slice(), 16).unwrap())]
    Address(u16),
}