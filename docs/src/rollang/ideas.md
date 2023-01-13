# Draft Ideas

# Drafts

According to the [Basic Rules: Using Ability Scores](https://www.dndbeyond.com/sources/basic-rules/using-ability-scores),
there are different kinds of rolls:

## Group Checks

You could roll multiple d20s in one command.

```
groupcheck := 3d20
```

But what if you want to apply individual modifiers to each roll?

```go
groupcheck := (
    uthal := d20+2,
    mycin := d20+4,
    gold := d20+3:adv,
)
```

In this case, uthal's, mycin's, and gold's rolls are scoped to `groupcheck`.

```F#
groupcheck.mycin
```

## Extending Behaviour of Rolls


`rollang` is essentially a framework for modifying a single function, `roll`.

1. The general **input** is a set of `Rollable` entities. You bind Modifiers to each input
2. The Modifiers are functions that modify the result of the roll. You chain together modifiers to acheive different roll affects
2. The **output** is a list of numbers and the sum of that list: `Roll<Result(number[]), Sum(number)>`


## Example

Let's say you want to modify rolls with some kind of random `Poison` debuff.

You would create a `Poison` modifier like this:

```nim
Poison :: mod(d: Roll<Pending>) {
    result = result - (roll d)
}
```

With explanations

```nim
# `Poison` is the identifier
# `mod` means `modifier`
# A mod can accept a paraemeter that fullfuills a `Query`
Poison :: mod(d: Roll<Pending>) {
    # Every modifier implicitly has `result: [int]` parameter
    # Similarly, a modifier will implicitly return `result: [int]`
    result = result - (roll d)
    # You can't recursively use mods
    #   So no `result = result - (roll d:Poison(d4))
}
```

And use it like this:

```nim
roll 1d20:Poison(d4)
```

---

Another example, this time we implement advantage:

```nim
adv :: mod() {
    second_roll := (roll d20)
    result = max (result, second_roll)
}
```

```ts
roll d20:adv
```

Basically, the idea is the user of `rollang` decides how to manipulate the results of a roll, while the interpreter does the bookkeeping for you.


[Micro features of Programming languages](https://buttondown.email/hillelwayne/archive/microfeatures-id-like-to-see-in-more-languages/)
