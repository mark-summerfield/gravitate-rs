#!/bin/bash
tokei -f -c80 -tPython,Rust -etarget
git st
