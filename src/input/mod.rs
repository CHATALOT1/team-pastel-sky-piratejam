//! Contains all logic/functionality for receiving user input.
//!
//! Largely, this module receives user input and conveys the necessary information to other areas of
//! the codebase using Events defined in *those* modules rather than in this one.
//!
//! See
//! [this filter](https://github.com/CHATALOT1/team-pastel-sky-piratejam/labels/module%3A%20input)
//! for relevant Issues and PRs.
use super::*;

/// This [`Schedule`] contains systems that receive and interpret user input. It is run after
/// [`PreUpdate`] and before [`Update`].
#[derive(bevy::ecs::schedule::ScheduleLabel, Debug, Clone, PartialEq, Eq, Hash)]
struct HandleInput;

/// Input Plugin. See [module-level documentation](self).
pub fn plugin(app: &mut App) {
    // Initialize the `HandleInput` schedule and add it to the `MainScheduleOrder`, to ensure that
    // it is run.
    app.init_schedule(HandleInput);
    app.world_mut()
        .resource_mut::<bevy::app::MainScheduleOrder>()
        .insert_after(PreUpdate, HandleInput);
}
