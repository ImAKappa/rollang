# Data Structures

### Number

There are no decimal numbers in DnD 5e (as far as I know), so `Number`'s are synonymous with "Integer"

Basic operations:

- addition (`+`)
- subtraction (`-`)
- multiplication (`*`)
- floor division (`/`) because you usually round down in DnD
  - There is the `ceildiv` function if you need it

A `number` can be any value from $-2^{64-1}$ to $2^{64-1}$

### Bool

Same behaviour as other languages, different labels. "Pass" and "fail" are more often used in DnD parlance.

```ts
pass // true
fail // false
```

### String

Strings of text are surrounded by `""`. There is no distinction between characters and strings.

```rust
"Investigation check" // string

len("Hello") // 5
```

### Arrays: [T]

Array is a container that can only contain the same type

```ts
[1, 12, -4] // [number]

["Hello", "World"] // [string]

len([0, -3, 7]) // 3
```

