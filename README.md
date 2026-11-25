# Cells

A simple [life-like cellular automaton](https://en.wikipedia.org/wiki/Life-like_cellular_automaton) simulator.

## Demo

![A gif animation with a demo of Cells](./assets/demo.gif)

## Features

- [x] Pausing and resuming simulation
- [ ] Step-by-step simulation
- [x] Loading initial cells configuration from a file
- [x] Setting rules via command line option
- [ ] Setting rules via a header line in file
- [ ] Ability to toggle cell state via a mouse click
- [ ] Dumping current state to a file

## Usage

```bash
cargo run
```

### Choosing simulation rules

Rules can be passed as a string in Golly/RLE format. Defaults to Conway's Game of Life 'B3/S23'.

```bash
cargo run -- --rules 'B2/S23'
```

Rules' [notation](https://en.wikipedia.org/wiki/Life-like_cellular_automaton#Notation_for_rules):

> In the notation used by the Golly open-source cellular automaton package and in the RLE format for storing cellular automaton patterns, a rule is written in the form By/Sx where x and y are the same as in the MCell notation. Thus, in this notation, Conway's Game of Life is denoted B3/S23. The "B" in this format stands for "birth" and the "S" stands for "survival". Thus, B3/S23 states that:
> If a cell is dead, and has 3 neighbors, then the cell is born in the next tick
> If a cell is alive, and has 2 or 3 neighbors, then the cell survives to the next tick
> Otherwise, a cell becomes empty in the next tick.

### Providing initial cells configuration

`cells` can load initial automaton state from file containing `H` lines of `0` and `1`. Every line must contain the same number of characters `W`.
An automaton will be initialized with a grid of `H` height, `W` width, alive cells in positions of `1` and dead cells in positions of `0`.

See [examples/blinker.cells](./examples/blinker.cells) for example.

```bash
cargo run -- --cells-file examples/blinker.cells
```

### Choosing different color for cells

`cells` uses [ratutui Color](https://docs.rs/ratatui/latest/ratatui/prelude/enum.Color.html).
Any valid Color string representation can be used to customize cell color.

```
# ANSI color
cargo run -- --cells-color green
# RGB color
cargo run -- --cells-color #FF0000
# 8-bit color
cargo run -- --cells-color 91
```
