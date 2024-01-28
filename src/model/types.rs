use self::square_names::*;
// TODO: move to common
pub type SquareIndex = u8;
pub type TreeNodeIndex = usize;

pub const TREE_NODE_ROOT_INDEX: usize = 0;

#[rustfmt::skip]
pub const EN_PASSANT_CANDIDATES: [(Option<SquareIndex>, Option<SquareIndex>); 64] = [
    (None, None), (None, None), (None, None), (None, None), (None, None), (None, None), (None, None), (None, None),
    (None, None), (None, None), (None, None), (None, None), (None, None), (None, None), (None, None), (None, None),
    (None, B4OP), (A4OP, C4OP), (B4OP, D4OP), (C4OP, E4OP), (D4OP, F4OP), (E4OP, G4OP), (F4OP, H4OP), (G4OP, None),
    (None, None), (None, None), (None, None), (None, None), (None, None), (None, None), (None, None), (None, None),
    (None, None), (None, None), (None, None), (None, None), (None, None), (None, None), (None, None), (None, None),
    (None, B5OP), (A5OP, C5OP), (B5OP, D5OP), (C5OP, E5OP), (D5OP, F5OP), (E5OP, G5OP), (F5OP, H5OP), (G5OP, None),
    (None, None), (None, None), (None, None), (None, None), (None, None), (None, None), (None, None), (None, None),
    (None, None), (None, None), (None, None), (None, None), (None, None), (None, None), (None, None), (None, None),
];

