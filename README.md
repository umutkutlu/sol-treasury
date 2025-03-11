## Project Overview

The treasury game is a Solana program written with Anchor that implements a simple gambling mechanism:

1. Players deposit SOL into the treasury. They get a chance depending on the SOL amount.
2. The game owner determines winners and losers using a random input.
3. If a player wins, they receive the treasury they deserved.
4. If a player loses, their deposited amount is added to the treasury.

## Configuration

The fuzzing parameters are configured in the project setup.
- Exit upon crash = true
- Number of iterations: 1000
