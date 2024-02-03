# leucippus

## ideas
* mcts
* from fen
* lots of tests for
  * evalutation
  * search: in particular mates

## todos
* x,y vs rank,file
* replace starting position fen with Board::new()
* in tests: replace assert_eq on move lists with assert on .contains
* bitboards
* zobrist hash

## MCTS extension

### Links
* https://sci-hub.et-fine.com/10.3233/icg-180065

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
