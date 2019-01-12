extern crate colored;

mod kaj;

use self::kaj::source::*;

fn main() {
  let test = r#"
a = 100

fun foo a =
  a = a + 100

  return a

foo(a)
  "#;

  let source = Source::from("<main>", test.lines().map(|x| x.into()).collect::<Vec<String>>());
  let lexer  = Lexer::default(test.chars().collect(), &source);

  let mut tokens = Vec::new();

  for token_result in lexer {
    if let Ok(token) = token_result {
      tokens.push(token)
    } else {
      return
    }
  }
}
