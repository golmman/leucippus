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

### Build and Run

To make sure that all of your cpus instruction are used prefix with the following rustflags, e.g.:
```
RUSTFLAGS="-C target-cpu=native" cargo build --release
```

To run in analyze-mode for the starting position:

```sh
cargo run --release
```

To run in analyze-mode for a position given with fen:

```sh
cargo run --release -- 'rnbqkbnr/1ppppppp/pB6/8/8/2P2P2/PP1PP1PP/RNB1K1NR b KQkq - 0 1'
```

#### Magic Bitboard Generation

Generating the magic bitboard tables for bishops and rooks at compile time is
currently very slow because of rusts' relatively slow code interpretation step.

So something like
```rs
const BISHOP_TABLE: BishopTable = {
    ... generate table
};
```
is currently not feasible if you want to prevent compile times of multiple minutes.
Tools like rust-analyzer do also struggle with this.

Instead the raw bitboard data is generated as huge static files and not computed:
```sh
RUSTFLAGS='--cfg bishop_magics' cargo run > src/bitboards/move/bishop_table.rs
RUSTFLAGS='--cfg rook_magics' cargo run > src/bitboards/move/rook_table.rs
```

### Benchmarking

see https://github.com/sharkdp/hyperfine
Requirements:

```sh
cargo install hyperfine
```

Execute the convenience script `./bench.sh` which runs 1 preparation and 10
iterations of 1000 simulations.

Execute the script with the `-r` option to export the result as json to
`bench_results`

### Profiling

see https://github.com/flamegraph-rs/flamegraph
Requirements on Debian:

```sh
cargo install flamegraph
sudo apt install -y linux-perf
```

Execute the convenience script `./flamegraph.sh`
which creates `./flamegraph.svg` with a width of 8000px.

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
- if kings touch certain positions like kkq are also draw
- pawn structure is easy: there are never double pawns
- '1k6/8/pppppppp/8/3b4/2P2BN1/4BRB1/1K2NRN1 w - - 0 1' ???

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
