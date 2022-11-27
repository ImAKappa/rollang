#!/usr/bin/env python3
"""rollang.py

Main module for rollang
"""
# TODO: Pretty/Rich print program info
# ~TODO~: Cusomize prompt to look like die

from dataclasses import dataclass

from roller import Roller

@dataclass
class ProgramInfo:
    name: str
    version: str
    docs: str

program_info = ProgramInfo("Roller", "v0.1.0", "https://google.com")

prompt = "🎲 "

def main() -> None:
    while True:
        try:
            s = input(prompt)
        except EOFError:
            break
        else:
            print(s)
    return

if __name__ == "__main__":
    print(program_info)
    main()
