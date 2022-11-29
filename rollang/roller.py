#! /usr/bin/env python3
"""roller.py

`Roller` is the interpreter for rollang
"""
import io
from pathlib import Path
import sys
from dataclasses import dataclass

from scanner import Scanner

@dataclass
class ProgramInfo:
    name: str
    version: str
    docs: str

class Roller:

    def __init__(self, program_info, args):
        self.program_info = program_info
        self.prompt = "🎲 "
        self.had_error = False

        if len(args) > 2:
            print("Usage: roller [script]")
        elif len(args) == 2:
            self.run_file(args[1])
        else:
            self.run_prompt()

    def run_file(self, file: Path) -> None:
        with io.open(file, mode="r") as f:
            source = f.read()
        self.run(source)

        if self.had_error:
            sys.exit(65)
        return NotImplemented

    def run_prompt(self) -> None:
        print(self.program_info)
        while True:
            try:
                line = input(self.prompt)
            except EOFError:
                break
            else:
                self.run(line)
                self.had_error = False
        return

    def run(self, source: str) -> None:
        scanner = Scanner(source)
        tokens = scanner.scan_tokens()

        # For now, just print the tokens
        for token in tokens:
            print(token)        
        return NotImplemented

    def error(self, line: int, message: str) -> None:
        self.report(line, "", message)
    
    def report(self, line: int, where: str, message: str):
        print(f"[line {line}] Error {where}: {message}")
        self.had_error = True