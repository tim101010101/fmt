use crate::parser_combiner::Parser;

// TODO temporarily use cloning to passing the tests
pub fn literal<'input, Input, InputItem>(expect: Input) -> impl Parser<'input, Input, ()>
where
    InputItem: PartialOrd,
    Input: IntoIterator<Item = InputItem> + FromIterator<InputItem> + PartialOrd + Clone,
{
    move |input: Input| {
        let len = expect.clone().into_iter().count();
        let expect_input = input.clone().into_iter().take(len).collect::<Input>();
        if expect_input == expect {
            Ok((input.into_iter().skip(len).collect(), ()))
        } else {
            Err(input)
        }
    }
}

// TODO temporarily use cloning to passing the tests
pub fn single_literal<'input, Input, InputItem>(expect: InputItem) -> impl Parser<'input, Input, ()>
where
    InputItem: PartialOrd,
    Input: IntoIterator<Item = InputItem> + FromIterator<InputItem> + PartialOrd + Clone,
{
    move |input: Input| match input.clone().into_iter().next() {
        Some(next) if next == expect => Ok((input.into_iter().skip(1).collect(), ())),
        _ => Err(input),
    }
}

#[cfg(test)]
mod tests {
    use crate::parser_combiner::meta_parser::{literal, single_literal};
    use crate::parser_combiner::Parser;

    #[derive(Debug, PartialEq, PartialOrd, Eq, Copy, Clone)]
    struct SyntaxKind(pub u16);
    const A: SyntaxKind = SyntaxKind(1);
    const B: SyntaxKind = SyntaxKind(2);

    #[test]
    fn test_literal() {
        let input = "Hello";
        assert_eq!(
            Ok((vec![], ())),
            literal(Vec::from("Hello")).parse(input.into())
        );

        let input = vec![A, B];
        assert_eq!(Ok((vec![B], ())), literal(vec![A]).parse(input))
    }

    #[test]
    fn test_single_literal() {
        // let input = "Hello";
        // assert_eq!(Ok(("ello", ())), single_literal('H').parse(input));

        let input = vec![A, B];
        assert_eq!(Ok((vec![B], ())), single_literal(A).parse(input))
    }
}
