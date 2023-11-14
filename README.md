# Pargo

## Overview

Like `cargo` but for playing with pdp-11 assembly and simulators.

Describe your env in a `Pargo.toml` file, place your assembly source files in the `src` directory
and run `pargo run` to build and load the code in the simulator.

It depends on:

* Richard Krehbiel's `macro11` cross-assembler
* [SimH](https://github.com/simh/simh)

## Goal

The goal of the project is mostly educational.

I'd like to pretend I'm living in an alternate timeline where PDP-11 is still a thing but
I don't want to give up to all the amenities of modern software development.

* I don't want to fiddle with makefiles.
* Make it easy for other people to just reproduce my builds
* Focus on system (embedded) PDP-11 programming. i.e. not building binaries for OSes such as RSX-11, but more focusing on building full OS-es for it

Non-goals:

* building and running legacy PDP-1 software. For that there is plenty of stuff out there.

## Roadmap

* `pargo build`
* `pargo run`
* `pargo test`: run automated tests in simulated env
* `pargo fmt`: format the assembly sources!

In addition to the build tool I'd also like to explore more tooling:

* Reimplement a macro assembler. A bit for it's own sake but also because I'm a bit frustrated about the UX of macro11.
* Reimplement a simulator. Just for fun