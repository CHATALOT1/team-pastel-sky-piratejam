//! Contains all logic/functionality for displaying information to the player visually.
//!
//! Some submodules (e.g. [board]) directly correspond to other modules at the crate level
//! (e.g. [crate::board]). These are for *graphical* functionality that is specifically related to
//! the *mechanical* functionality that those higher-level modules provide.
//!
//! See
//! [this filter](https://github.com/CHATALOT1/team-pastel-sky-piratejam/labels/module%3A%20graphics)
//! for relevant Issues and PRs.
use super::*;
use bevy::app::PluginGroupBuilder;

pub mod board;

/// Plugins that add graphical functionality to the game. See [module-level documentation](self).
pub struct GraphicsPlugins;
impl PluginGroup for GraphicsPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
    }
}
