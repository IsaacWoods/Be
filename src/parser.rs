/*
 * Copyright (C) 2017, Isaac Woods.
 * See LICENCE.md
 */

use token::{TokenStream,TokenKind};

#[derive(Debug)]
pub enum BindingState
{
    Unknown,
    Known(isize),
}

#[derive(Debug)]
pub struct Binding
{
    name    : String,
    state   : BindingState,
}

pub struct Parser<'a>
{
    stream      : TokenStream<'a>,
    bindings    : Vec<Binding>,
}

impl<'a> Parser<'a>
{
    pub fn new(stream : TokenStream<'a>) -> Parser<'a>
    {
        Parser
        {
            stream      : stream,
            bindings    : Vec::new(),
        }
    }

    pub fn parse(&mut self)
    {
        while let Some(token) = self.stream.next()
        {
            match token.kind
            {
                TokenKind::Let => self.parse_let(),

                TokenKind::NewLine => { },

                _ => println!("Unparsed: {:?}", token),
            }
        }

        for binding in self.bindings.iter()
        {
            println!("Binding: {:?}", binding);
        }
    }

    pub fn parse_let(&mut self)
    {
        let identifier = self.stream.next().unwrap();
        let name : String;

        match identifier.kind
        {
            TokenKind::Identifier(identifier_name) => name = identifier_name,
            _ => panic!("Incorrect kind for let binding: identifier expected"),
        }

        self.stream.consume(TokenKind::Equals);

        let value = self.stream.next().unwrap();

        match value.kind
        {
            TokenKind::Identifier(value_name) =>
            {
                // TODO: store a thunk to calculate this binding's value when we can
                self.bindings.push(Binding { name : name, state : BindingState::Unknown });
            },

            TokenKind::Integer(value) =>
            {
                self.bindings.push(Binding { name : name, state : BindingState::Known(value) });
            },

            _ => panic!("Shits fucked"),
        }
    }
}
