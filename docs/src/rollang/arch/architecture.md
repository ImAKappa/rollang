# Architecture


## Usage

The language is implemented by the `rollang` interpreter. There is a REPL, and alternatively you can write code in a `.roll` script files.

The language makes it easy to:

* modify rolls,
* construct groups of rolls,

The interpreter is designed to do all the bookkeeping for you.

# Implementation

> Architecture for `rollang` interpreter
>
> 1. Lexer (string -> token) 
> 
> 2. Token Parser (token -> AST) 
>
> 3. AST Parser (AST -> ECS)
>
> 3. ECS Interpreter (run ECS)



Use an ECS-like framework to manage rolls

## Dice

Single die

```rust
>>> 1d20
Entities:   [0]
Amount:     [1]
Sides:      [20]
Rolls:      [Roll<Pending>]
```

Multiple dice:

```rust
>>> 2d8
Entities:   [0]
Amount:     [2]
Sides:      [8]
Rolls:      [Roll<Pending>]
```

Collection of multiple dice

```rust
>>> [1d20, 2d8]
Entities:   [0, 1]
Amount:     [1, 2]
Sides:      [20, 8]
Rolls:      [Roll<Pending>, Roll<Pending>]
```

Dice and modifiers

```rust
>>> [1d20+4, 2d8-2]
Entities:   [0, 1]
Amounts:     [1, 2]
Sides:      [20, 8]
Modifiers:  [+4, -2]
Rolls:      [Roll<Pending>, Roll<Pending>]
```

---

## Rolling

```rust
roll [<Rollable>+]
```

Runs the following system:

```rust
Results: [0, 0]

for id in Entities {
    // Roll
    roll_results = {
        match Rolls[id] {
            Pending => {
                let rolls = []
                // Amount
                for i in 1..=Amounts[id] {
                    // Sides
                    roll = random.int(1, Sides[id])
                    rolls.push(roll)
                }
            }
            Result(result) => result
        }
    }
    // Update Rolls
    Rolls[id] = Roll::Result(roll_results)
    // Add roll_results
    results = sum(roll_results)
    // Modifiers
    results += Modifiers[id]

}
```
