let snippet = `
var b3 = 2;
a = 1 + ( b3 + 4 );
return a;
`;

const EOF = undefined;
const TT = {
  num: 'num',
  id: 'id',
  keywords: 'keywords',
  lparen: 'lparen',
  rparen: 'rparen',
  semicolon: 'semicolon',
  whitespace: 'whitespace',
  plus: 'plus',
  assign: 'assign',
};

const S = {
  start: 'start',
  done: 'done',
  ...TT,
};

const isKeywords = t => ['function', 'return', 'if', 'var'].includes(t);
const isDigit = c => /\d/.test(c);
const isValidId = c => /[A-Za-z0-9]/.test(c);
const isBlank = c => /(\s|\t|\n)/.test(c);

const tokenize = code => {
  let state = S.start;
  let currentToken = null;
  let idx = 0;
  let lookup = 0;

  while (code[lookup] !== EOF) {
    while (state !== S.done) {
      let c = code[lookup++];
      switch (state) {
        case S.start:
          if (isDigit(c)) {
            state = S.num;
          } else if (isValidId(c)) {
            state = S.id;
          } else if (isBlank(c)) {
            state = S.done;
          } else if (c === '=') {
            currentToken = [TT.assign, '='];
            state = S.done;
          } else if (c === '+') {
            currentToken = [TT.plus, '+'];
            state = S.done;
          } else if (c === ';') {
            currentToken = [TT.semicolon, ';'];
            state = S.done;
          } else if (c === '(') {
            currentToken = [TT.lparen, '('];
            state = S.done;
          } else if (c === ')') {
            currentToken = [TT.rparen, ')'];
            state = S.done;
          }
          break;

        case S.num:
          if (isDigit(c)) {
            state = S.num;
          } else {
            currentToken = [TT.num, code.slice(idx, lookup - 1)];
            lookup -= 1;
            state = S.done;
          }
          break;

        case S.id:
          if (isValidId(c)) {
            state = S.id;
          } else {
            let tempToken = code.slice(idx, lookup - 1);
            lookup -= 1;
            if (isKeywords(tempToken)) {
              currentToken = [TT.keywords, tempToken];
            } else {
              currentToken = [TT.id, tempToken];
            }
            state = S.done;
          }
          break;
      }
    }

    currentToken && console.log(currentToken);
    currentToken = null;

    idx = lookup;
    state = S.start;
  }
};

tokenize(snippet);

