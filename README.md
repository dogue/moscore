# Moscore

## Introduction

Welcome to Moscore! This project is currently under development and not ready for use. It's a Rust crate for creating emulators for 6502-based systems.

## Project Goals

The primary goal of Moscore is to enable users of the crate to craft emulators for a variety of systems. We aim to achieve this by providing a complete, thoroughly-tested, and *mostly* cycle-accurate implementation of the 6502 that can accept custom memory mappings and peripherals which act as the "system bus".

As development progresses and the project nears completion, we plan to provide a detailed manual on how to develop an emulator with Moscore, as well as an example reference project.

## Current State

As of now, the project is in its early stages of development. Here's a brief overview of what we have so far:

- Core functionality being developed in `src/core.rs`.
- `Bus` trait within the `src/traits.rs`, for creating custom memory maps.
- `DefaultBus` is provided as an example/default memory map, which splits the 64K address space evenly between RAM and ROM.
- A suite of unit tests under `src/core/tests` to ensure reliability and correctness of the core components.

If you're curious about the current progress, checking out the files in `src/core/tests/` is a good barometer for which instructions are "complete".

## Contributing

While Moscore is still in the building phase, we welcome contributions and ideas from the community. Feel free to fork the repository, submit issues, or propose pull requests.

## Disclaimer

Please note that Moscore is still in development and is not ready for use. The current codebase may undergo significant changes.

## The Name

I wanted to pay homage to MOS Technologies while emphasizing that this project is not an emulator itself, but a "core" for building other emulators.
