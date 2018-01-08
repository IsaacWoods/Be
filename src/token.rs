/*
 * Copyright (C) 2017, Isaac Woods.
 * See LICENCE.md
 */

use cautious_iter::CautiousIterator;
use std::iter::Peekable;
use std::str::Chars;

#[derive(Debug)]
pub enum TokenKind
{
    DollarSign,

    Identifier(String),

    NewLine,
}

#[derive(Debug)]
pub struct Token
{
    kind        : TokenKind,
    offset      : usize,
    line        : usize,
    line_offset : usize,
}

pub struct TokenStream<'a>
{
    it                  : Peekable<Chars<'a>>,
    current_offset      : usize,
    current_line        : usize,
    current_line_offset : usize,
}

impl<'a> TokenStream<'a>
{
    pub fn new(source : &'a str) -> TokenStream
    {
        TokenStream
        {
            it                  : source.chars().peekable(),
            current_offset      : 0,
            current_line        : 0,
            current_line_offset : 0,
        }
    }

    fn make_token(&self, kind : TokenKind) -> Token
    {
        Token
        {
            kind        : kind,
            offset      : self.current_offset,
            line        : self.current_line,
            line_offset : self.current_line_offset
        }
    }
}

impl<'a> Iterator for TokenStream<'a>
{
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item>
    {
        let c = match self.it.next()
                {
                    Some(c)   => c,
                    None      => return None,
                };

        self.current_offset += 1;
        self.current_line_offset += 1;

        match c
        {
            '$' => Some(self.make_token(TokenKind::DollarSign)),

            '\n' =>
            {
                let token = self.make_token(TokenKind::NewLine);
                self.current_line += 1;
                self.current_line_offset = 0;
                Some(token)
            },

            'a'...'z' | 'A'...'Z' =>
            {
                let mut string = String::new();
                string.push(c);

                for next_char in self.it.cautious_take_while(
                    |&c| {
                        match c
                        {
                            'a'...'z' | 'A'...'Z'   => true,
                            _                       => false,
                        }
                    })
                {
                    string.push(next_char);
                }

                Some(self.make_token(TokenKind::Identifier(string)))
            },

            _ => panic!("Tried to lex unexpected character: {}", c),
        }
    }
}
