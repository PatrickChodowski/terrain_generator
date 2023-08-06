
use bevy::prelude::*;

pub mod edit_planes;
use edit_planes::PlaneEditorPlugin;

pub mod mtb_camera;
pub mod mtb_grid;
pub mod mtb_ui;

use mtb_core::planes::PlanesPlugin;
use mtb_camera::MTBCameraPlugin;
use mtb_grid::MTBGridPlugin;
use mtb_ui::MTBUIPlugin;


pub struct MTBEditorPlugin;

impl Plugin for MTBEditorPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_plugin(PlaneEditorPlugin)
        .add_plugin(MTBCameraPlugin)
        .add_plugin(MTBGridPlugin)
        .add_plugin(MTBUIPlugin)
        .add_plugin(PlanesPlugin)
        ;
    }
  }

