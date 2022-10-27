use crate::parser_combiner::traits::Parser;

pub fn pair<'input, Input, P1, P2, O1, O2>(
    parser1: P1,
    parser2: P2,
) -> impl Parser<'input, Input, (O1, O2)>
where
    P1: Parser<'input, Input, O1>,
    P2: Parser<'input, Input, O2>,
{
    move |input| {
        parser1.parse(input).and_then(|(next_input, output1)| {
            parser2
                .parse(next_input)
                .map(|(final_input, output2)| (final_input, (output1, output2)))
        })
    }
}

pub fn map<'input, Input, P, Output, MapFn, NewOutput>(
    parser: P,
    map_fn: MapFn,
) -> impl Parser<'input, Input, NewOutput>
where
    P: Parser<'input, Input, Output>,
    MapFn: Fn(Output) -> NewOutput,
{
    move |input| {
        parser
            .parse(input)
            .map(|(next_input, output)| (next_input, map_fn(output)))
    }
}

pub fn left<'input, Input, P1, P2, O1, O2>(
    parser1: P1,
    parser2: P2,
) -> impl Parser<'input, Input, O1>
where
    P1: Parser<'input, Input, O1>,
    P2: Parser<'input, Input, O2>,
{
    map(pair(parser1, parser2), |(left, _)| left)
}

pub fn right<'input, Input, P1, P2, O1, O2>(
    parser1: P1,
    parser2: P2,
) -> impl Parser<'input, Input, O2>
where
    P1: Parser<'input, Input, O1>,
    P2: Parser<'input, Input, O2>,
{
    map(pair(parser1, parser2), |(_, right)| right)
}

pub fn and_then<'input, Input, CurParser, CurOutput, NextFn, NextParser, NextOutput>(
    cur_parser: CurParser,
    next_fn: NextFn,
) -> impl Parser<'input, Input, NextOutput>
where
    CurParser: Parser<'input, Input, CurOutput>,
    NextParser: Parser<'input, Input, NextOutput>,
    NextFn: Fn(CurOutput) -> NextParser,
{
    move |input| match cur_parser.parse(input) {
        Ok((next_input, cur_output)) => match next_fn(cur_output).parse(next_input) {
            Ok((final_input, next_output)) => Ok((final_input, next_output)),
            Err(err) => Err(err),
        },
        Err(err) => Err(err),
    }
}

// TODO just deep clone for the time being
pub fn judge<'input, Input, P, Output, JudgeFn>(
    parser: P,
    judge_fn: JudgeFn,
) -> impl Parser<'input, Input, Output>
where
    P: Parser<'input, Input, Output>,
    JudgeFn: Fn(&Output) -> bool,
    Input: Clone,
{
    move |input: Input| {
        let input_clone = input.clone();
        match parser.parse(input) {
            Ok((next_input, output)) if judge_fn(&output) => Ok((next_input, output)),
            _ => Err(input_clone),
        }
    }
}

// TODO just deep clone for the time being
pub fn either<'input, Input, P1, P2, Output>(
    parser1: P1,
    parser2: P2,
) -> impl Parser<'input, Input, Output>
where
    P1: Parser<'input, Input, Output>,
    P2: Parser<'input, Input, Output>,
    Input: Clone,
{
    move |input: Input| {
        let input_clone = input.clone();
        match parser1.parse(input) {
            Ok((next_input, output)) => Ok((next_input, output)),
            Err(_) => parser2.parse(input_clone),
        }
    }
}

// TODO just deep clone for the time being
pub fn one_or_more<'input, Input, P, Output>(parser: P) -> impl Parser<'input, Input, Vec<Output>>
where
    P: Parser<'input, Input, Output>,
    Input: Clone,
{
    move |mut input: Input| {
        let mut result = Vec::new();

        if let Ok((next_input, item)) = parser.parse(input.clone()) {
            input = next_input;
            result.push(item);
        } else {
            return Err(input);
        }

        while let Ok((next_input, item)) = parser.parse(input.clone()) {
            input = next_input;
            result.push(item);
        }

        Ok((input, result))
    }
}

