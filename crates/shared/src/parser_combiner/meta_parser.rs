use crate::parser_combiner::Parser;

pub fn empty<'input, Input>() -> impl Parser<'input, Input, ()>
where
    Input: 'input,
{
    move |input: Input| Ok((input, ()))
}

// TODO temporarily use cloning to passing the tests
pub fn atom<'input, Input, InputItem>() -> impl Parser<'input, Input, InputItem>
where
    InputItem: PartialOrd,
    Input: IntoIterator<Item = InputItem> + FromIterator<InputItem> + PartialOrd + Clone,
{
    move |input: Input| match input.clone().into_iter().next() {
        Some(next) => Ok((input.into_iter().skip(1).collect(), next)),
        None => Err(input),
    }
}

// TODO temporarily use cloning to passing the tests
pub fn literal<'input, Input, InputItem>(expect: Input) -> impl Parser<'input, Input, Input>
where
    InputItem: PartialOrd,
    Input: IntoIterator<Item = InputItem> + FromIterator<InputItem> + PartialOrd + Clone,
{
    move |input: Input| {
        let len = expect.clone().into_iter().count();
        let expect_input = input.clone().into_iter().take(len).collect::<Input>();
        if expect_input == expect {
            Ok((input.into_iter().skip(len).collect(), expect_input))
        } else {
            Err(input)
        }
    }
}

// TODO temporarily use cloning to passing the tests
pub fn single_literal<'input, Input, InputItem>(
    expect: InputItem,
) -> impl Parser<'input, Input, InputItem>
where
    InputItem: PartialOrd,
    Input: IntoIterator<Item = InputItem> + FromIterator<InputItem> + PartialOrd + Clone,
{
    move |input: Input| match input.clone().into_iter().next() {
        Some(next) if next == expect => Ok((input.into_iter().skip(1).collect(), next)),
        _ => Err(input),
    }
}

pub fn skip_literal<'input, Input, InputItem>(expect: Input) -> impl Parser<'input, Input, ()>
where
    InputItem: PartialOrd + 'input,
    Input: IntoIterator<Item = InputItem> + FromIterator<InputItem> + PartialOrd + Clone + 'input,
{
    literal(expect).map(|_| ())
}

pub fn skip_single_literal<'input, Input, InputItem>(
    expect: InputItem,
) -> impl Parser<'input, Input, ()>
where
    InputItem: PartialOrd + 'input,
    Input: IntoIterator<Item = InputItem> + FromIterator<InputItem> + PartialOrd + Clone + 'input,
{
    single_literal(expect).map(|_| ())
}

#[cfg(test)]
mod tests {
    use crate::parser_combiner::meta_parser::{
        literal, single_literal, skip_literal, skip_single_literal,
    };
    use crate::parser_combiner::{atom, empty, Parser};

    #[derive(Debug, PartialEq, PartialOrd, Eq, Copy, Clone)]
    struct SyntaxKind(pub u16);
    const A: SyntaxKind = SyntaxKind(1);
    const B: SyntaxKind = SyntaxKind(2);

    #[test]
    fn test_empty() {
        let input = vec![A, B];
        assert_eq!(Ok((vec![A, B], ())), empty().parse(input))
    }

    #[test]
    fn test_atom() {
        let input = vec![A, B];
        assert_eq!(Ok((vec![B], A)), atom().parse(input))
    }

    #[test]
    fn test_literal() {
        let input = "Hello";
        assert_eq!(
            Ok((vec![], Vec::from("Hello"))),
            literal(Vec::from("Hello")).parse(input.into())
        );

        let input = vec![A, B];
        assert_eq!(Ok((vec![B], vec![A])), literal(vec![A]).parse(input))
    }

    #[test]
    fn test_single_literal() {
        let input = vec![A, B];
        assert_eq!(Ok((vec![B], A)), single_literal(A).parse(input))
    }

    #[test]
    fn test_skip_literal() {
        let input = "Hello";
        assert_eq!(
            Ok((vec![], ())),
            skip_literal(Vec::from("Hello")).parse(input.into())
        );

        let input = vec![A, B];
        assert_eq!(Ok((vec![B], ())), skip_literal(vec![A]).parse(input))
    }

    #[test]
    fn test_skip_single_literal() {
        // let input = "Hello";
        // assert_eq!(Ok(("ello", ())), single_literal('H').parse(input));

        let input = vec![A, B];
        assert_eq!(Ok((vec![B], ())), skip_single_literal(A).parse(input))
    }
}