#[rustfmt::skip]
pub const SQUARE_NEIGHBORHOODS: [[Option<u8>; 8]; 64] = [
    [Some(1),Some(8),Some(9),None,None,None,None,None,],
    [Some(0),Some(2),Some(8),Some(9),Some(10),None,None,None,],
    [Some(1),Some(3),Some(9),Some(10),Some(11),None,None,None,],
    [Some(2),Some(4),Some(10),Some(11),Some(12),None,None,None,],
    [Some(3),Some(5),Some(11),Some(12),Some(13),None,None,None,],
    [Some(4),Some(6),Some(12),Some(13),Some(14),None,None,None,],
    [Some(5),Some(7),Some(13),Some(14),Some(15),None,None,None,],
    [Some(6),Some(14),Some(15),None,None,None,None,None,],
    [Some(0),Some(1),Some(9),Some(16),Some(17),None,None,None,],
    [Some(0),Some(1),Some(2),Some(8),Some(10),Some(16),Some(17),Some(18),],
    [Some(1),Some(2),Some(3),Some(9),Some(11),Some(17),Some(18),Some(19),],
    [Some(2),Some(3),Some(4),Some(10),Some(12),Some(18),Some(19),Some(20),],
    [Some(3),Some(4),Some(5),Some(11),Some(13),Some(19),Some(20),Some(21),],
    [Some(4),Some(5),Some(6),Some(12),Some(14),Some(20),Some(21),Some(22),],
    [Some(5),Some(6),Some(7),Some(13),Some(15),Some(21),Some(22),Some(23),],
    [Some(6),Some(7),Some(14),Some(22),Some(23),None,None,None,],
    [Some(8),Some(9),Some(17),Some(24),Some(25),None,None,None,],
    [Some(8),Some(9),Some(10),Some(16),Some(18),Some(24),Some(25),Some(26),],
    [Some(9),Some(10),Some(11),Some(17),Some(19),Some(25),Some(26),Some(27),],
    [Some(10),Some(11),Some(12),Some(18),Some(20),Some(26),Some(27),Some(28),],
    [Some(11),Some(12),Some(13),Some(19),Some(21),Some(27),Some(28),Some(29),],
    [Some(12),Some(13),Some(14),Some(20),Some(22),Some(28),Some(29),Some(30),],
    [Some(13),Some(14),Some(15),Some(21),Some(23),Some(29),Some(30),Some(31),],
    [Some(14),Some(15),Some(22),Some(30),Some(31),None,None,None,],
    [Some(16),Some(17),Some(25),Some(32),Some(33),None,None,None,],
    [Some(16),Some(17),Some(18),Some(24),Some(26),Some(32),Some(33),Some(34),],
    [Some(17),Some(18),Some(19),Some(25),Some(27),Some(33),Some(34),Some(35),],
    [Some(18),Some(19),Some(20),Some(26),Some(28),Some(34),Some(35),Some(36),],
    [Some(19),Some(20),Some(21),Some(27),Some(29),Some(35),Some(36),Some(37),],
    [Some(20),Some(21),Some(22),Some(28),Some(30),Some(36),Some(37),Some(38),],
    [Some(21),Some(22),Some(23),Some(29),Some(31),Some(37),Some(38),Some(39),],
    [Some(22),Some(23),Some(30),Some(38),Some(39),None,None,None,],
    [Some(24),Some(25),Some(33),Some(40),Some(41),None,None,None,],
    [Some(24),Some(25),Some(26),Some(32),Some(34),Some(40),Some(41),Some(42),],
    [Some(25),Some(26),Some(27),Some(33),Some(35),Some(41),Some(42),Some(43),],
    [Some(26),Some(27),Some(28),Some(34),Some(36),Some(42),Some(43),Some(44),],
    [Some(27),Some(28),Some(29),Some(35),Some(37),Some(43),Some(44),Some(45),],
    [Some(28),Some(29),Some(30),Some(36),Some(38),Some(44),Some(45),Some(46),],
    [Some(29),Some(30),Some(31),Some(37),Some(39),Some(45),Some(46),Some(47),],
    [Some(30),Some(31),Some(38),Some(46),Some(47),None,None,None,],
    [Some(32),Some(33),Some(41),Some(48),Some(49),None,None,None,],
    [Some(32),Some(33),Some(34),Some(40),Some(42),Some(48),Some(49),Some(50),],
    [Some(33),Some(34),Some(35),Some(41),Some(43),Some(49),Some(50),Some(51),],
    [Some(34),Some(35),Some(36),Some(42),Some(44),Some(50),Some(51),Some(52),],
    [Some(35),Some(36),Some(37),Some(43),Some(45),Some(51),Some(52),Some(53),],
    [Some(36),Some(37),Some(38),Some(44),Some(46),Some(52),Some(53),Some(54),],
    [Some(37),Some(38),Some(39),Some(45),Some(47),Some(53),Some(54),Some(55),],
    [Some(38),Some(39),Some(46),Some(54),Some(55),None,None,None,],
    [Some(40),Some(41),Some(49),Some(56),Some(57),None,None,None,],
    [Some(40),Some(41),Some(42),Some(48),Some(50),Some(56),Some(57),Some(58),],
    [Some(41),Some(42),Some(43),Some(49),Some(51),Some(57),Some(58),Some(59),],
    [Some(42),Some(43),Some(44),Some(50),Some(52),Some(58),Some(59),Some(60),],
    [Some(43),Some(44),Some(45),Some(51),Some(53),Some(59),Some(60),Some(61),],
    [Some(44),Some(45),Some(46),Some(52),Some(54),Some(60),Some(61),Some(62),],
    [Some(45),Some(46),Some(47),Some(53),Some(55),Some(61),Some(62),Some(63),],
    [Some(46),Some(47),Some(54),Some(62),Some(63),None,None,None,],
    [Some(48),Some(49),Some(57),None,None,None,None,None,],
    [Some(48),Some(49),Some(50),Some(56),Some(58),None,None,None,],
    [Some(49),Some(50),Some(51),Some(57),Some(59),None,None,None,],
    [Some(50),Some(51),Some(52),Some(58),Some(60),None,None,None,],
    [Some(51),Some(52),Some(53),Some(59),Some(61),None,None,None,],
    [Some(52),Some(53),Some(54),Some(60),Some(62),None,None,None,],
    [Some(53),Some(54),Some(55),Some(61),Some(63),None,None,None,],
    [Some(54),Some(55),Some(62),None,None,None,None,None,],
];

