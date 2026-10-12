use bevy::prelude::{Color, Component};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in crate::game) enum BrickKind {
    Normal,
    Tnt,
    ExtraBall,
    ClearRow,
    ClearColumn,
}

#[derive(Component, Clone, Copy)]
pub(in crate::game) struct Brick {
    pub(in crate::game) row: usize,
    pub(in crate::game) column: usize,
    pub(in crate::game) kind: BrickKind,
    pub(in crate::game) color: Color,
}
