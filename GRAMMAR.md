# rollang Grammar

RegEx is based on the rules from [https://regexr.com/](https://regexr.com/)

```C
program:
    | statement

statement:
    | expr
    | "let" IDENTIFIER "=" expr
    | expr modifier*
    | expr modifier* COMPARISON INT

expr:
    | dice|die
    | "roll" dice|die
    | "adv"
    | "ddv"
    | "atk"
    | "sav"
    | "seed" "(" INT ")"
    | "ac" "(" INT ")"
    | "dc" "(" INT ")"
    | RESULT

modifier:
    | ("+"|"-") NUMBER
    | ("+"|"-") NUMBER annotation

dice:
    | "[" (die ",")+ "]"

die:
    | INT "d" INT
    | INT "d" INT annotation

annotation:
    | ":" STRING

INT: [0-9]+

STRING: "\"" [.|"\n"]+ "\""

// Identifiers can't start with numbers, can end with a question mark
IDENTIFIER: [a-zA-Z$_][a-zA-Z0-9$_]*[?]?

COMPARISON: ">"|">="|"<"|"<="|"=="|"!="

RESULT: "success"|"failure"
```