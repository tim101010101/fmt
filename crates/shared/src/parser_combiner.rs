mod boxed_parser;
mod combiner;
// mod input_stream;
mod meta_parser;
mod traits;

pub use {
    boxed_parser::BoxedParser,
    combiner::*,
    meta_parser::*,
    traits::{Parser, ParserResult},
};

/// implement `Parser` trait for all the `Parser-Like` functions
impl<'input, Input, Output, F> Parser<'input, Input, Output>
    for F
where
    F: Fn(Input) -> ParserResult<'input, Input, Output>,
{
    fn parse(
        &self,
        input: Input,
    ) -> ParserResult<'input, Input, Output> {
        self(input)
    }
}

impl<'input, Input, Output>
    BoxedParser<'input, Input, Output>
{
    pub fn new<P>(parser: P) -> Self
    where
        P: Parser<'input, Input, Output> + 'input,
    {
        BoxedParser {
            parser: Box::new(parser),
        }
    }
}

/// parser wrapped in the `Box` implemented the `Parser` trait
impl<'input, Input, Output> Parser<'input, Input, Output>
    for BoxedParser<'input, Input, Output>
{
    fn parse(
        &self,
        input: Input,
    ) -> ParserResult<'input, Input, Output> {
        self.parser.parse(input)
    }
}
