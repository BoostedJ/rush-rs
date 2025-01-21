
#[derive(Hash, PartialEq, Eq, Debug, Clone, Copy)]
pub struct Square(pub usize);

pub const FILE_CHARS: [char; 8] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];
pub const RANK_CHARS: [char; 8] = ['1', '2', '3', '4', '5', '6', '7', '8'];

pub mod masks {

    pub const A_FILE: u64 = 0x0101010101010101;
    pub const B_FILE: u64 = 0x0202020202020202;
    pub const C_FILE: u64 = 0x0404040404040404;
    pub const D_FILE: u64 = 0x0808080808080808;
    pub const H_FILE: u64 = 0x8080808080808080;
    pub const G_FILE: u64 = 0x4040404040404040;
    pub const F_FILE: u64 = 0x2020202020202020;
    pub const E_FILE: u64 = 0x1010101010101010;

    pub const RANK_1: u64 = 0x00000000000000FF;
    pub const RANK_2: u64 = 0x000000000000FF00;
    pub const RANK_3: u64 = 0x0000000000FF0000;
    pub const RANK_4: u64 = 0x00000000FF000000;
    pub const RANK_5: u64 = 0x000000FF00000000;
    pub const RANK_6: u64 = 0x0000FF0000000000;
    pub const RANK_7: u64 = 0x00FF000000000000;
    pub const RANK_8: u64 = 0xFF00000000000000;

    pub const ALL: u64 = 0xFFFFFFFFFFFFFFFF;
    pub const NONE: u64 = 0x0;


    #[inline]
    pub fn file_for_x(x: u8) -> u64 {
        match x {
            0 => A_FILE,
            1 => B_FILE,
            2 => C_FILE,
            3 => D_FILE,
            4 => E_FILE,
            5 => F_FILE,
            6 => G_FILE,
            7 => H_FILE,
            _ => 0,
        }
    }

    pub mod neighbors {
        pub const A_FILE: u64 = super::B_FILE;
        pub const B_FILE: u64 = super::A_FILE | super::C_FILE;
        pub const C_FILE: u64 = super::B_FILE | super::D_FILE;
        pub const D_FILE: u64 = super::C_FILE | super::E_FILE;
        pub const E_FILE: u64 = super::D_FILE | super::F_FILE;
        pub const F_FILE: u64 = super::E_FILE | super::G_FILE;
        pub const G_FILE: u64 = super::F_FILE | super::H_FILE;
        pub const H_FILE: u64 = super::G_FILE;
    }
}

pub mod named {
    use super::Square;

    // Rank 1
    pub const A1: Square = Square(0);
    pub const B1: Square = Square(1);
    pub const C1: Square = Square(2);
    pub const D1: Square = Square(3);
    pub const E1: Square = Square(4);
    pub const F1: Square = Square(5);
    pub const G1: Square = Square(6);
    pub const H1: Square = Square(7);
    // Rank 2
    pub const A2: Square = Square(8);
    pub const B2: Square = Square(9);
    pub const C2: Square = Square(10);
    pub const D2: Square = Square(11);
    pub const E2: Square = Square(12);
    pub const F2: Square = Square(13);
    pub const G2: Square = Square(14);
    pub const H2: Square = Square(15);
    // Rank 3
    pub const A3: Square = Square(16);
    pub const B3: Square = Square(17);
    pub const C3: Square = Square(18);
    pub const D3: Square = Square(19);
    pub const E3: Square = Square(20);
    pub const F3: Square = Square(21);
    pub const G3: Square = Square(22);
    pub const H3: Square = Square(23);
    // Rank 4
    pub const A4: Square = Square(24);
    pub const B4: Square = Square(25);
    pub const C4: Square = Square(26);
    pub const D4: Square = Square(27);
    pub const E4: Square = Square(28);
    pub const F4: Square = Square(29);
    pub const G4: Square = Square(30);
    pub const H4: Square = Square(31);
    // Rank 5
    pub const A5: Square = Square(32);
    pub const B5: Square = Square(33);
    pub const C5: Square = Square(34);
    pub const D5: Square = Square(35);
    pub const E5: Square = Square(36);
    pub const F5: Square = Square(37);
    pub const G5: Square = Square(38);
    pub const H5: Square = Square(39);
    // Rank 6
    pub const A6: Square = Square(40);
    pub const B6: Square = Square(41);
    pub const C6: Square = Square(42);
    pub const D6: Square = Square(43);
    pub const E6: Square = Square(44);
    pub const F6: Square = Square(45);
    pub const G6: Square = Square(46);
    pub const H6: Square = Square(47);
    // Rank 7
    pub const A7: Square = Square(48);
    pub const B7: Square = Square(49);
    pub const C7: Square = Square(50);
    pub const D7: Square = Square(51);
    pub const E7: Square = Square(52);
    pub const F7: Square = Square(53);
    pub const G7: Square = Square(54);
    pub const H7: Square = Square(55);
    // Rank 8
    pub const A8: Square = Square(56);
    pub const B8: Square = Square(57);
    pub const C8: Square = Square(58);
    pub const D8: Square = Square(59);
    pub const E8: Square = Square(60);
    pub const F8: Square = Square(61);
    pub const G8: Square = Square(62);
    pub const H8: Square = Square(63);
}
