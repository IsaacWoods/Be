/*
 * Copyright (C) 2017, Isaac Woods.
 * See LICENCE.md
 */

mod cautious_iter;
mod token;

use token::TokenStream;

fn main() {
    let source = "abc$$def";
    let token_stream = TokenStream::new(source);

    for token in token_stream
    {
        println!("{:#?}", token);
    }
}
