# rollang

A DSL and interpreter for DnD rolls. Inspired by [Roll20 Macro Language](https://wiki.roll20.net/Macro_Guide)

> This is just for fun. For actual DnD games, nothing beats real dice :)


<details><summary>Implementation Notes</summary>
The interpreter is currently written in Python because that's what I know best at the moment. But I wanna switch to a compiled language soon, so realistically the final interpreter will probably be written in Nim. Although, a functional language might be a better fit, like OCaml or F#.
</details>

## TODO

- [X] Scanner
- [ ] Parser
- [ ] Interpreter
- [ ] Byte-code interpreter???
- [ ] Switch implementation language to one of Nim, OCaml, or F#

---

## Interpreter

```bash
python .\rollang\rollang.py
```

To exit on Windows, press `Ctrl-z`.
Alternatively, type `exit` (doesn't work yet).

---

## Source files

The `rollang` interpreter will execute the file line by line.

Only forward declarations are allowed.

Save your file with a `.roll` extension, e.g. `myfile.roll`

---

## Dice

Single die

```python
>>> 1d20
1d20
```

Multiple dice

```python
>>> [1d20, 2d4]
1d20 2d4
```

Annotate dice

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

You can evaluate dice using the `roll` command.

Single die

```python
>>> roll 1d20
1d20=7
```

```python
>>> roll 1d20:"Fire"
1d20:"Fire"=7
```

Multiple dice

```python
>>> roll [3d4, 2d6]
3d4=[2, 2, 3] 2d6=[4, 1] ->12
```

---

## Seed

At the beginning of the session, `roll` file (see below), or when running the interpreter, you can set a seed for reproducibility:

```python
>>> seed(1234)
seed=1234
```

or 

```bash
roller --seed 1234
```

---

## Operations

Addition

```python
>>> roll 1d20+4
1d20=6 +4 ->10
```

```python
>>> roll 1d20:"Fire"-4:"Debuff"
roll 1d20:"Fire"=6 -4:"Debuff" ->10
```

Comparison

```python
>>> roll 3d6 > 6
3d6=[2, 4, 2] ->7 success
```

```python
>>> roll [3d6, 2d8] > 20
3d6=[2, 2, 1] 2d8=[6, 2] ->13 failure
```

```python
>>> roll 3d6 >= 6
3d6=[2, 2, 2] ->6 success
```

---

### **Operator Precedence**

1. Roll
2. Addition/Subtraction
3. Comparison

```python
>>> roll 1d20+4 > 9
1d20=7 +4 ->11 success
```

---

## Initiative

```python
>>> roll init 3d20
Initative
---------
1: 1d20=16
2: 1d20=12
3: 1d20=4
```

---

## Advantage/Disadvantage

Advantage

```python
>>> adv [1d6, 2d4]
1d6=4 2d4=[2, 3] ->9
1d6=5 2d4=[1, 4] ->10 <<<
```

Disadvantage

```python
>>> ddv [1d6, 2d4]
1d6=4 2d4=[2, 3] ->9 <<<
1d6=5 2d4=[1, 4] ->10
```

---

## Attack/Saving throws

### Armor Class

```python
>>> ac(15)
AC=15
```

```python
>>> ac(15)+2
AC=15 +2 ->17
```

```python
>>> ac(15)+2:"Spell effect"
AC=15 +2:"Spell effect" ->17
```

### Attack rolls

`atk` is just a semantic shorthand for `roll 1d20`

```python
>>> atk
1d20=7
```

```python
>>> atk+4
1d20=5 +4 ->9
```

```cpp
>>> atk >= ac(15)
1d20=12 failure
```

```cpp
>>> atk +4 +2 >= ac(15)
1d20=12 +4 +2 ->18 success
```

Natural one auto-fails

```cpp
>>> atk +100 >= ac(15)
1d20=1 +100 failure
```

Natural twenty auto-succeeds

```cpp
>>> atk -100 >= ac(15)
1d20=20 -100 success
```

### Difficulty Class

Same as Armor Class, but use `dc`

```cpp
>>> dc(15)+2:"Spell effect"
DC=15 +2:"Spell effect" ->17
```

### Saving Throws

Saving throws function the same as attack rolls, but just have a different identifier for semantic convenience

```cpp
>>> sav +4 +2 >= dc(15)
1d20=12 +4 +2 ->18 success
```

---

## Variables

Save rolls in variables using the `let` keyword

```rust
>>> let divine_attack = roll [2d8, 1d4]
>>> divine_attack
2d8=[3, 7] ->10
>>> divine_attack+4
2d8=[3, 7] ->14
```

If you want to save, but not evaluate, the roll then just bind a set of dice to an identifier

> For now, it's required to end the identifier name with a question mark to indicate it is probabilistic.
>
> This is analogous to how some languages use question marks to explicitly denote possible null types (TypeScript, Kotlin, etc.). However, in `rollang`, there is no such thing as a `null` type because variables cannot be `null`.

```rust
>>> let flame_strike? = [2d8+4, 1d4]
>>> flame_strike?
2d8+4 1d4
```

> The goal is to have automatic type inference, but I dunno how to do that yet.

Then, when you want to evaluate the roll

```python
>>> roll flame_strike?
2d8=[5, 2] +4 1d4=6 ->17
```

Also, variable names cannot start with a number, and must only contain letters, numbers, and underscore (currently not enforced, but working on it). They will also be case-sensitive.

---

## Comments

C-style comments: `//`

Multiline comments are not supported, yet, though they will also use C-style `/* */`. However, you will be able to nest them.
