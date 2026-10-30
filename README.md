# SigmaOS terminal

This repository reserves the terminal component boundary in the SigmaOS
multi-repository architecture. It currently contains no terminal implementation;
the engine and language remain unclassified until source code establishes them.

## Contract

`COMPONENT.toml` records the component responsibility, integration repository,
branch, and scaffold status. `NOASSERTION` marks that this scaffold does not
declare a license. The contract validator checks these fields on pull requests
and pushes to `main`.

## Implementation status

Status: scaffold. Before implementation, define the terminal protocol and API,
identify upstream behaviors being adopted, document security and compatibility
tradeoffs, and add focused tests with the implementation.
