use super::cpu::Registers;
use logos::Logos;

#[derive(Logos, Debug, PartialEq, Clone, Copy)]
#[logos(skip r"[ \t\n,]+")]
pub enum Token<'a> {
    #[token("false", |_| false)]
    #[token("true", |_| true)]
    Bool(bool),

    #[regex(r"[a-zA-Z]{1,8}", |lex| lex.slice())]
    Word(&'a str),

    #[regex(r"[ABCDEHL]", |lex|  Registers::from(lex.slice()),priority=2)]
    Register(Registers),

    #[regex(r"[;].*", logos::skip)]
    Comment,

    #[regex(r"[a-zA-z]+:", |lex| lex.slice().trim_end_matches(':'))]
    Label(&'a str),

    #[regex(r"[0-9A-F]{2}", |lex| u8::from_str_radix(lex.slice(), 16).unwrap())]
    Number(u8),

    #[regex(r"[0-9]{4}", |lex| u16::from_str_radix(lex.slice(), 16).unwrap())]
    Address(u16),
}
