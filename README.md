# Pick the Right Stuff

This repo contains the source code used for the research:
***Zero, Finite, and Infinite Belief History of Theory of Mind Reasoning in Large Language Models***.

It consists of the following two parts:

1. **Code of *Pick the Right Stuff***: located in `game` directory, which contains the code for the game implementation.
2. **Code of Analyses**: located in `research` directory, which contains the code for the analysis experiments with the
   game.

## Game

The game, _**Pick the Right Stuff**_, is implemented in Rust with the *client-server* architecture.
The game server is responsible for generating and providing the game environment and the client is responsible for
playing the game.
The game server communicates with the client using the
TCP stream, so it is flexible for future work to use it to evaluate other LLMs or AI systems.

### Prerequisites

1. Install Rust and its tool chain, like `cargo`.
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

The code of analyses can be found in the `research` directory.

1. Install the required packages:

```bash
pip install -r requirements.txt
```

2. Put your OPENAI key in the `.env` file (For GPT-3.5 Turbo):

```
OPENAI_KEY=your_openai_key
```

3. Download [Ollama](https://ollama.com/) and pulls the models you want (refer to Ollama website/document).
4. Modify the `main` function in `main.py` to set appropriate settings for the experiments.
5. Run the experiments:

```bash
python main.py
```