# `rollang` Reference

`rollang` is a language for DnD players to easily manage dice rolls during play.
This reference describes the syntax and semantics of the language.
Implementation details are not the main focus, however, some notes are present.
 
Note that this reference assumes familiarity with the core mechanics of DnD.s
For DnD rules see the [DnD Beyond Introduction](https://www.dndbeyond.com/sources/basic-rules/introduction)

---

## 2 Lexical Analysis

A **session** describes the set of commands given to, and executed, by a `rollang` interpreter.
A session is ended by the `end` keyword.

The interpreter may be run as an interactive REPL, or alternatively it may parse commands written to a `.roll` source file.

Throughout this reference, `>>>` refers to a REPL prompt

## 2.1 Line Structure

### 2.1.1 Logical lines

**Logical Lines** are terminated by characters representing the `NEWLINE` token.

### 2.1.2 Physical lines

**Physical Lines** are by terminated by characters representing a carriage return.
This is specific to the host platform

> The **host platform** is the operating system that manages the interpreter process, like Windows, Linux, or MacOS

Not that a logical line can span multiple physical lines
if the logical line contains specific tokens, like curly braces `{ }`

### 2.1.3 Comments

**Comments** are lines that are ignored by the interpreter

Single-line comments are denoted by `//`

Multi-line comments are denoted by `/* */`

### 2.1.4 Whitespace

Whitespace separates tokens, although the amount of whitespace beyond one is insignificant.

## 2.2 Identifiers & Keywords

### 2.2.1 Keywords

| Keyword | Semantic |
|---|---|
| `roll` | procedure to roll a dice and produce a random number |
| `print` | Send bytes/string to standard out |
| `if` | Control flow |
| `elif` | Control flow |
| `else` | Control flow |
| `for` | Control flow (loops) |

### 2.2.2 Valid identifiers

`kebab-case` is recommended due to legibility.

Identifiers cannot start with a digit.

Identifiers are case-sensitive.

Valid characters:

- `a-z`
- `A-Z`
- `-`
- `_`
- `0-9`

## 2.3 Literals

### 2.3.1 Dice (`dice`)

The fundamental data type in `rollang` is `dice`.
The general syntax is `{amount}d{sides}`

```ts
1d20
```

if the `amount` is ommitted, it is assumed to be `1`.

```ts
d20
```

To roll a dice, use the built-in procedure `roll`:

```ts
roll 3d8
```

Or for multiple dice:

```ts
roll 3d8 2d6
```

`Dice` have the following properties:

```ts
>>> 4d6.sides
6
>>> 4d6.amount
4
>>> 4d6.max
24
>>> 4d6.min
4
```

### 2.3.2 Number (`num`)

In DnD there is no need for decimal numbers, so all numbers in `rollang` are integers.

> An **integer** is a positive or negative whole number
> Examples include is `-3`, `1234`, and `0`

### 2.3.3 String (`str`)

Strings are delimited by double quotes `" "`

They are `utf-8` encoded by default.

### 2.3.4 Compound Literals

The basic literals, `dice`, `num`, `str` can be paired together to form **compound literals**.

#### 2.3.4.a Dice + Number = Dice with Modifier

You can append a modifier, some positive or negative number, to dice

```python
>>> roll 1d20+4
1d20=6 +4 ->10
```

If you want to include spaces between the dice and the modifier, you need to use parentheses

```python
>>> roll (1d20 +4)
1d20=6 +4 ->10
```

#### 2.3.4.b Dice + String = Annotated Dice

Annotate dice with a string of text

```python
2d4:"Fire attack"
```

#### 2.3.4.c String + Number = Annotated Modifier

```python
>>> roll (1d20:"Fire" -4:"Debuff")
1d20:"Fire"=6 -4:"Debuff" ->2
```

### 2.4 Collections

### 2.4.1 Vector (`vec<T>`)

A **vector** is a generic container that can hold multiple of the same kind of stuff.
Vectors are denoted by `[ ]` and the elements of a vector are separated by commas `,`

> A vector is very similar to a list, except that vectors can only hold one type of thing
>
> - [1, "hi"] _would not_ be a vector because the collection contains mixed types (`num` and `str`)
>
> - [1, 2] _would_ be a vector because all elements of the collection are of the same type (`num`)

For example, here is a vector of dice:

```ts
[2d4, 3d8]
```

Compound literals are treated as a single type, therefore they can be collected into vectors:

```python
>>> [2d4:"Fire attack", 2d8:"Lightning"]
[2d4:"Fire attack", 2d8:"Lightning"]
```

## Built-in Functions

**repr**: prints type and values, similar to Python `repr()`. Useful for developers

```rust
repr 1d20
// Rollable: Amount(1), Sides(20), Roll<Pending>
```

**tostring**: converts literal to string

```rust
tostring 1d20
// "d20"
```

**print**: prints string version of literal to standard IO

```rust
print 1d20
// d20
```

You can specify the terminal character (default is newline, `"\n"`)

```rust
print (1d20, "")
print (1d20)
// d20d20
```

**roll**: rolls some dice (more generally a [Rollable]() or [Composite Rollable]()). For short, use `!r`

```rust
roll 1d20
// ->7
repr (roll 1d20)
// "Roll: Result([7]), Sum(7)"
```

```rust
!r 2d8
// [5, 1] ->6
repr (!r 2d8)
// "Roll: Result([5, 1]), Sum(6)"
```

**rollf**: rolls dice and formats the information about the roll. For short, use `!rf`

> This is useful for printing logs, or printing the rolls to a file

```rust
rollf 1d20
// 1d20=7
!rf 2d6
// 2d6=[5, 3] ->8
```

Under the hood it basically calls 1) `print (1d20 "=")`, the 2) `print (roll 1d20)`

