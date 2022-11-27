# rollang

A DSL and interpreter for DnD rolls. Inspired by [Roll20 Macro Language](https://wiki.roll20.net/Macro_Guide)

> This is just for fun. For actual DnD games, nothing beats real dice :)

# Usage

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

```python

```

## Operations

Addition

```python
>>> roll 1d20+4
1d20=6 +4 ->10
```

```python
>>> roll 1d20:"Fire"+4:"Buff"
roll 1d20:"Fire"=6 +4:"Buff" ->10
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

### **Operator Precedence**

1. Roll
2. Addition/Subtraction
3. Comparison

```python
>>> roll 1d20+4 > 9
1d20=7 +4 ->11 success
```

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

## Attack/Saving throws

### Armor Class

```python
>>> ac15
AC=15
```

```python
>>> ac15+2
AC=17
```

### Attack rolls

```python
atk <modifiers>
```

```python
>>> atk
1d20=5 pending
```

```python
>>> atk +4
1d20=5 +4 ->9 pending
```

```python
>>> atk >= ac15
1d20=12 failure
```

```python
>>> atk +4 +2 >= ac15
1d20=12 +4 +2 ->18 success
```

Natural one auto-fails

```python
>>> atk +100 >= ac15
1d20=1 +100 failure
```

Natural twenty auto-succeeds

```python
>>> atk -100 >= ac15
1d20=20 -100 success
```

### Difficulty Class

Same as Armor Class

```python
>>> dc15+2
DC=17
```

### Saving Throws

Saving throws function the same as attack rolls, but just have a different identifier

```python
>>> sav +4 +2 >= dc15
1d20=12 +4 +2 ->18 success
```

## Variables

Save rolls in variables using the `let` keyword

```js
>>> let my_attack = roll [2d8 1d4]
>>> my_attack
2d8=[3, 7] ->10
>>> my_attack+4
2d8=[3, 7] ->14
```

If you want to save but not evaluate the roll, then just bind a set of dice to an identifier

> For now, it's required to end the identifier name with a question mark to indicate it is probabilistic

```js
>>> let my_attack? = [2d8+4 1d4]
>>> my_attack?
2d8+4 1d4 pending
```

Then, when you want to evaluate the roll

```python
>>> roll my_attack?
2d8=[5, 2] +4 1d4=6 ->17
```

## Comments

C-style comments: `//`

Multiline comments are not supported.

## Source files

The `rollang` interpreter will execute the file line by line.

Only forward declarations are allowed.

`myfile.roll`