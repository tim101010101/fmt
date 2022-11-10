use crate::parser_combiner::boxed_parser::BoxedParser;
use crate::parser_combiner::combiner::{and_then, either, judge, map};

/// Result((Input, Output), Input)
pub type ParserResult<'input, Input, Output> = Result<(Input, Output), Input>;

pub trait Parser<'input, Input, Output> {
    fn parse(&self, input: Input) -> ParserResult<'input, Input, Output>;

    fn map<MapFn, NewOutput>(self, map_fn: MapFn) -> BoxedParser<'input, Input, NewOutput>
    where
        Self: Sized + 'input,
        Input: 'input,
        Output: 'input,
        NewOutput: 'input,
        MapFn: Fn(Output) -> NewOutput + 'input,
    {
        BoxedParser::new(map(self, map_fn))
    }

    fn judge<JudgeFn>(self, judge_fn: JudgeFn) -> BoxedParser<'input, Input, Output>
    where
        Self: Sized + 'input,
        Input: Clone + 'input,
        Output: 'input,
        JudgeFn: Fn(&Output) -> bool + 'input,
    {
        BoxedParser::new(judge(self, judge_fn))
    }

    fn and_then<NextFn, NextParser, NextOutput>(
        self,
        next_fn: NextFn,
    ) -> BoxedParser<'input, Input, NextOutput>
    where
        Self: Sized + 'input,
        Input: 'input,
        Output: 'input,
        NextOutput: 'input,
        NextParser: Parser<'input, Input, NextOutput> + 'input,
        NextFn: Fn(Output) -> NextParser + 'input,
    {
        BoxedParser::new(and_then(self, next_fn))
    }

    fn or<OtherParser>(self, other_parser: OtherParser) -> BoxedParser<'input, Input, Output>
    where
        Self: Sized + 'input,
        Input: Clone + 'input,
        Output: 'input,
        OtherParser: Parser<'input, Input, Output> + 'input,
    {
        BoxedParser::new(either(self, other_parser))
    }
}
