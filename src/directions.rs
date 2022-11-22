use crate::point::GridPoint;
use glam::{IVec2, IVec3};
use std::ops::Add;

// X X ASC DESC  UP RIGHT DOWN LEFT
//0b0000 0000

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Direction(u8);

impl Direction {
    /// Array of four orthogonal grid directions.
    pub const DIR_4: &[Direction] = &[Self::UP, Self::DOWN, Self::LEFT, Self::RIGHT];

    /// Array of eight adjacent grid directions.
    pub const DIR_8: &[Direction] = &[
        Self::UP,
        Self::DOWN,
        Self::LEFT,
        Self::RIGHT,
        Self::UP_LEFT,
        Self::UP_RIGHT,
        Self::DOWN_LEFT,
        Self::DOWN_RIGHT,
    ];

    // Flat traversal
    pub const NONE: Self = Self(0b0);
    pub const UP: Self = Self(0b1000);
    pub const UP_RIGHT: Self = Self(0b1100);
    pub const RIGHT: Self = Self(0b0100);
    pub const DOWN_RIGHT: Self = Self(0b0110);
    pub const DOWN: Self = Self(0b0010);
    pub const DOWN_LEFT: Self = Self(0b0011);
    pub const LEFT: Self = Self(0b0001);
    pub const UP_LEFT: Self = Self(0b1001);

    // Descending Traversal
    pub const DESCEND: Self = Self(0b10000);
    pub const DESC_UP: Self = Self(0b11000);
    pub const DESC_UP_RIGHT: Self = Self(0b11100);
    pub const DESC_RIGHT: Self = Self(0b10100);
    pub const DESC_DOWN_RIGHT: Self = Self(0b10110);
    pub const DESC_DOWN: Self = Self(0b10010);
    pub const DESC_DOWN_LEFT: Self = Self(0b10011);
    pub const DESC_LEFT: Self = Self(0b10001);
    pub const DESC_UP_LEFT: Self = Self(0b11001);

    // Ascending Traversal
    pub const ASCEND: Self = Self(0b100000);
    pub const ASC_UP: Self = Self(0b101000);
    pub const ASC_UP_RIGHT: Self = Self(0b101100);
    pub const ASC_RIGHT: Self = Self(0b100100);
    pub const ASC_DOWN_RIGHT: Self = Self(0b100110);
    pub const ASC_DOWN: Self = Self(0b100010);
    pub const ASC_DOWN_LEFT: Self = Self(0b100011);
    pub const ASC_LEFT: Self = Self(0b100001);
    pub const ASC_UP_LEFT: Self = Self(0b101001);
}

impl From<[i32; 2]> for Direction {
    fn from(a: [i32; 2]) -> Self {
        match a {
            [0, 0] => Self::NONE,
            [0, 1] => Self::UP,
            [1, 1] => Self::UP_RIGHT,
            [1, 0] => Self::RIGHT,
            [1, -1] => Self::DOWN_RIGHT,
            [0, -1] => Self::DOWN,
            [-1, -1] => Self::DOWN_LEFT,
            [-1, 0] => Self::LEFT,
            [-1, 1] => Self::UP_LEFT,
            _ => unreachable!(),
        }
    }
}

impl From<Direction> for [i32; 2] {
    fn from(d: Direction) -> Self {
        match d {
            Direction::NONE => [0, 0],
            Direction::UP => [0, 1],
            Direction::UP_RIGHT => [1, 1],
            Direction::RIGHT => [1, 0],
            Direction::DOWN_RIGHT => [1, -1],
            Direction::DOWN => [0, -1],
            Direction::DOWN_LEFT => [-1, -1],
            Direction::LEFT => [-1, 0],
            Direction::UP_LEFT => [-1, 1],
            _ => unreachable!(),
        }
    }
}

impl From<Direction> for IVec2 {
    fn from(d: Direction) -> Self {
        let xy: [i32; 2] = d.into();
        Self { x: xy[0], y: xy[1] }
    }
}

impl From<IVec2> for Direction {
    fn from(v: IVec2) -> Self {
        Self::from(v.as_array())
    }
}

impl Add<Direction> for IVec2 {
    type Output = IVec2;

    fn add(self, addend: Direction) -> Self::Output {
        let xy: [i32; 2] = addend.into();
        Self::Output {
            x: xy[0] + self.x,
            y: xy[1] + self.y,
        }
    }
}

impl Add<IVec2> for Direction {
    type Output = IVec2;

    fn add(self, addend: Self::Output) -> Self::Output {
        let xy: [i32; 2] = self.into();
        Self::Output {
            x: xy[0] + addend.x,
            y: xy[1] + addend.y,
        }
    }
}

impl From<IVec3> for Direction {
    fn from(v: IVec3) -> Self {
        match v.z {
            0 => Self::from([v.x, v.y]),
            1 => Self(Self::from([v.x, v.y]).0 | Self::ASCEND.0),
            -1 => Self(Self::from([v.x, v.y]).0 | Self::DESCEND.0),
            _ => unreachable!(),
        }
    }
}
