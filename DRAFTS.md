# Drafts

According to the [Basic Rules: Using Ability Scores](https://www.dndbeyond.com/sources/basic-rules/using-ability-scores),
there are different kinds of rolls:

## Group Checks

```F#
let groupcheck = roll 3d20
```

But what if you want to apply individual modifiers to each roll?

```go
groupcheck := (
    uthal= d20+2,
    mycin= d20+4,
    gold= d20+3:adv,
)
```

Access rolls

```F#
groupcheck.mycin
```

## Extending Behaviour


`rollang` is essentially a framework for modifying a single function, `roll`.

* The general **input** is some rollable entity with an arbitrary set of components. `Entity(number)<Rollable, ...>`
* The `roll` function is updated to combine all the defined components.
* The **output** is a list of numbers and the sum of that list: `Roll<Result(number[]), Sum(number)>`.




Users should be able to create:

1) Custom Components to add data to rolls

* Components with values

```ts
// Just bind values normally
Poison :: Rollable
```

* Flag components

```ts
flag 
```

2. Update the `roll` function with custom handlers. Users should be able to:

* reorganize the sequence of handlers
* combine results of different handlers

as long as the final result is a **list of numbers**

There are a two kinds of handlers:

1. **Within-Entity Handler**: These handlers modify the value of rolls within entities
2. **Cross-Entity Handler**: These handlers perform some operation on the rolls of multiple different entities



```ts
// Example: A system for rolling entities with the "poisoned" component
system poison(entity) with (Poison) =
    entity + Poison.dmg
```

```F#
let player = (
    let health = 20,
    let dmg = 2d6+4 
)


roll player:Poison(dmg: )
```

---

Implementing advantage

Define a component

```rust
comp adv
```

```rust
with (adv) {
    max(roll entity, roll entity)
}
```

Users need a way to hook into the result of a roll

```ts
roll d20:adv
```

Since the program focuses on rolling, any component should modify the result

Maybe we can treat it like Python, where the "self" object is used in the method declaration



```ts
system roll(entities: Rollable) -> (result := 0) {
    
}
```

```ts
>>> debug roll
System
```

```
update(roll)

```ts
update(roll) with (adv: Component) -> (result := 0) {
    roll
}
```