#[rustfmt::skip]
pub const SQUARES_TOUCH: [[u8; 64]; 64] = [
    [0,1,0,0,0,0,0,0,1,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,],
    [1,0,1,0,0,0,0,0,1,1,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,],
    [0,1,0,1,0,0,0,0,0,1,1,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,],
    [0,0,1,0,1,0,0,0,0,0,1,1,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,],
    [0,0,0,1,0,1,0,0,0,0,0,1,1,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,],
    [0,0,0,0,1,0,1,0,0,0,0,0,1,1,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,],
    [0,0,0,0,0,1,0,1,0,0,0,0,0,1,1,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,],
    [0,0,0,0,0,0,1,0,0,0,0,0,0,0,1,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,],
    [1,1,0,0,0,0,0,0,0,1,0,0,0,0,0,0,1,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,],
    [1,1,1,0,0,0,0,0,1,0,1,0,0,0,0,0,1,1,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,],
    [0,1,1,1,0,0,0,0,0,1,0,1,0,0,0,0,0,1,1,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,],
    [0,0,1,1,1,0,0,0,0,0,1,0,1,0,0,0,0,0,1,1,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,],
    [0,0,0,1,1,1,0,0,0,0,0,1,0,1,0,0,0,0,0,1,1,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,],
    [0,0,0,0,1,1,1,0,0,0,0,0,1,0,1,0,0,0,0,0,1,1,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,],
    [0,0,0,0,0,1,1,1,0,0,0,0,0,1,0,1,0,0,0,0,0,1,1,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,],
    [0,0,0,0,0,0,1,1,0,0,0,0,0,0,1,0,0,0,0,0,0,0,1,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,],
    [0,0,0,0,0,0,0,0,1,1,0,0,0,0,0,0,0,1,0,0,0,0,0,0,1,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,],
    [0,0,0,0,0,0,0,0,1,1,1,0,0,0,0,0,1,0,1,0,0,0,0,0,1,1,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,],
    [0,0,0,0,0,0,0,0,0,1,1,1,0,0,0,0,0,1,0,1,0,0,0,0,0,1,1,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,],
    [0,0,0,0,0,0,0,0,0,0,1,1,1,0,0,0,0,0,1,0,1,0,0,0,0,0,1,1,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,],
    [0,0,0,0,0,0,0,0,0,0,0,1,1,1,0,0,0,0,0,1,0,1,0,0,0,0,0,1,1,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,],
    [0,0,0,0,0,0,0,0,0,0,0,0,1,1,1,0,0,0,0,0,1,0,1,0,0,0,0,0,1,1,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,],
    [0,0,0,0,0,0,0,0,0,0,0,0,0,1,1,1,0,0,0,0,0,1,0,1,0,0,0,0,0,1,1,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,],
    [0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,1,0,0,0,0,0,0,1,0,0,0,0,0,0,0,1,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,],
    [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,1,0,0,0,0,0,0,0,1,0,0,0,0,0,0,1,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,],
    [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,1,1,0,0,0,0,0,1,0,1,0,0,0,0,0,1,1,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,],
    [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,1,1,0,0,0,0,0,1,0,1,0,0,0,0,0,1,1,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,],
    [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,1,1,0,0,0,0,0,1,0,1,0,0,0,0,0,1,1,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,],
    [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,1,1,0,0,0,0,0,1,0,1,0,0,0,0,0,1,1,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,],
    [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,1,1,0,0,0,0,0,1,0,1,0,0,0,0,0,1,1,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,],
    [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,1,1,0,0,0,0,0,1,0,1,0,0,0,0,0,1,1,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,],
    [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,1,0,0,0,0,0,0,1,0,0,0,0,0,0,0,1,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,],
    [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,1,0,0,0,0,0,0,0,1,0,0,0,0,0,0,1,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,],
    [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,1,1,0,0,0,0,0,1,0,1,0,0,0,0,0,1,1,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,],
    [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,1,1,0,0,0,0,0,1,0,1,0,0,0,0,0,1,1,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,],
    [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,1,1,0,0,0,0,0,1,0,1,0,0,0,0,0,1,1,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,],
    [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,1,1,0,0,0,0,0,1,0,1,0,0,0,0,0,1,1,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,],
    [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,1,1,0,0,0,0,0,1,0,1,0,0,0,0,0,1,1,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,],
    [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,1,1,0,0,0,0,0,1,0,1,0,0,0,0,0,1,1,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,],
    [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,1,0,0,0,0,0,0,1,0,0,0,0,0,0,0,1,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,],
    [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,1,0,0,0,0,0,0,0,1,0,0,0,0,0,0,1,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,],
    [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,1,1,0,0,0,0,0,1,0,1,0,0,0,0,0,1,1,1,0,0,0,0,0,0,0,0,0,0,0,0,0,],
    [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,1,1,0,0,0,0,0,1,0,1,0,0,0,0,0,1,1,1,0,0,0,0,0,0,0,0,0,0,0,0,],
    [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,1,1,0,0,0,0,0,1,0,1,0,0,0,0,0,1,1,1,0,0,0,0,0,0,0,0,0,0,0,],
    [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,1,1,0,0,0,0,0,1,0,1,0,0,0,0,0,1,1,1,0,0,0,0,0,0,0,0,0,0,],
    [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,1,1,0,0,0,0,0,1,0,1,0,0,0,0,0,1,1,1,0,0,0,0,0,0,0,0,0,],
    [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,1,1,0,0,0,0,0,1,0,1,0,0,0,0,0,1,1,1,0,0,0,0,0,0,0,0,],
    [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,1,0,0,0,0,0,0,1,0,0,0,0,0,0,0,1,1,0,0,0,0,0,0,0,0,],
    [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,1,0,0,0,0,0,0,0,1,0,0,0,0,0,0,1,1,0,0,0,0,0,0,],
    [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,1,1,0,0,0,0,0,1,0,1,0,0,0,0,0,1,1,1,0,0,0,0,0,],
    [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,1,1,0,0,0,0,0,1,0,1,0,0,0,0,0,1,1,1,0,0,0,0,],
    [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,1,1,0,0,0,0,0,1,0,1,0,0,0,0,0,1,1,1,0,0,0,],
    [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,1,1,0,0,0,0,0,1,0,1,0,0,0,0,0,1,1,1,0,0,],
    [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,1,1,0,0,0,0,0,1,0,1,0,0,0,0,0,1,1,1,0,],
    [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,1,1,0,0,0,0,0,1,0,1,0,0,0,0,0,1,1,1,],
    [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,1,0,0,0,0,0,0,1,0,0,0,0,0,0,0,1,1,],
    [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,1,0,0,0,0,0,0,0,1,0,0,0,0,0,0,],
    [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,1,1,0,0,0,0,0,1,0,1,0,0,0,0,0,],
    [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,1,1,0,0,0,0,0,1,0,1,0,0,0,0,],
    [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,1,1,0,0,0,0,0,1,0,1,0,0,0,],
    [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,1,1,0,0,0,0,0,1,0,1,0,0,],
    [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,1,1,0,0,0,0,0,1,0,1,0,],
    [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,1,1,0,0,0,0,0,1,0,1,],
    [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,1,0,0,0,0,0,0,1,0,],
];

