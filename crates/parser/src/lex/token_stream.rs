use crate::lex::lexed::Token;
use crate::syntax_kind::SyntaxKind;

#[derive(Clone)]
pub struct TokenStream {
    stream: Vec<Token>,
    mark: Vec<usize>,
    cur: usize,
}

impl TokenStream {
    pub fn new(stream: Vec<Token>) -> Self {
        TokenStream {
            stream,
            mark: Vec::new(),
            cur: 0,
        }
    }
    pub fn next(&mut self) -> Option<Token> {
        if let Some(token) = self.stream.get(self.cur) {
            self.cur += 1;
            Some(token.to_owned())
        } else {
            None
        }
    }
    pub fn look_ahead(&self, dist: usize) -> Option<SyntaxKind> {
        if let Some((kind, _)) = self.stream.get(self.cur + dist) {
            Some(*kind)
        } else {
            None
        }
    }
    pub fn mark(&mut self) {
        self.mark.push(self.cur);
    }
    pub fn back_track(&mut self) {
        self.cur = if let Some(m) = self.mark.pop() { m } else { 0 }
    }
    pub fn try_match(&mut self, expect: SyntaxKind) -> Result<Token, ()> {
        match self.look_ahead(0) {
            Some(kind) if kind == expect => Ok(self.next().unwrap().to_owned()),
            _ => Err(()),
        }
    }
    pub fn try_match_many(&mut self, expects: Vec<SyntaxKind>) -> Result<Vec<Token>, ()> {
        let mut res = Vec::<Token>::new();
        for expect in expects {
            match self.try_match(expect) {
                Ok(token) => res.push(token),
                Err(_) => return Err(()),
            };
        }
        Ok(res)
    }
}

#[cfg(test)]
mod tests {
    use crate::lex::lexed::Token;
    use crate::lex::token_stream::TokenStream;
    use crate::syntax_kind::*;
    use std::borrow::{BorrowMut, Cow};

    fn get_token_stream() -> Vec<Token> {
        Vec::from([
            (DEFINATOR, "const".to_string()),
            (ID, "a".to_string()),
            (EQ, "=".to_string()),
            (STRING, "\"Hello World\"".to_string()),
            (SEMI, ";".to_string()),
        ])
    }

    #[test]
    fn t() {
        let token_stream = TokenStream::new(get_token_stream());
        let mut c = Cow::Borrowed(&token_stream);
        let get_next = |t: &mut TokenStream| t.next().unwrap().to_owned();

        assert_eq!(get_next(c.to_mut()), (DEFINATOR, "const".to_string()));
        assert_eq!(get_next(c.to_mut()), (ID, "a".to_string()));
        assert_eq!(c.cur, 2);
    }

    #[test]
    fn test_try_match() {
        let mut token_stream = TokenStream::new(get_token_stream());
        assert_eq!(
            Ok((DEFINATOR, "const".to_string())),
            token_stream.try_match(DEFINATOR)
        );
        assert_eq!(Ok((ID, "a".to_string())), token_stream.try_match(ID));
        assert_eq!(Ok((EQ, "=".to_string())), token_stream.try_match(EQ));
        assert_eq!(Err(()), token_stream.try_match(ID))
    }

    #[test]
    fn test_try_match_many() {
        let mut token_stream = TokenStream::new(get_token_stream());
        let token_list = token_stream.try_match_many(Vec::from([DEFINATOR, ID, EQ]));
        assert_eq!(
            token_list,
            Ok(Vec::from([
                (DEFINATOR, "const".to_string()),
                (ID, "a".to_string()),
                (EQ, "=".to_string()),
            ]))
        );

        assert_eq!(token_stream.try_match(DEFINATOR), Err(()));
    }
}
