use crate::parser_combiner::traits::Parser;

pub struct BoxedParser<'input, Input, Output> {
    pub(crate) parser: Box<dyn Parser<'input, Input, Output> + 'input>,
}
