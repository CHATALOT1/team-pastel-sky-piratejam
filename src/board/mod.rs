//! Contains all backend logic/functionality for the game 'board'.
//! The 'board' encompasses the map that the player sees and the characters on it.
//!
//! Other logic relating exclusively to the *mechanics* of the board should also be here
//! (e.g. Illumination system).
//!
//! Code pertaining to the *graphics* of the board (e.g. sprite loading, etc.) should be in
//! [crate::graphics::board]. Code pertaining to sound emitted by entites on the board/other board-
//! based SFX should be in [crate::sound].
//!
//! See
//! [this filter](https://github.com/CHATALOT1/team-pastel-sky-piratejam/labels/module%3A%20board)
//! for relevant Issues and PRs.
use super::*;

/// Board Plugin. See [module-level documentation](self).
pub fn plugin(app: &mut App) {}
