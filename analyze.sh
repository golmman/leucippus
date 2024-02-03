#!/bin/bash

#
# BLACK TO MOVE
#

# forced mate in 3 for black
#cargo run --release -- 'rnbqkb1r/pppppppp/5n2/8/P7/8/1PPPPPPP/RNBQKBNR b KQkq - 0 2'

# forced mate in 3 for white
#cargo run --release -- '3R1r1r/p1k2P2/7p/1pp1B1p1/4P3/3p2P1/P2P3P/R5K1 b - - 1 25'

# forced mate in 2 for white
#cargo run --release -- '5r1r/p2R1P2/1k5p/1pp1B1p1/4P3/3p2P1/P2P3P/R5K1 b - - 3 26'

#
# WHITE TO MOVE
#

# forced mate in 1 for white
#cargo run --release -- 'rnbqkbnr/1pppppp1/p6p/4N3/8/8/PPPPPPPP/RNBQKB1R w KQkq - 0 3'

# forced mate in 3 for white
#cargo run --release -- 'rnbqkbnr/1ppppppp/p7/8/8/5N2/PPPPPPPP/RNBQKB1R w KQkq - 0 2'

# 3 best moves: Qh4, Nb5, Qg5
#cargo run --release -- 'rnbqkbnr/pppp3p/4p1p1/5p1Q/3N4/4P3/PPPP1PPP/RNB1KB1R w KQkq - 0 5'

# 1 clear winning move (c7,b6), also instant losing moves (c7,c5), (c7,c6)
cargo run --release -- 'rnbqkbnr/1ppppppp/pB6/8/8/2P2P2/PP1PP1PP/RNB1K1NR b KQkq - 0 1'
