# rollang User Guide

`rollang` is a DnD dice rolling simulator.

> For DnD rules see the DnD Basic Rules [2014](https://www.dndbeyond.com/sources/basic-rules/introduction) or [2024](https://www.dndbeyond.com/sources/dnd/br-2024/playing-the-game).

## Why?

There are many existing dice rollers - even Google has a built in widget for dice rolling.
However, some are too simple and inflexible, and others have really complicated notation.

`rollang` provides a simple, yet flexible way to roll dice.

## TL;DR

Want to roll a d20?

```python
>>> r d20
16
```

Want to perform a saving throw?

```python
>>> r d20 > dc16 "DEX"
14 < DC16 failed DEX save
```

Or an attack roll?

```python
>>> r d20
20
```

Want to roll with advantage or disadvantage?

```python
>>> ra d20
<16> & 4
>>> rd d20
16 & <4>
```

Want multiple annotated damage rolls?

```python
>>> r 3d6+5 "Fireball" & 2d4 "Ray of Frost"
13  Fireball        [4 + 3 + 6]
3   Ray of Frost    [1 + 2]
```

Want to reuse rolls?

```python
>>> combo := 3d6+5 "Fireball" & 2d4 "Ray of Frost"
>>> r combo
8  Fireball        [2 + 5 + 1]
7  Ray of Frost    [3 + 4]
```

See below for more commands.

## Command-Line Iterface

Start the rollang interpreter by typing `rollang` into a shell (like `Command Prompt`/`PowerShell` on Windows, `Bash` on Linux, or `Terminal` on Mac OS). This is the start of a `rollang` **session**

```bash
rollang
```

A `>>>` will show up. This is the prompt of the `rollang` interpreter.

```python
Rollang v0.1.0
>>>
```

If you want to record a session, you can use

```bash
rollang --log PATH/TO/LOGFILE.txt
```

Rolls are random, but at the beginning of a session or `.roll` file (see below) you can set a seed to get a reproducible sequence of rolls:

```python
rollang --seed 1234
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

## Rolling

You can roll dice using the `r` command

Single die:

```lua
>>> r 1d20
7
```

To combine multiple dice rolls use `+`:

```lua
>>> r 3d4 + 2d6
12 <= (2+2+3) + (4+1)
```

Alternatively, perform multiple seperate rolls using `&`:

```lua
>>> r 3d4 & 2d6
7 <= (2+2+3)
5 <= (4+1)
```

### Annotations

```lua
>>> r 8d6 "Fire Ball"
28 Fire Ball <= (6+4+3+5+4+2+1+3)
```

### Modifiers

You can append a modifier, some positive or negative number, to dice.

```python
>>> r 2d10+4
14 <= (6+4)+4
```

### Advantage/Disadvantage

Rolling with advantage or disadvantage means rolling a second d20. With advantage means you take the higher result, disadvantage the lower.

> See [Basic Rules#AdvantageDisadvantage](https://www.dndbeyond.com/sources/basic-rules/using-ability-scores#AdvantageandDisadvantage)

Roll with Advantage

```js
>>> ra d20+5
7 | [16]
```

Roll with Disadvantage

```js
>>> rd d20+5r
[7] | 16
```

### Attack Rolls, Saving Throws, and Ability Checks

```js
>>> r 20+4 > ac17
22 <= (18)+4 success
```

```js
>>> r d20+4 > dc16 "WIS"
7 <= (3)+4 failure
```

```js
>>> r d20+4 "Athletics"
18 <= (14)+4
```

### Critical Hit

Critical hit calculations are based on standard 5e rules: double the amount of dice rolled, then add modifiers

```lua
>>> rcrit 3d8+4
38 <- 2*(8, 3, 6)+4
```

### Reroll

Reroll any twos

```lua
>>> rr(2) 4d12
```

Reroll any ones and twos

```lua
>>> rr(1, 2) 4d12
```

### Keep Highest

Keep the two highest rolls

```lua
>>> rkh(2) 4d12
```

### Keep Lowest

Keep the two lowest rolls

```lua
>>> rkl(2) 4d12
```

### Drop Highest

Drops the two highest rolls

```lua
>>> rdh(2) 4d12
```

### Drop Lowest

Drop the two lowest rolls

```lua
>>> rdl(2) 4d12
```

## Named Rolls

You can name rolls and results for reuse.

Create a reusable combo:

```python
>>> combo = 8d6 "Fireball" & 1d4 "Ray of Frost"
>>> r combo
8  Fireball        [2 + 5 + 1]
7  Ray of Frost    [3 + 4]
```

Save the result of a roll:

```python
>>> my_roll = r d20
>>> my_roll
7
```

Bind new value (notice `=` instead of `:=`)

```python
>>> my_roll = r d20
2
```

> There are some other rules/restrictions on binding names. Please see the [rollang specification](../dev/spec.md) for details