## Bindings

You can bind literals (die, number, string), to a name. This makes it easier to save and reuse results.

```go
my_binding := roll d20
repr my_binding
// ->6
```

## Snapping Pieces Together

Snap different pieces together with the `:` (bind) operator

```rust
repr 1d20:"Fire attack"
// Rollable: Amount(1), Sides(20), Roll<Pending>, Note("Fire attack")
```

```rust
repr d20:+4
// Rollable: Amount(1), Sides(20), Modifier(+4), Roll<Pending>
```

## Functions


```nim
addnums :: fn(a: number, b: number) -> (c: number) {
    c = a + b
    return c
}
```

## Composite Rolls

Often times, you need to roll multiple values at once, or group multiple different rolls.

For example, rolling abilities:

```go
Abilities :: comp {
    STR: 4d6:max,
    DEX: 4d6:max,
    CON: 4d6:max,
    INT: 4d6:max,
    WIS: 4d6:max,
    CHA: 4d6:max,
}
```

Note that `roll abilities` creates a composite roll

```ts
Abilities
/* CompositeRoll: [
    STR: Rollable(Amount(4), Sides(6), Roll<Pending>),
    ...
    CHA: Rollable(Amount(4), Sides(6), Roll<Pending>),
]
*/
```

```ts
roll Abilities

```

This works because `abilities` is a built-in Composite Rollable

Rollang has a reflection method, `repr`:

```ts
repr abilities
// "abilities: Roll<Pending> = (let STR = max 4d6, let DEX = max 4d6, let CON = max 4d6, let INT = max 4d6, let WIS = max 4d6, let CHA = max 4d6,)"
```





---

## Rolling

You can roll dice using the `roll` command

Single die

```python
>>> roll 1d20
1d20=7
```

```python
>>> roll 1d20:"Fire"
1d20:"Fire"=7
```

Multiple separate dice rolls

```python
>>> roll (3d4 2d6)
3d4=[2, 2, 3] ->7
2d6=[4, 1] ->5
```

Alternatively, you can provide a roll list

```python
>>> roll [3d4, 2d6]
3d4=[2, 2, 3] 2d6=[4, 1] ->12
```

The result, specifically sum, of the roll is shown after the `->`. In the example above, the result is `12` because `2 + 2 + 3 + 4 + 1 = 12`

---

## Seed

Rolls are random, but at the beginning of a session, `roll` file (see below), or when running the interpreter, you can set a seed to get a reproducible sequence of random numbers:

```python
>>> seed 1234
seed=1234
```

or 

```bash
rollang --seed 1234
```

---

## Advantage/Disadvantage

