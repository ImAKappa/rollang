#!/usr/bin/env python3
"""scanner.py

Scanner class and ScannerError.
The Scanner converts the source code into a list of Tokens

TODO: Add support for multi-line comments
"""
# Stdlib

# App
from error import Error
from tokentype import TokType, keywords
from rolltoken import Token
# Logs
import logging
from utils.ezlog import new_logger
log = new_logger(__name__)
log.setLevel(logging.DEBUG)

class ScannerError(Error):
    """Base-class for scanning errors"""
    def __init__(self, line: int, message: str):
        super.__init__(message)
        self.line = line

class Scanner:
    """Scans source code and converts it into a list of Tokens"""
    
    def __init__(self, source: str):
        self.source = source
        self.tokens: list[Token] = []
        self.start = 0
        self.current = 0
        self.line = 1

    def scan_tokens(self) -> list[Token]:
        while not self.is_at_end():
            self.start = self.current
            self.scan_token()

        self.tokens.append(Token(TokType.EOF, "", None, self.line))
        return self.tokens

    def scan_token(self) -> None:
        c = self.advance()
        match c:
            # Single-character tokens
            case "-":
                self.add_token(TokType.MINUS)
            case "+":
                self.add_token(TokType.PLUS)
            case ",":
                self.add_token(TokType.COMMA)
            # One or two character tokens
            case "=":
                self.add_token(TokType.EQUAL_EQUAL if self.match("=") else TokType.EQUAL)
            case "!":
                self.add_token(TokType.BANG_EQUAL if self.match("=") else TokType.BANG)
            case ">":
                self.add_token(TokType.GREATER_EQUAL if self.match("=") else TokType.GREATER)
            case "<":
                self.add_token(TokType.LESS_EQUAL if self.match("=") else TokType.LESS)
            # Comments (//)
            case "/":
                if self.match("/"):
                    while self.peek() != "\n" and not self.is_at_end():
                        self.advance()
            # Whitespace
            case " " | "\r" | "\t":
                pass
            # Newline
            case "\n":
                self.line += 1
            # Strings
            case "\"":
                self.string()
            # Literals
            # Keywords
            case _:
                # Number
                if c.isdigit():
                    self.number_or_dice()
                elif self.is_alpha(c):
                    self.identifier()
                else:
                    raise ScannerError(self.line, f"Unexpected character {c}")
        return

    def advance(self) -> str:
        c = self.source[self.current]
        self.current += 1
        return c

    def match(self, expected: str) -> bool:
        if self.is_at_end(): return False
        if self.source[self.current] != expected: return False
        self.current += 1
        return True

    def peek(self) -> str:
        if self.is_at_end():
            return "\0"
        return self.source[self.current]

    def peek_next(self) -> str:
        if self.current + 1 >= len(self.source):
            return "\0"
        return self.source[self.current + 1]

    def is_at_end(self):
        return self.current >= len(self.source)

    def add_token(self, type: TokType, literal: object = None) -> None:
        text = self.source[self.start:self.current]
        self.tokens.append(Token(type, text, literal, self.line))

    def string(self) -> None:
        while self.peek() != "\"" and not self.is_at_end():
            if self.peek() == "\n":
                self.line += 1
            self.advance()
        
        if self.is_at_end():
            raise Scanner(self.line, "Unterminated string.")
        
        # The closing ".
        self.advance()

        # Trim the surrounding quotes
        value = self.source[self.start+1:self.current-1]
        self.add_token(TokType.STRING, value)
        return

    def number_or_dice(self) -> None:
        while self.peek().isdigit():
            self.advance()
        value = self.source[self.start:self.current]
        # Check if die
        if self.peek() == "d" and self.peek_next().isdigit():
            # Consume the "d"
            self.advance()
            self.dice()
            return
        self.add_token(TokType.NUMBER, int(value))
        return

    def dice(self) -> None:
        while self.peek().isdigit():
            self.advance()
        self.add_token(TokType.DICE, self.source[self.start:self.current])
        return

    def is_alpha(self, c: str):
        return c.isalpha() or c == "_"

    def is_alphanumeric(self, c: str):
        return self.is_alpha(c) or c.isdigit()
    
    def identifier(self) -> None:
        while self.is_alphanumeric(self.peek()):
            self.advance()
        text = self.source[self.start: self.current]
        type = keywords.get(text, None)
        if type is None:
            type = TokType.IDENTIFIER
        self.add_token(type)
        return
        