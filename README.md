# MOScore

## Introduction

Welcome to MOScore! This project is currently under development and not ready for use. It's a Rust crate for creating emulators for 6502-based systems.

## Project Goals

The primary goal of MOScore is to enable users of the crate to craft emulators for a variety of systems. We aim to achieve this by providing a complete, thoroughly-tested, and *mostly* cycle-accurate implementation of the 6502 that can accept custom memory mappings and peripherals which act as the "system bus".

>[!NOTE]
>During development, efforts are being made to keep the clock cycle counts accurate. The unit tests for instructions verify that each instruction sends the correct number of clock signal pulses to the bus during execution. However, there may be discrepancies in regards to where these pulses occur within the instruction execution steps. Once the instruction set implementation is complete, work is planned to address this and ensure as much accuracy as we are capable of providing.

As development progresses and the project nears completion, we plan to provide a detailed manual on how to develop an emulator with MOScore, as well as an example reference project.

## Current State

As of now, the project is in its early stages of development. Here's a brief overview of what we have so far:

- Core functionality being developed in `src/core.rs`.
- `Bus` trait within the `src/traits.rs`, for creating custom memory maps.
- `DefaultBus` is provided as an example/default memory map, which splits the 64K address space evenly between RAM and ROM.
- A suite of unit tests under `src/core/tests` to ensure reliability and correctness of the core components.

If you're curious about the current progress, checking out the files in `src/core/tests/` is a good barometer for which instructions are "complete".

## The Name

I wanted to pay homage to MOS Technologies while emphasizing that this project is not a full emulator itself, but a "core" for building other emulators.
