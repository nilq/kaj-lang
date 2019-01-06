# noom

## Syntax

### Variables

```
a := "hey"

b: int = 10

c: bool =
  true
```

### Functions

#### As statements

```
fn add(a: int, b: int) -> int =
  a + b
```

#### As expressions

Has inferred return type ...

```
add := fn(a: int, b: int) => a + b
```

### Literals

### Arrays

```
a: [int] = [1, 2, 3, 4, 5]
```

#### Tables

```
a: table = {
  key: 100
  100
  20
}
```

### Flow-control

#### If

```
if true
  print("true is true")
elif
  print("true isn't true")
else
  print("true is something else")
```

#### Switch

```
a := 10

switch a
  0  => print("zero")
  5  => print("five")
  10 => print("ten")
```

### Loops

#### For

```
for i := 1, 100
  print(i)
```

```
for i in [1, 2, 5, 6]
  print(i)
```

```
for i in 0 .. 100
  print(i)
```

#### Loop

```
loop
  print("this is forever")
```

```
a := 1

loop a < 100
  print(a)

  a += 1
```

```
loop 10
  print("ten times")
```
