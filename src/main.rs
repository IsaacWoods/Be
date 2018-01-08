/*
 * Copyright (C) 2017, Isaac Woods.
 * See LICENCE.md
 */

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

                            while let Some(&(_, n)) = char_indices.peek()
                            {
                                match n
                                {
                                    'a'...'z' | 'A'...'Z' =>
                                    {
                                        string.push(n);
                                        char_indices.next();
                                    },

                                    _ => break,
                                }
                            }

                            Token::new(TokenKind::Identifier(string), offset, line, line_offset)
                        },

                        _ => panic!("Unexpected char to be lexed: {}", c),
                    };

        println!("{:#?}", token);
    }
}
