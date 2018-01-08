/*
 * Copyright (C) 2017, Isaac Woods.
 * See LICENCE.md
 */

mod cautious_iter;

use cautious_iter::CautiousIterator;

#[derive(Debug)]
enum TokenKind
{
    DollarSign,

    Identifier(String),

    NewLine,
}

#[derive(Debug)]
struct Token
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

fn main() {
    let source = "abc$$def";
    let mut line = 0;
    let mut line_offset = 0;
    let mut char_indices = source.char_indices().peekable();

    while let Some((offset, c)) = char_indices.next()
    {
        let token = match c
                    {
                        '$' => Token::new(TokenKind::DollarSign, offset, line, line_offset),

                        '\n' =>
                        {
                            let temp = Token::new(TokenKind::NewLine, offset, line, line_offset);
                            line += 1;
                            line_offset = 0;
                            temp
                        },

                        'a'...'z' | 'A'...'Z' =>
                        {
                            let mut string = String::new();
                            string.push(c);

                            for (_, n) in char_indices.cautious_take_while(
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

                            Token::new(TokenKind::Identifier(string), offset, line, line_offset)
                        },

                        _ => panic!("Tried to lex unexpected character: {}", c),
                    };

        println!("{:#?}", token);
    }
}
