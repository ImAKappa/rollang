#!/usr/bin/env python3
"""rolltoken.py

Module for defining tokens and token types for lexer
"""

from enum import Enum

class TokType(Enum):
    # Single character tokens
    MINUS = "-"
    PLUS = "+"
    COMMA = ","

    # One or two character tokens
    EQUAL = "="
    EQUAL_EQUAL = "=="
    GREATER = ">"
    GREATER_EQUAL = ">="
    LESS = "<"
    LESS_EQUAL = "<="
    BANG = "!"
    BANG_EQUAL = "!="
    LBRACKET = "["
    RBRACKET = "]"

    # Literals
    IDENTIFIER = "IDENTIFIER"
    SUCCESS = "SUCCESS" # True
    FAILURE = "FAILURE" # False
    NUMBER = "NUMBER"
    STRING = "STRING"
    DICE = "DICE"
    AC = "AC"
    DC = "DC"

    # Keywords
    ROLL = "ROLL"
    ADV = "ADV"
    DDV = "DDV"
    ATK = "ATK"
    SAV = "SAV"
    SEED = "SEED"
    LET = "LET"

    EOF = "EOF"

keywords = {
    "roll": TokType.ROLL,
    "adv": TokType.ADV,
    "ddv": TokType.DDV,
    "atk": TokType.ATK,
    "sav": TokType.SAV,
    "seed": TokType.SEED,
    "let": TokType.LET,
}