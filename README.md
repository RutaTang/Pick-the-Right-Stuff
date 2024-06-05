# Pick the Right Stuff

This repo contains the source code used for the Research:
***Zero, Finite, and Infinite Belief History of Theory of Mind Reasoning in Large Language Models***.

It consists of the following two parts:

1. **Game**: located in `game` directory, which contains the code for the game implementation.
2. **Research**: located in `research` directory, which contains the code for the research experiments with the game.

## Game

The game, _**Pick the Right Stuff**_, is implemented in Rust with *client-server* architecture. The game server is
implemented in Rust and the client is implemented in Python. The game server is responsible for generating the game
environment and the client is responsible for playing the game. The game server communicates with the client using the
TCP stream.

### Prerequisites

1. Install Rust and its tool chains, like `cargo`.
2. Run `cargo run` will show the help/usage of the cli.

### Run Server

1. Run for Zero Belief History:

```bash
cargo run -- serve -m zero
```

2. Run for Finite Belief History:

```bash
cargo run -- serve -m finite
```

### Run Client

_**Run Server before running the client.**_

1. Run for Zero Belief History:

```bash
cargo run -- client -p 8080
``` 

2. Run for Finite Belief History:

```bash
cargo run -- client -p 8081
```


Now, it's ready to play the game!


## Research

The analysis is implemented in the `research` directory. You can directly run the experiments by run `main.py`. 
If you want to test different experiment settings, try to modify the `main` function in `main.py`. 