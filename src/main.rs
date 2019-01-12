extern crate colored;

mod kaj;

use self::kaj::source::*;
use self::kaj::lexer::*;
use self::kaj::parser::*;

fn main() {
  let test = r#"
player = {
  x: 100
  y: 100
}

fun player\move x y =
  self.x = self.x + x
  self.y = self.y + y
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
