
use serde::{Serialize, Deserialize};
use super::modifiers::{ModifierTrait, Edge};
use super::utils::{AABBs, PlaneData};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct WanderNoise {
    pub wander:               Wanders,
    pub width:                f32,
    pub height:               f32,
    pub source_edge:          Option<Edge>,
    pub target_edge:          Option<Edge>,
    pub source_point:         Option<(f32, f32)>,
    pub target_point:         Option<(f32, f32)>
}

impl WanderNoise {
    pub fn aabbs(&self, pd: &PlaneData) -> AABBs {
        let aabbs = AABBs::new();
        return aabbs;
    }
}

impl ModifierTrait for WanderNoise {
    fn apply(&self, pos: &[f32; 3], aabbs: &AABBs) -> f32 {
        if aabbs.has_point(pos) {
            return self.height;
        }
        return pos[1];
    }
}





#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum Wanders {
    DrunkRobot,     // Random Direction n steps, choose random direction n steps
    DrunkZax,       // goes nsew or e, or e or e (in general follows in direction)
    DrunkEmu,       // doesnt allow backwards
    DrunkEmu90,     // not, grid, steers randomly between 0-90 angle
    DrunkEmu60,     // 0-60
    DrunkEmu30,     // 0-30
    DrunkEmuPerlin, // next angle is generated by perlin
    DrunkWanderer   // nswe, goal goal goal, when you have a start and end point
}
