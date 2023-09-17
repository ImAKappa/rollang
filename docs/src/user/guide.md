# rollang User Guide

`rollang` is a language and interpreter for DnD players to easily simulate dice rolls.

> For DnD rules see the [DnD Beyond Introduction](https://www.dndbeyond.com/sources/basic-rules/introduction)


## Command-Line Iterface

Start the rollang interpreter by typing `rollang` into a shell (like `Command Prompt`/`PowerShell` on Windows, `Bash` on Linux, or `Terminal` on Mac OS). This is the start of a `rollang` **session**

```bash
rollang
```

A `>>>` will show up. This is the prompt of the `rollang` interpreter.

```python
Rollang v0.1 12th Jan 2023
>>>
```

If you want to record a session, you can use

```bash
rollang --log PATH/TO/LOGFILE.txt
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

Create a collection of dice, called a **Rollset**

```lua
>>> {1d20, 2d4}
{1d20, 2d4}
```

Annotate dice with a string of text

```lua
>>> 2d4[Fire Ball]
2d4 Fire Ball
```

```lua
>>> {2d4[Fire Ball], 2d8[Lightning Strike]}
{2d4 Fire Ball, 2d8 Lightning Strike}
```

Rollsets can be annotated too

```lua
>>> {2d4[Fire Ball], 2d8[Lightning Strike]}[Combo]
{2d4 Fire Ball, 2d8 Lightning Strike} Combo
```

## Rolling

You can roll dice using the `roll` command

Single die

```lua
>>> roll 1d20
7 <- 1d20
```

```lua
>>> roll 1d20[Fire]
7 <- 1d20
```

Multiple separate dice rolls

```lua
>>> roll (3d4 2d6)
7 <- 3d4=[2, 2, 3]
5 <- 2d6=[4, 1]
```

Alternatively, you can combine rolls by using a Rollset

```lua
>>> roll {3d4, 2d6}[Combo]
12 Combo <- 3d4=[2, 2, 3] 2d6=[4, 1]
```

Which also means you can roll multiple Rollsets

```lua
>> roll ({3d4, 2d6} {4d6, 2d10})
12 <-  3d4=[2, 2, 3]     2d6=[4, 1]
23 <-  4d6=[4, 2, 5, 1]  2d10=[8, 3]
```


> Instead of typing `roll` over and over, you can use the _bang_-command `r!` like
>
> ```bash
> >>> r! d20
> 17 <- d20
> ```

## Seed

Rolls are random, but at the beginning of a session, `.roll` file (see below), or when running the interpreter, you can set a seed to get a reproducible sequence of random numbers:

```python
>>> seed 1234
```

## Advantage/Disadvantage

> See [Basic Rules#AdvantageDisadvantage](https://www.dndbeyond.com/sources/basic-rules/using-ability-scores#AdvantageandDisadvantage)

Roll with Advantage

```lua
>>> r!adv d20
7    <- 1d20
[16] <- 1d20
```

Roll with Disadvantage

```python
>>> r!ddv d20
[7] <- 1d20 
16  <- 1d20
```

> Note: This is nearly equivalent to
>
> ```bash
> >>> r! max(2 * d20)
> ```
>
> Except the `radv` command will return both roll results, and format the output

## Modifiers

You can append a modifier, some positive or negative number, to dice.

```python
>>> r! 1d20+4
14 <- 2d10=[6, 4] +4
```

Don't include spaces by accident

```python
>>> r! 2d20 +4
Error
```

```python
>>> r! 1d20[Ability Check]-4[Debuff]
2 <- 1d20[Ability Check]=6 -4[Debuf]
```

> See [Custom Modifiers]() section below to see how to define your own modifiers

## Roll macro

Specify the kind of roll

### Reroll

```lua
>>> r!rr(< 2) d20 // Rerolls any result less than 2
```

### Drop Highest

```lua
>>> r!dh(> 2) // Drops any result higher than 2
```

### Drop Lowest

```lua
>>> r!dl(< 2) // Drops any result higher than 2
```

### Adv & Ddv

```lua
>>> r!adv d20
>>> r!ddv d20
```

### Exploding Dice

```js
>>> r!xplode(> 16) d20 // Rerolls and adds sum if greater than 16
```

### DC & AC

```js
>>> r!sav(> 16) d20+4
```

```js
>>> r!atk(< 17) d20+4
```

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
>>> roll dnd.ability_scores
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


## Functions

Some built-in functions in `rollang` are shown below. For a full list, see the specification docs.

```python
>>> r! max(3d6)
3d6 =[3, 6, 2] max ->6
```

Another example is `sum`. This is implicitly called using the roll (`roll`, `r!`) method

```python
>>> sum(1, 2, 3)
6
```

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

Comments: `//`

Multiline comments `/* */`. They are nestable!