// TODO just deep clone for the time being
pub fn zero_or_more<'input, Input, P, Output>(parser: P) -> impl Parser<'input, Input, Vec<Output>>
where
    P: Parser<'input, Input, Output>,
    Input: Clone,
{
    move |mut input: Input| {
        let mut result = Vec::new();

        while let Ok((next_input, item)) = parser.parse(input.clone()) {
            input = next_input;
            result.push(item);
        }

        Ok((input, result))
    }
}

#[cfg(test)]
mod tests {
    use crate::parser_combiner::boxed_parser::BoxedParser;
    use crate::parser_combiner::combiner::{and_then, either, judge, left, map, pair, right};
    use crate::parser_combiner::traits::Parser;

    #[derive(Debug, PartialEq, PartialOrd, Eq, Copy, Clone)]
    struct SyntaxKind(pub u16);
    const A: SyntaxKind = SyntaxKind(1);
    const B: SyntaxKind = SyntaxKind(2);

    fn str_parser<'input>(expect: &'input str) -> impl Parser<'input, &str, &str> {
        move |input: &'input str| match input.get(0..expect.len()) {
            Some(next) if next == expect => Ok((&input[expect.len()..], expect)),
            _ => Err(input),
        }
    }

    fn token_parser<'input>(
        expect: SyntaxKind,
    ) -> impl Parser<'input, Vec<SyntaxKind>, SyntaxKind> {
        move |mut input: Vec<SyntaxKind>| match input.pop() {
            Some(next) if next == expect => Ok((input, next)),
            _ => Err(input),
        }
    }

    fn get_stuff<'input>() -> (
        Vec<SyntaxKind>,
        BoxedParser<'input, Vec<SyntaxKind>, SyntaxKind>,
        BoxedParser<'input, Vec<SyntaxKind>, SyntaxKind>,
    ) {
        (
            vec![B, A],
            BoxedParser::new(token_parser(A)),
            BoxedParser::new(token_parser(B)),
        )
    }

    #[test]
    fn test_str_parser() {
        let input = "Hello World";
        let hello_parser = str_parser("Hello");
        assert_eq!(Ok((" World", "Hello")), hello_parser.parse(input))
    }

    #[test]
    fn test_token_parser() {
        let input = vec![B, A];
        let a_parser = token_parser(A);
        assert_eq!(Ok((vec![B], A)), a_parser.parse(input))
    }

    #[test]
    fn test_pair() {
        let (input, p1, p2) = get_stuff();
        assert_eq!(Ok((vec![], (A, B))), pair(p1, p2).parse(input))
    }

    #[test]
    fn test_map() {
        let (input, p1, _) = get_stuff();
        assert_eq!(
            Ok((vec![B], vec![A])),
            map(p1, |o| { vec![o] }).parse(input)
        )
    }

    #[test]
    fn test_left() {
        let (input, p1, p2) = get_stuff();
        assert_eq!(Ok((vec![], A)), left(p1, p2).parse(input))
    }

    #[test]
    fn test_right() {
        let (input, p1, p2) = get_stuff();
        assert_eq!(Ok((vec![], B)), right(p1, p2).parse(input))
    }

    #[test]
    fn test_judge() {
        let (input, p1, _) = get_stuff();
        assert_eq!(Ok((vec![B], A)), judge(p1, |o| *o == A).parse(input))
    }

    #[test]
    fn test_and_then() {
        let (input, p1, _) = get_stuff();
        assert_eq!(
            Ok((vec![], B)),
            and_then(p1, move |_| { judge(token_parser(B), |k| *k == B) }).parse(input)
        )
    }

    #[test]
    fn test_either() {
        let (input, p1, p2) = get_stuff();
        assert_eq!(Ok((vec![B], A)), either(p2, p1).parse(input))
    }
}
