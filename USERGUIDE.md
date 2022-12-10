# rollang

> This is a HOW-TO guide for rollang. For details on the specification of the language (aimed at developers), see [ROLLANG.md](./ROLLANG.md)

> For DnD rules see the [DnD Beyond Introduction](https://www.dndbeyond.com/sources/basic-rules/introduction)

`rollang` is a language for DnD players to easily simulate dice rolls.

Start the rollang interpreter by typing `rollang` into a shell (like `Command Prompt`/`PowerShell` on Windows, `Bash` on Linux, or `Terminal` on Mac OS). This is the start of a `rollang` **session**

```bash
rollang
```

A `>>>` will show up. This is the prompt of the `rollang` interpreter.

```python
Rollang v0.1 8th Dec 2022
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

## Comments

C-style comments: `//`

Multiline comments are not supported, yet, though they will also use C-style `/* */`. However, you will be able to nest them.
