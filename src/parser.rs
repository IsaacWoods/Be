/*
 * Copyright (C) 2017, Isaac Woods.
 * See LICENCE.md
 */

use token::{TokenStream,TokenKind};

pub struct Parser<'a>
{
    stream : TokenStream<'a>,
}

impl<'a> Parser<'a>
{
    pub fn new(stream : TokenStream<'a>) -> Parser<'a>
    {
        Parser
        {
            stream : stream,
        }
    }

    pub fn parse(&mut self)
    {
        while let Some(token) = self.stream.next()
        {
            match token.kind
            {
                TokenKind::Let => self.parse_let(),

                _ => println!("Unparsed: {:?}", token),
            }
        }
    }

    pub fn parse_let(&mut self)
    {
        println!("Parsing let");
    }
}
