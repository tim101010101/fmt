# FMT

A simple tool for checking and formatting code

## Feature

- format js

## TODO

- check
- support multiple languages

## Current Progress

```rust
#[test]
fn smoke() {
    let input = r#"const a=1;function foo(bar,baz){const a=1;if(a==1){const a=1;}else if(b==2){const b=1;}}"#;
    let ast = syntax(lex(input)).unwrap();
    let mut g = Generator::new();
    let output = g.gen(&ast);
    println!("{}", output);
}
```

output

```js
const a = 1;
function foo(bar, baz) {
  const a = 1;
  if (a == 1) {
    const a = 1;
  } else if (b == 2) {
    const b = 1;
  }
}
```

