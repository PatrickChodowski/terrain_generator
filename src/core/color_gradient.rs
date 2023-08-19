
use bevy_egui::egui::{DragValue, Color32, Ui};
use bevy::prelude::ResMut;
use crate::editor::mtb_ui::ModResources;


#[derive(Debug, Clone, Copy)]
pub struct ColorGradient {
    pub min_height: f32,
    pub max_height: f32,
    pub min_color: [f32; 4],
    pub _min_color: Color32,
    pub max_color: [f32; 4],
    pub _max_color: Color32
}


impl ColorGradient {
    pub fn apply(&self, height: f32) -> [f32; 4] {
        let scale = (height - self.min_height)/(self.max_height - self.min_height);
        return [self.min_color[0] + scale*(self.max_color[0] - self.min_color[0]), 
                self.min_color[1] + scale*(self.max_color[1] - self.min_color[1]),
                self.min_color[2] + scale*(self.max_color[2] - self.min_color[2]),
                self.min_color[3] + scale*(self.max_color[3] - self.min_color[3])];

    }
    pub fn new() -> Self {
        let mut clr = ColorGradient{_min_color: Color32::WHITE.linear_multiply(0.5), 
                                    min_color: [1.0, 1.0, 1.0, 1.0],
                                    _max_color: Color32::BLACK.linear_multiply(0.5), 
                                    max_color: [1.0, 1.0, 1.0, 1.0],
                                    min_height: 0.0,
                                    max_height: 10.0};
        clr.set();
        return clr;
    }
    pub fn set(&mut self){
        self.min_color = [(self._min_color.r() as f32)/255.0, 
                          (self._min_color.g() as f32)/255.0, 
                          (self._min_color.b() as f32)/255.0, 
                          (self._min_color.a() as f32)/255.0];

        self.max_color = [(self._max_color.r() as f32)/255.0, 
                          (self._max_color.g() as f32)/255.0, 
                          (self._max_color.b() as f32)/255.0, 
                          (self._max_color.a() as f32)/255.0];
    }

    pub fn ui(ui: &mut Ui, mod_res: &mut ResMut<ModResources>) {
        ui.vertical(|ui| {
            ui.label("Color Gradient");
            ui.label("Min Height:");
            ui.add(DragValue::new(&mut mod_res.color_gradient.min_height).speed(1.0));
            ui.label("Max Height:");
            ui.add(DragValue::new(&mut mod_res.color_gradient.max_height).speed(1.0));
            ui.label("Min Color:");
            ui.color_edit_button_srgba(&mut mod_res.color_gradient._min_color);
            ui.label("Max Color:");
            ui.color_edit_button_srgba(&mut mod_res.color_gradient._max_color);
          });
    }
}
