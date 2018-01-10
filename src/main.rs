/*
 * Copyright (C) 2017, Isaac Woods.
 * See LICENCE.md
 */

mod cautious_iter;
mod token;
mod parser;

use token::TokenStream;
use parser::Parser;

fn main() {
    /*
     * XXX: Use # to escape special characters
     */
    let source = r###"
        let x = 5
        let y = x
    "###;

    let token_stream = TokenStream::new(source);
    let mut parser = Parser::new(token_stream);
    parser.parse();
}
