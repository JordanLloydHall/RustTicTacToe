# Rust Tic-Tac-Toe
## Motivation
Rust is becoming an increasingly popular programming language prized for its speed, powerful type system and zero-cost abstractions. I wanted to do an investigation into the languages native patterns. Usually when programming in a new language I enjoy programming a Tic Tac Toe engine with a Minimax algorithm for Player-vs-AI demoing,

## Introduction
The game of tic tac toe hopefully requires no introduction, however it is worth noting that both the engine and the AI will work for a board of arbitrary size. Minimax is a recursive model-based algorithm for minimizing loss in a turn-based zero-sum game. The variant used in this project is minimax with alpha-beta pruning, and a few memoization techniques designed to bring down the time complexity of an otherwise expensive algorithm.  

## Technology
The game engine and the AI was written entirely in the Rust programming languge. The Rayon cargo package allowed for a drop-in parallelized replacement for iterators.

The AI is a parallel Minimax with alpha-beta pruning and a transposition table for memoization. This technqiue allows the AI to run up to a 4-by-4 Tic Tac Toe board in a reasonable amount of time.

## Getting Started
If you want to get the game running immediately or wish to tinker around with the code, this is the place to start!

### Prerequisites
Getting the code to run is a simple as installing the Rust compiler and then staring the program.

    $ curl https://sh.rustup.rs -sSf | sh
    $ cargo run --release
    

## What I learned
I have learned a lot about Rust and its idiosyncrasies. I have found that the powerful type system makes working in rust very pleasant, and its functional style carries over very easily from Haskell. Rust is also incredibly fast for a language which is as easy to develop in. It feels like writing Kotlin or Haskell, but with the speed of C. As a result, this was the least amount of time I have ever spent making a Tic Tac Toe engine/AI and is also the fastest that it's ever been. I am more than pleased with the result.

However, Rust is slow to compile. If you've played around with this project, you may have found that compiling takes 10 seconds, and my larger Rust projects take up to 3 minutes to compile on my machine. This is something that the Rust team have been working on endlessly, and I expect some speedups in the future.

## Authors

* **Jordan Hall** - *Project Developer*