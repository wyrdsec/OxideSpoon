# RISC V OS written in Rust

## Abstract

I really needed a project while I am grinding [slayer](https://oldschool.runescape.wiki/w/Slayer) in [osrs](https://oldschool.runescape.com/) so I feel like I am not wasting my time (I am).

The end goal is to create something like [xv6](https://en.wikipedia.org/wiki/Xv6) linux, with possibly some user/kernel space separation and 2d graphics support.

## References

Most of my code so far is copied from my old professor Dr. Stephen Marz in his [os blog](https://osblog.stephenmarz.com/).

### Quick links!

1. [https://osblog.stephenmarz.com](https://osblog.stephenmarz.com)
2. [https://os.phil-opp.com](https://os.phil-opp.com)
3. [https://github.com/skyzh/core-os-riscv](https://github.com/skyzh/core-os-riscv)
    - Man this person's way smarter than me
4. [Youtube Series by Dr. Harry H. Porter](https://www.youtube.com/watch?v=fWUJKH0RNFE&list=PLbtzT1TYeoMhTPzyTZboW_j7TPAnjv9XB)
    - *Avada-compiler*

## Requirements

1. Rust-nightly
2. qemu-system-riscv64