pub mod square_names {
    use super::SquareIndex;

    pub const A1: SquareIndex = 0;
    pub const B1: SquareIndex = 1;
    pub const C1: SquareIndex = 2;
    pub const D1: SquareIndex = 3;
    pub const E1: SquareIndex = 4;
    pub const F1: SquareIndex = 5;
    pub const G1: SquareIndex = 6;
    pub const H1: SquareIndex = 7;
    pub const A2: SquareIndex = 8;
    pub const B2: SquareIndex = 9;
    pub const C2: SquareIndex = 10;
    pub const D2: SquareIndex = 11;
    pub const E2: SquareIndex = 12;
    pub const F2: SquareIndex = 13;
    pub const G2: SquareIndex = 14;
    pub const H2: SquareIndex = 15;
    pub const A3: SquareIndex = 16;
    pub const B3: SquareIndex = 17;
    pub const C3: SquareIndex = 18;
    pub const D3: SquareIndex = 19;
    pub const E3: SquareIndex = 20;
    pub const F3: SquareIndex = 21;
    pub const G3: SquareIndex = 22;
    pub const H3: SquareIndex = 23;
    pub const A4: SquareIndex = 24;
    pub const B4: SquareIndex = 25;
    pub const C4: SquareIndex = 26;
    pub const D4: SquareIndex = 27;
    pub const E4: SquareIndex = 28;
    pub const F4: SquareIndex = 29;
    pub const G4: SquareIndex = 30;
    pub const H4: SquareIndex = 31;
    pub const A5: SquareIndex = 32;
    pub const B5: SquareIndex = 33;
    pub const C5: SquareIndex = 34;
    pub const D5: SquareIndex = 35;
    pub const E5: SquareIndex = 36;
    pub const F5: SquareIndex = 37;
    pub const G5: SquareIndex = 38;
    pub const H5: SquareIndex = 39;
    pub const A6: SquareIndex = 40;
    pub const B6: SquareIndex = 41;
    pub const C6: SquareIndex = 42;
    pub const D6: SquareIndex = 43;
    pub const E6: SquareIndex = 44;
    pub const F6: SquareIndex = 45;
    pub const G6: SquareIndex = 46;
    pub const H6: SquareIndex = 47;
    pub const A7: SquareIndex = 48;
    pub const B7: SquareIndex = 49;
    pub const C7: SquareIndex = 50;
    pub const D7: SquareIndex = 51;
    pub const E7: SquareIndex = 52;
    pub const F7: SquareIndex = 53;
    pub const G7: SquareIndex = 54;
    pub const H7: SquareIndex = 55;
    pub const A8: SquareIndex = 56;
    pub const B8: SquareIndex = 57;
    pub const C8: SquareIndex = 58;
    pub const D8: SquareIndex = 59;
    pub const E8: SquareIndex = 60;
    pub const F8: SquareIndex = 61;
    pub const G8: SquareIndex = 62;
    pub const H8: SquareIndex = 63;

