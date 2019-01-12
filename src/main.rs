extern crate colored;

mod kaj;

use self::kaj::source::*;
use self::kaj::lexer::*;
use self::kaj::parser::*;

fn main() {
  let test = r#"
a = 100

player = {
  x: 100
  y: 100
}

fun player.move self x y =
  self.x = self.x + x
  self.y = self.y + y

fun foo.load a =
  return [a, 1, 3, 4]

fun bar a b =
  if b + a == 110
    return 10
  elif true
    return "hey"
  else
    return "hey"

bar(100 100)
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
