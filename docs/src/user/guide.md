# rollang User Guide

`rollang` is a language and roll simulator for DnD.

> For DnD rules see the [DnD Beyond Introduction](https://www.dndbeyond.com/sources/basic-rules/introduction)


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

You can roll dice using the `roll` command

Single die

```lua
>>> roll 1d20
7
```

Or you can use the bang command `r!`:

```rust
>>> r! 1d20
7
```

You can roll multiple die:

```lua
>>> r! 3d4 + 2d6
12 <- (2, 2, 3) + (4, 1)
```

### Annotated roll

```lua
>>> r!'Fire Ball' 2d8
```

## Modifiers

You can append a modifier, some positive or negative number, to dice.

```python
>>> r! 2d10+4
14 <- (6, 4)+4
```

> Don't include spaces by accident
>
> ```python
> >>> r! 1d20 +4
> Error: Unable to parse roll
> ```

## Roll Methods

### Advantage/Disadvantage

Rolling with advantage or disadvantage means rolling a second d20. With advantage means you take the higher result, disadvantage the lower.

> See [Basic Rules#AdvantageDisadvantage](https://www.dndbeyond.com/sources/basic-rules/using-ability-scores#AdvantageandDisadvantage)

Roll with Advantage

```js
>>> r!adv d20+5
7 | [16] <- (2)+5 | (11)+5
```

Roll with Disadvantage

```js
>>> r!dis d20+5
[7] | 16 <- (2)+5 | (11)+5
```

### Attack Rolls, Saving Throws, and Ability Checks

```js
>>> r!atk(17) d20+4
success <- (20)+4 > 17 AC 
```

```js
>>> r!sav(16, 'WIS') d20+4
fail <- (3)+4 < 16 WIS DC
```

```js
>>> r!chk(14, 'Athletics') d20+4
succeed <- (11)+4 > 14 Athletics DC
```

### Critical Hit

Critical hit calculations are based on standard 5e rules: double the amount of dice rolled, then add modifiers

```lua
>>> r!crit 3d8+4
38 <- 2*(8, 3, 6)+4
```

### Reroll

Reroll any twos

```lua
>>> r!rr(2) 4d12
```

Reroll any ones and twos

```lua
>>> r!rr(1, 2) 4d12
```

### Keep Highest

Keep the two highest rolls

```lua
>>> r!kh(2) 4d12
```

### Keep Lowest

Keep the two lowest rolls

```lua
>>> r!kl(2) 4d12
```

### Drop Highest

Drops the two highest rolls

```lua
>>> r!dh(2) 4d12
```

### Drop Lowest

Drop the two lowest rolls

```lua
>>> r!dl(2) 4d12
```

## Bindings

You can bind values, like a dice or number, to a name. This makes it easier to save and reuse results.

```rust
>>> my_roll := r! d20
>>> my_roll
7
```

Bind new value (notice `=` instead of `:=`)

```lua
>>> my_roll = r! d20
2
```

> There are some other rules/restrictions on binding names. Please see the [rollang specification](../dev/spec.md) for details

## Rollsets

Rollang supports collection of named rolls, called `Rollsets`.

There are built-in rollsets, but you can also define your own.

### Ability Scores

`ability_scores` is a default binding that generates ability scores according to the instructions in the [Basic Rules#DetermineAbilityScores](https://www.dndbeyond.com/sources/basic-rules/step-by-step-characters#3DetermineAbilityScores) on DnDBeyond

```js
>>> r!rr(1) dnd['Ability Scores']
STR 15
DEX 14 
CON 11 
INT 17 
WIS 10
CHA 12
```

### User-Defined Rollsets

```go
>>> party_initiative := {'Utankh': d20-1, 'Mythelia': d20+1, 'Boro-Boro': d20}
```

```lua
>>> r! party_initiative['Mythelia']
17 <- (16)+1
```

### Initiative

For initiative, provide a rollset of your players

```rust
>>> r!init(party_initiative)
Mythelia    16
Boro-Boro   12
Utankh      4
```

## Further Reading

There are more details in the [spec](..dev/spec), but this is mostly for developers!