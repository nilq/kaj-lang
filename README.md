# Kaj

A programming language for generic use, designed with following ideas in mind:

- To have completely dynamic, thus just as unreasonably hard to debug as Lua
- To have lots of sugar and a less wordy syntax
- To borrow cool concepts from other languages like F#, Lobster, HolyC, Rust and maybe others 

## A taste of Kaj

```
use kaj.tables (*)

world = []

fun make_player x y =
  return {
    x: x
    y: y
  }

fun love.load =
  world.push(make_player(100 100))
```

```
use entities (*)

-- match-pattern functions
function spawn_from_color color x y =
  | 'black'  => make_block(x y)
  | 'yellow' => make_player(x y)
  | _        => error('no such thing')
```
