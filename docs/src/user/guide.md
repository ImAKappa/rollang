# rollang User Guide

`rollang` is a language and interpreter for DnD players to easily simulate dice rolls.

> For DnD rules see the [DnD Beyond Introduction](https://www.dndbeyond.com/sources/basic-rules/introduction)

Start the rollang interpreter by typing `rollang` into a shell (like `Command Prompt`/`PowerShell` on Windows, `Bash` on Linux, or `Terminal` on Mac OS). This is the start of a `rollang` **session**

```bash
rollang
```

A `>>>` will show up. This is the prompt of the `rollang` interpreter.

```python
Rollang v0.1 12th Jan 2023
>>>
```

## Dice

Create a die by specifying the `[number of dice]d[number of sides]`

```python
>>> 1d20
1d20
```

If the number of dice is `1`, you can omit it

```python
>>> d20
1d20
```

Create a collection of dice, called a **roll list**

```python
>>> [1d20, 2d4]
1d20 2d4
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

## Rolling

You can roll dice using the `roll` command

Single die

```python
>>> roll 1d20
1d20 =7
```

```python
>>> roll 1d20:"Fire"
1d20:"Fire" =7
```

Multiple separate dice rolls

```python
>>> roll (3d4 2d6)
3d4 =[2, 2, 3] ->7
2d6 =[4, 1] ->5
```

Alternatively, you can provide a roll list

```python
>>> roll [3d4, 2d6]
3d4 =[2, 2, 3] 2d6 =[4, 1] ->12
```

The result, specifically sum, of the roll is shown after the `->`. In the example above, the result is `12` because `2 + 2 + 3 + 4 + 1 = 12`

> Instead of typing `roll` over and over, you can use the short-form `r!` like
>
> ```bash
> >>> r! d20
> d20 =17
> ```

## Seed

Rolls are random, but at the beginning of a session, `.roll` file (see below), or when running the interpreter, you can set a seed to get a reproducible sequence of random numbers:

```python
>>> seed 1234
seed =1234
```

or 

```bash
rollang --seed 1234
```

## Advantage/Disadvantage

> See [Basic Rules#AdvantageDisadvantage](https://www.dndbeyond.com/sources/basic-rules/using-ability-scores#AdvantageandDisadvantage)

Advantage

```python
>>> r! d20:adv
1d20 =7
1d20 =16 <!-
```

Disadvantage

```python
>>> r! d20:dis
1d20 =7 <!-
1d20 =16
```

## Modifiers

You can append a modifier, some positive or negative number, to dice.

```python
>>> r! 1d20+4
1d20 =6 +4 ->10
```

If you want to include spaces between the dice and the modifier, you need to use parentheses

```python
>>> r! (1d20 +4)
1d20 =6 +4 ->10
```

```python
>>> r! (1d20:"Fire" -4:"Debuff")
1d20:"Fire" =6 -4:"Debuff" ->2
```

In `rollang`, modifiers extend beyond numbers and can be functions.

```python
>>> r! 3d6:max
3d6 =[3, 6, 2] max ->6
```

Another example is `sum`. This is implicitly called as the last modifier only if a die
has no other named modifiers

```python
>>> r! 3d6:sum
3d6 =[3, 6, 2] sum ->11
```

> See [Custom Modifiers]() section below to see how to define your own modifiers

## Bindings

You can bind values, like a dice, number, or result, to a name. This makes it easier to save and reuse results.

```rust
>>> my_binding := roll d20
>>> my_binding
7 
```

> ❗Note that you can't reassign a binding. If you want to reuse the same name, you have to write `name := roll d20` again. This will discard the previous value of `name`

> There are some other rules/restrictions on binding names. Please see the [rollang specification](../rollang/spec.md) for details

## Ability Scores

`ability_scores` is a default binding that generates ability scores according to the instructions in the [Basic Rules#DetermineAbilityScores](https://www.dndbeyond.com/sources/basic-rules/step-by-step-characters#3DetermineAbilityScores) on DnDBeyond

```python
>>> roll ability_scores
STR =15
DEX =14
CON =11
INT =17
WIS =10
CHA =12
```

## Armor Class / Difficulty Class

```python
>>> ac 15
AC =15
```

AC and DC can have modifiers, too

```python
>>> 15ac +2
AC=15 +2 ->17
```

As well as annotations

```python
>>> 15ac +2:"Spell effect"
AC=15 +2:"Spell effect" ->17
```

The same applies to difficulty class, but you use `dc`

```python
>>> 15dc+2
DC=15 +2 ->17
```

---

## Ability Checks

```python
>>> roll 3d6 > 6dc
3d6 =[2, 4, 2] ->7 ->pass
```

```python
>>> roll [3d6, 2d8] > 20dc
3d6=[2, 2, 1] 2d8=[6, 2] ->13 ->failure
```

```python
>>> roll 3d6 >= 6dc
3d6=[2, 2, 2] ->6 ->pass
```

Alternatively, you can just use a number instead of the `dc` command, as long as you include brackets and spaces:

```python
>>> roll 3d6 >= (6 - 2)
3d6=[2, 2, 2] ->6 ->pass
```

```python
>>> roll 3d6 >= 6ac-2
3d6=[2, 2, 2] ->6 ->pass
```

---

## Initiative

For initiative, you can specify the number of combat participants

```python
>>> roll (init 3)
Initiative
----------
(1) 1d20 =16
(2) 1d20 =12
(3) 1d20 =4
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

`atk` is just an alias for `1d20`

```python
>>> roll atk
1d20 =7
```

```python
>>> roll atk+4
1d20 =5 +4 ->9
```

```python
>>> roll (atk +4)
1d20 =5 +4 ->9
```

```cpp
>>> roll atk >= 15ac
1d20 =12 ->failure
```

```cpp
>>> roll (atk +4 +2) >= 19ac-3
1d20 =12 +4 +2 ->18 ->pass
```

Natural one auto-fails

```cpp
>>> roll (atk +100) >= 15ac
1d20 =1 +100 ->fail
```

Natural 20 auto-succeeds

```cpp
>>> roll (atk -100) >= 15ac
1d20 =20 -100 ->pass
```

### Saving Throws

Like attack rolls, `sav` is an alias for a d20

```python
>>> roll (sav +4 +2) >= 15dc
1d20=12 +4 +2 ->18 ->pass
```

---

## Composite Roll

Group rolls together into a `Composite Roll`

```rust
Dragon :: Comp(
    stats: roll ability_scores,
    fire_breath: 4d8+4,
    stomp: 3d6+2,
)
```

> Note the use of `::` instead of `:=` for defining compositions

```rust
>>> roll Dragon.fire_breath
Dragon
    .fire_breath =[3, 5, 1, 4] +4 ->17
```

Alternatively, roll all dice:

```ts
>>> roll Dragon
Dragon
    .fire_breath =[3, 5, 1, 4]  +4   ->17
    .stomp       =[6, 4, 6]     +2   ->18
```

Ability scores are an example of composition. When you run `roll abilities`,
behind the scenes the interpreter runs:

```ts
>>> roll Comp(
    STR: 4d6:max, 
    DEX: 4d6:max,
    CON: 4d6:max,
    INT: 4d6:max,
    WIS: 4d6:max,
    CHA: 4d6:max,
)
```

---

## Source files

You can write your rollang program in a plain-text file ending in a `.roll` extension.

```rust
// tutorial.roll
r! ability_scores
r! d20 > 15ac
```

## Comments

C-style comments: `//`

C-style multiline comments `/* */`. They are nestable!
