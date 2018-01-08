/*
 * Copyright (C) 2017, Isaac Woods.
 * See LICENCE.md
 */

use std::iter::Peekable;

pub struct CautiousTakeWhile<'a, T : Iterator + 'a, P> 
    where T::Item : 'a
{
    it          : &'a mut Peekable<T>,
    predicate   : P,
}

impl<'a, T : Iterator, P> Iterator for CautiousTakeWhile<'a, T, P>
    where P : FnMut(&T::Item) -> bool
{
    type Item = T::Item;

    fn next(&mut self) -> Option<T::Item>
    {
        if match self.it.peek()
           {
               Some(ref value)  => (self.predicate)(value),
               _                => false,
           }
        {
            self.it.next()
        }
        else
        {
            None
        }
    }
}

pub trait CautiousIterator<'a, T> : Iterator
    where T : Iterator
{
    fn cautious_take_while<P>(&'a mut self, P) -> CautiousTakeWhile<'a, T, P>
        where P : FnMut(&Self::Item) -> bool;
}

impl<'a, T : Iterator> CautiousIterator<'a, T> for Peekable<T>
{
    fn cautious_take_while<P>(&'a mut self, f : P) -> CautiousTakeWhile<'a, T, P>
        where P : FnMut(&'a(T::Item)) -> bool
        {
            CautiousTakeWhile
            {
                it              : self,
                predicate       : f,
            }
        }
}
