#!/usr/bin/env python3
"""rollang.py

Main module for rollang
"""
# TODO: Pretty/Rich print program info
# ~TODO~: Cusomize prompt to look like die

import sys
args = sys.argv

from roller import ProgramInfo, Roller

if __name__ == "__main__":
    program_info = ProgramInfo("Roller", "v0.1.0", "https://google.com")
    roller = Roller(program_info, args)