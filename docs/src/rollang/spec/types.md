# Types

### Result\<T\>

Result in `rollang` is a sum type with two variants, `Ok` or `Pend`

```rust
f := |x: Result<num>| -> x +4
```

> Maybe (Haskell) or Result (Swift). But instead of None, it handles Pending rolls.

```rust
Result<Pending>
Result<3>
Result<"Hello">
Result<[1, 2, 3]>
```

### Option\<T\>
