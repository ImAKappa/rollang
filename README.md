# rollang

A DSL and interpreter for DnD rolls. Inspired by [Roll20 Dice Reference](https://wiki.roll20.net/Dice_Reference) and [Avrae Discord bot](https://avrae.io/commands)

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

Maybe everything should be a function?

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

