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
    COLON = ":"
    LPAREN = "("
    RPAREN = ")"
    LBRACKET = "["
    RBRACKET = "]"
    QUESTION = "?"

    # One or two character tokens
    EQUAL = "="
    EQUAL_EQUAL = "=="
    GREATER = ">"
    GREATER_EQUAL = ">="
    LESS = "<"
    LESS_EQUAL = "<="
    BANG = "!"
    BANG_EQUAL = "!="

    # Literals
    IDENTIFIER = "IDENTIFIER"
    SUCCESS = "SUCCESS" # True
    FAILURE = "FAILURE" # False
    NUMBER = "NUMBER"
    STRING = "STRING"
    DICE = "DICE"

    # Keywords
    ROLL = "ROLL"
    ADV = "ADV"
    DDV = "DDV"
    ATK = "ATK"
    SAV = "SAV"
    SEED = "SEED"
    LET = "LET"
    AC = "AC"
    DC = "DC"
    # END = "END"

    EOF = "EOF"

keywords = {
    "roll": TokType.ROLL,
    "adv": TokType.ADV,
    "ddv": TokType.DDV,
    "atk": TokType.ATK,
    "sav": TokType.SAV,
    "seed": TokType.SEED,
    "let": TokType.LET,
    "ac": TokType.AC,
    "dc": TokType.DC,
}