    pub const A1OP: Option<SquareIndex> = Some(A1);
    pub const B1OP: Option<SquareIndex> = Some(B1);
    pub const C1OP: Option<SquareIndex> = Some(C1);
    pub const D1OP: Option<SquareIndex> = Some(D1);
    pub const E1OP: Option<SquareIndex> = Some(E1);
    pub const F1OP: Option<SquareIndex> = Some(F1);
    pub const G1OP: Option<SquareIndex> = Some(G1);
    pub const H1OP: Option<SquareIndex> = Some(H1);
    pub const A2OP: Option<SquareIndex> = Some(A2);
    pub const B2OP: Option<SquareIndex> = Some(B2);
    pub const C2OP: Option<SquareIndex> = Some(C2);
    pub const D2OP: Option<SquareIndex> = Some(D2);
    pub const E2OP: Option<SquareIndex> = Some(E2);
    pub const F2OP: Option<SquareIndex> = Some(F2);
    pub const G2OP: Option<SquareIndex> = Some(G2);
    pub const H2OP: Option<SquareIndex> = Some(H2);
    pub const A3OP: Option<SquareIndex> = Some(A3);
    pub const B3OP: Option<SquareIndex> = Some(B3);
    pub const C3OP: Option<SquareIndex> = Some(C3);
    pub const D3OP: Option<SquareIndex> = Some(D3);
    pub const E3OP: Option<SquareIndex> = Some(E3);
    pub const F3OP: Option<SquareIndex> = Some(F3);
    pub const G3OP: Option<SquareIndex> = Some(G3);
    pub const H3OP: Option<SquareIndex> = Some(H3);
    pub const A4OP: Option<SquareIndex> = Some(A4);
    pub const B4OP: Option<SquareIndex> = Some(B4);
    pub const C4OP: Option<SquareIndex> = Some(C4);
    pub const D4OP: Option<SquareIndex> = Some(D4);
    pub const E4OP: Option<SquareIndex> = Some(E4);
    pub const F4OP: Option<SquareIndex> = Some(F4);
    pub const G4OP: Option<SquareIndex> = Some(G4);
    pub const H4OP: Option<SquareIndex> = Some(H4);
    pub const A5OP: Option<SquareIndex> = Some(A5);
    pub const B5OP: Option<SquareIndex> = Some(B5);
    pub const C5OP: Option<SquareIndex> = Some(C5);
    pub const D5OP: Option<SquareIndex> = Some(D5);
    pub const E5OP: Option<SquareIndex> = Some(E5);
    pub const F5OP: Option<SquareIndex> = Some(F5);
    pub const G5OP: Option<SquareIndex> = Some(G5);
    pub const H5OP: Option<SquareIndex> = Some(H5);
    pub const A6OP: Option<SquareIndex> = Some(A6);
    pub const B6OP: Option<SquareIndex> = Some(B6);
    pub const C6OP: Option<SquareIndex> = Some(C6);
    pub const D6OP: Option<SquareIndex> = Some(D6);
    pub const E6OP: Option<SquareIndex> = Some(E6);
    pub const F6OP: Option<SquareIndex> = Some(F6);
    pub const G6OP: Option<SquareIndex> = Some(G6);
    pub const H6OP: Option<SquareIndex> = Some(H6);
    pub const A7OP: Option<SquareIndex> = Some(A7);
    pub const B7OP: Option<SquareIndex> = Some(B7);
    pub const C7OP: Option<SquareIndex> = Some(C7);
    pub const D7OP: Option<SquareIndex> = Some(D7);
    pub const E7OP: Option<SquareIndex> = Some(E7);
    pub const F7OP: Option<SquareIndex> = Some(F7);
    pub const G7OP: Option<SquareIndex> = Some(G7);
    pub const H7OP: Option<SquareIndex> = Some(H7);
    pub const A8OP: Option<SquareIndex> = Some(A8);
    pub const B8OP: Option<SquareIndex> = Some(B8);
    pub const C8OP: Option<SquareIndex> = Some(C8);
    pub const D8OP: Option<SquareIndex> = Some(D8);
    pub const E8OP: Option<SquareIndex> = Some(E8);
    pub const F8OP: Option<SquareIndex> = Some(F8);
    pub const G8OP: Option<SquareIndex> = Some(G8);
    pub const H8OP: Option<SquareIndex> = Some(H8);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn print_squares_touch() {
        // run with `cargo test print_squares_touch -- --nocapture`
        println!("pub const SQUARES_TOUCH: [[u8; 64]; 64] = [");

        for j in 0..64 {
            print!("    [");
            for i in 0..64 {
                if SQUARE_NEIGHBORHOODS[i].contains(&Some(j)) {
                    print!("1,");
                } else {
                    print!("0,");
                }
            }
            println!("],");
        }

        println!("];");
    }

    #[test]
    fn print_square_neighborhood_array() {
        // run with `cargo test print_square_neighborhood_array -- --nocapture`
        let dirs = vec![
            (-1, -1),
            (0, -1),
            (1, -1),
            (-1, 0),
            (1, 0),
            (-1, 1),
            (0, 1),
            (1, 1),
        ];

        println!("pub const SQUARE_NEIGHBORHOODS: [[Option<u8>; 8]; 64] = [");

        for i in 0..64 {
            let x = (i % 8) as i8;
            let y = (i / 8) as i8;
            let mut count = 0;

            print!("    [");
            for dir in &dirs {
                let neigh_x = x + dir.0;
                let neigh_y = y + dir.1;
                if neigh_x >= 0 && neigh_x < 8 && neigh_y >= 0 && neigh_y < 8 {
                    count += 1;
                    print!("Some({}),", 8 * neigh_y + neigh_x);
                }
            }
            for _j in 0..8 - count {
                print!("None,");
            }
            println!("],");
        }

        println!("];");
    }
}
