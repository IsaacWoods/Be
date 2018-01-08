/*
 * Copyright (C) 2017, Isaac Woods.
 * See LICENCE.md
 */

use cautious_iter::CautiousIterator;
use std::iter::Peekable;
use std::str::CharIndices;

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

impl Token
{
    fn new(kind : TokenKind, offset : usize, line : usize, line_offset : usize) -> Token
    {
        Token
        {
            kind        : kind,
            offset      : offset,
            line        : line,
            line_offset : line_offset,
        }
    }
}

pub struct TokenStream<'a>
{
    it                  : Peekable<CharIndices<'a>>,
    current_line        : usize,
    current_line_offset : usize,
}

impl<'a> TokenStream<'a>
{
    pub fn new(source : &'a str) -> TokenStream
    {
        TokenStream
        {
            it                  : source.char_indices().peekable(),
            current_line        : 0,
            current_line_offset : 0,
        }
    }

    fn make_token(&self, kind : TokenKind, offset : usize) -> Token
    {
        Token::new(kind, offset, self.current_line, self.current_line_offset)
    }
}

impl<'a> Iterator for TokenStream<'a>
{
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item>
    {
        let (offset, c) = match self.it.next()
                          {
                              Some(c)   => c,
                              None      => return None,
                          };

        match c
        {
            '$' => Some(self.make_token(TokenKind::DollarSign, offset)),

            '\n' =>
            {
                let token = self.make_token(TokenKind::NewLine, offset);
                self.current_line += 1;
                self.current_line_offset = 0;
                Some(token)
            },

            'a'...'z' | 'A'...'Z' =>
            {
                let mut string = String::new();
                string.push(c);

                for (_, n) in self.it.cautious_take_while(
                    |&(_, c)| {
                        match c
                        {
                            'a'...'z' | 'A'...'Z'   => true,
                            _                       => false,
                        }
                    })
                {
                    string.push(n);
                }

                Some(self.make_token(TokenKind::Identifier(string), offset))
            },

            _ => panic!("Tried to lex unexpected character: {}", c),
        }
    }
}
