extern crate colored;

mod kaj;

use self::kaj::source::*;
use self::kaj::lexer::*;
use self::kaj::parser::*;

fn main() {
  let test = r#"
a = 100

fun foo.load a =
  return a + 100

fun bar =
  return 10
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

  let mut parser  = Parser::new(tokens, &source);

  match parser.parse() {
    Ok(ref ast) => {
      println!("{:#?}", ast)
    },
    _ => (),
  }
}
