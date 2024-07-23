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

/// This [`Schedule`] contains systems that update the game's graphics based on changes made. it is
/// run after [`Update`] and before [`PostUpdate`].
#[derive(bevy::ecs::schedule::ScheduleLabel, Debug, Clone, PartialEq, Eq, Hash)]
struct UpdateGraphics;

/// Plugins that add graphical functionality to the game. See [module-level documentation](self).
pub struct GraphicsPlugins;
impl PluginGroup for GraphicsPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>().add(base_plugin)
    }
}

/// A plugin containing base functionality useful to all other plugins in [`GraphicsPlugins`]
fn base_plugin(app: &mut App) {
    // Initialize the `UpdateGraphics` schedule and add it to the `MainScheduleOrder`, to ensure
    // that it is run.
    app.init_schedule(UpdateGraphics);
    app.world_mut()
        .resource_mut::<bevy::app::MainScheduleOrder>()
        .insert_after(Update, UpdateGraphics);
}
