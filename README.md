# leucippus

## Run, build, tests, ...

### Contribution

Before contribution you should run the linter, make sure tests are green and
format the code:

```sh
cargo check --tests
cargo test
cargo +nightly fmt
```

### Extended tests

Some long running tests are marked with `#[ignore]`, to execute them:

```sh
cargo test --release -- --ignored
```

To run all tests:

```sh
cargo test --release -- --include-ignored
```

### Run

To run in analyze-mode for the starting position:

```sh
cargo run --release
```

To run in analyze-mode for a position given with fen:

```sh
cargo run --release -- 'rnbqkbnr/1ppppppp/pB6/8/8/2P2P2/PP1PP1PP/RNB1K1NR b KQkq - 0 1'
```

## ideas

- mcts
- from fen
- lots of tests for
  - evalutation
  - search: in particular mates

## todos

- x,y vs rank,file
- replace starting position fen with Board::new()
- in tests: replace assert_eq on move lists with assert on .contains
- bitboards
- zobrist hash
- evaluation: kkr is draw
  - draw
    - kkb
    - kkn
    - kkr
    - kknn
  - not draw
    - kkrn
    - kkrb
    - kkbb
    - kknb
- pawn structure is easy: there are never double pawns

## MCTS extension

### Links

- https://sci-hub.et-fine.com/10.3233/icg-180065

### Ideas

MCTS is bad at detecting forced wins/losses inside the explored tree. This can
be helped with the following ideas.

#### Backtracking

```rs
if Some(win_color) = node.evaluation.get_win_color() {
  if win_color != board.active_color
    || parent.children.iter().all(|c| c.evaluation == node.evaluation)
  {
    parent.evaluation = node.evaluation;
  }
}
```

#### Selection

```rs
if node.evaluation != Evaluation::Inconclusive {
  continue;
}
```

## Profiling and Benchmarking

```sh
./flamegraph.sh
```

https://github.com/sharkdp/hyperfine