> See [Basic Rules#AdvantageDisadvantage](https://www.dndbeyond.com/sources/basic-rules/using-ability-scores#AdvantageandDisadvantage)

Advantage

```python
>>> roll adv
1d20=7
1d20=16 <<<
```

Disadvantage

```python
>>> roll dis
1d20=7
1d20=16 <<<
```

---


---

## Built-in Commands

Highest roll

```python
>>> max (roll 2d6)
2d6=[3, 5] max ->5
```

Lowest roll

```python
>>> min (roll 3d4)
3d4=[2, 2, 3, 4] min ->2
```

Lazy-evaluation

```python
>>> max 2d4
2d4<max>
```

---

## Bindings

You can bind values, like a dice, number, or result, to a name. This makes it easier to save and reuse results.

```rust
>>> let my_binding = roll d20
my_binding=7 
```

> Note that you can't reassign a binding. If you want to reuse the same name, you have to write `let name = roll d20` again. This will discard the previous value.

> There are some other rules/restrictions on bindings. Please see the [rollang specification](./ROLLANG.md) for details

---

## Ability Scores

`abilities` is a special command that generates ability scores according to the instructions in the [Basic Rules#DetermineAbilityScores](https://www.dndbeyond.com/sources/basic-rules/step-by-step-characters#3DetermineAbilityScores) on DnDBeyond

```python
>>> roll abilities
STR=15
DEX=14
CON=11
INT=17
WIS=10
CHA=12
```

---

## Armor Class / Difficulty Class

```python
>>> ac 15
AC=15
```

AC and DC can have modifiers, too

```python
>>> ac (15 +2)
AC=15 +2 ->17
```

As well as annotations

```python
>>> ac (15 +2:"Spell effect")
AC=15 +2:"Spell effect" ->17
```

The same applies to difficulty class, but you use `dc`

```python
>>> dc (15 +2)
DC=15 +2 ->17
```

---

## Ability Checks

```python
>>> roll 3d6 > dc 6
3d6=[2, 4, 2] ->7 success
```

```python
>>> roll [3d6, 2d8] > dc 20
3d6=[2, 2, 1] 2d8=[6, 2] ->13 failure
```

```python
>>> roll 3d6 >= dc 6
3d6=[2, 2, 2] ->6 success
```

Alternatively, you can just use a number instead of the `dc` command (but then you can't include a modifier):

```python
>>> roll 3d6 >= 6
3d6=[2, 2, 2] ->6 success
```

---

## Initiative

For initiative, you can specify the number of combat participants

```python
>>> roll (init 3)
Initiative
----------
(1) 1d20=16
(2) 1d20=12
(3) 1d20=4
```

Or provide a list of strings

```python
>>> roll (init ["utahkh", "mythelia", "boro-boro"])
Initiative
----------
(1) "mythelia"  =16
(2) "boro-boro" =12
(3) "utahkh"    =4
```

> For more advanced users, you can provide a list of Types that implement the `str` function

---

## Attack rolls

`atk` is just a semantic shorthand for `1d20`

```python
>>> roll atk
1d20=7
```

```python
>>> roll atk+4
1d20=5 +4 ->9
```

```python
>>> roll (atk +4)
1d20=5 +4 ->9
```

```cpp
>>> roll atk >= ac 15
1d20=12 >=? ac(15) failure
```

```cpp
>>> roll (atk +4 +2) >= ac (19 -3)
(1d20=12 +4 +2 ->18) >=? ac(19 -3 ->16) success
```

Natural one auto-fails

```cpp
>>> roll (atk +100) >= ac 15
1d20=1 +100 >=? ac(15) failure
```

Natural 20 auto-succeeds

```cpp
>>> roll (atk -100) >= ac 15
1d20=20 -100 >=? ac(15) success
```

### Saving Throws

Saving throws function the same as attack rolls, but just have a different identifier for semantic convenience

```cpp
>>> roll (sav +4 +2) >= dc 15
1d20=12 +4 +2 ->18 >=? dc(15) success
```

---

## Composition

Use composition to group rolls together

```rust
let Dragon = (
    let stats = roll abilities,
    let fire_breath = 4d8+4,
    let stomp = 3d6+2,
)
```

> Note the use of `:=` instead of `=` for defining compositions

One-liner version

```rust
>>> let Dragon = (let stats=roll abilities, let fire_breath=4d8+4, let stomp=3d6+2)
Dragon::
    stats::
        STR
        DEX
        CON
        INT
        WIS
        CHA
    fire_breath
    stomp
```

```rust
>>> roll Dragon::fire_breath
Dragon::
    fire_breath=[3, 5, 1, 4] +4 ->17
```

Alternatively, roll all dice:

```ts
>>> roll Dragon
Dragon::
    fire_breath=[3, 5, 1, 4] +4 ->17
    stomp=[6, 4, 6] +2 ->18
```

Ability scores are an example of composition. When you run `roll abilities`, really the interpreter runs:

```python
>>> (
    let STR = max 4d6, 
    let DEX = max 4d6,
    let CON = max 4d6,
    let INT = max 4d6,
    let WIS = max 4d6,
    let CHA = max 4d6,
)
```

---

## Source files




