# rollang Spec

# rollang Spec

# rollang

> This is the specification of the language

> For DnD rules see the [DnD Beyond Introduction](https://www.dndbeyond.com/sources/basic-rules/introduction)

`rollang` is a language for DnD players to easily simulate dice rolls.

---

## Usage

The language is implemented by the `rollang` interpreter. There is a REPL, and alternatively you can write code in a `.roll` script files.

The language makes it easy to:

* modify rolls,
* construct groups of rolls,

The interpreter is designed to do all the bookkeeping for you.

## Pieces

Instead of a sophisticated, generic, and extensible type system (I'm not smart or experienced enough in language design to pull that off), `rollang` uses a simple Entity Component System to handle more complex data manipulations.

> Because the word `comp` is reserved for `Composite Roll`s, I will use Pieces instead of "Components". But in this context they mean the same thing.

```rust
Roll :: pcs {
    Result<[number]>,  // No need to write Result: Result
    Sum(Result<number>)
}
// "A Roll is made of the pieces: possible number array called Result, possible number called Sum"
```

```rust
Rollable :: pcs {
    Amount(number),
    Sides(number),
    Roll, // No need to write Roll: Roll
}
```

`Rollable` is essentially the definition for the most important thing in `rollang`: dice!

## Dice

For convenience, and consistency with DnD 5e, you can manipulate `Rollable`s with the syntax `[amount]d[sides]`

```ts
1d20
// if [amount]=1, then you can omit it 
d20
/* Rollable
 *      Amount(1)
 *      Sides(20)
 *      Roll<Pending>
 */
```

To roll a dice, use the built-in procedure `roll`:

```ts
roll 3d8
// Roll: Result([6, 2, 3]), Sum(11)
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



Annotate dice with a string of text

```python
>>> 2d4:"Fire attack"
2d4:"Fire attack"
```

```python
>>> [2d4:"Fire attack", 2d8:"Lightning"]
2d4:"Fire attack" 2d8:"Lightning"
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

## Modifiers

You can append a modifier, some positive or negative number, to dice.

```python
>>> roll 1d20+4
1d20=6 +4 ->10
```

If you want to include spaces between the dice and the modifier, you need to use parentheses

```python
>>> roll (1d20 +4)
1d20=6 +4 ->10
```

```python
>>> roll (1d20:"Fire" -4:"Debuff")
roll 1d20:"Fire"=6 -4:"Debuff" ->2
```
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




