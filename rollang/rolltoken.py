#!/usr/bin/env python3
"""rolltoken.py

Module for defining tokens and token types for lexer
"""

from dataclasses import dataclass
from tokentype import TokType

@dataclass
class Token:
    toktype: TokType
    lexeme: str
    literal: object