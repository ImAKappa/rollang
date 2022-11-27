#!/usr/bin/env python3
"""rolltoken.py

Module for defining tokens and token types for lexer
"""

from enum import Enum, auto

class TokType(Enum):
    # Single character tokens
    MINUS = auto()
    PLUS = auto()
    COMMA = auto()

    # One or two character tokens
    EQUAL = auto()
    GREATER = auto()
    GREATER_EQUAL = auto()
    LESS = auto()
    LESS_EQUAL = auto()

    # Literals
    IDENTIFIER = auto()
    NUMBER = auto()
    DIE = auto()
    AC = auto()
    DC = auto()

    # Keywords
    ROLL = auto()
    ADV = auto()
    DDV = auto()
    ATK = auto()
    SAV = auto()
    SEED = auto()