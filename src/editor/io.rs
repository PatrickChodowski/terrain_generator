use bevy::prelude::*;
use bevy::render::mesh::VertexAttributeValues;
use bevy_mod_picking::prelude::*;
use serde::{Serialize, Deserialize};
use std::io::{BufWriter, Write};
use std::fs::{self, File};

use crate::core::planes::{PlaneData, TerrainPlane, PickPlane, plane_mesh};
use crate::core::utils::get_mesh_stats;
use crate::core::vertex::Vertex;

pub struct IOPlugin;

impl Plugin for IOPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_event::<WriteData>()
        .add_event::<LoadData>()
        .insert_resource(IOName::new())
        .add_systems(PostUpdate, write_data.run_if(on_event::<WriteData>()))
        .add_systems(PostUpdate, load_data.run_if(on_event::<LoadData>()))
      ;                      
    }
  }


#[derive(Resource)]
pub struct IOName {
    pub data: String
}
impl IOName {
    pub fn new() -> IOName {
        IOName { data: "".to_string() }
    }
}


#[derive(Event)]
pub struct WriteData;

#[derive(Event)]
pub struct LoadData;

#[derive(Serialize, Deserialize)]
pub struct SavePlaneData {
    pub plane:        PlaneData,
    pub vertex:       Vec<Vertex>
}
impl SavePlaneData {
    pub fn from_pd(pd: &PlaneData) -> Self {
        SavePlaneData{plane: pd.clone(), vertex: Vec::new()}
    }
    
    pub fn spawn(&self,
                 commands:           &mut Commands, 
                 meshes:             &mut ResMut<Assets<Mesh>>,
                 materials:          &mut ResMut<Assets<StandardMaterial>>) -> Entity {

        
        let mut mesh = plane_mesh(&self.plane.subdivisions, &self.plane.dims);

        // Adjust vertices:
        let mut v_clr: Option<Vec<[f32;4]>> = None;
        let mut v_pos: Option<Vec<[f32; 3]>>;

        v_pos = Some(mesh.attribute(Mesh::ATTRIBUTE_POSITION).unwrap().as_float3().unwrap().to_vec());
        if let Some(attr_vcolor) = mesh.attribute(Mesh::ATTRIBUTE_COLOR) {
            if let VertexAttributeValues::Float32x4(vcolors) = attr_vcolor {
                v_clr = Some(vcolors.to_vec());
            }
        } else {
            v_clr = Some(vec![[1.0, 1.0, 1.0, 1.0]; v_pos.as_ref().unwrap().len()]);
        }

        if v_pos.is_some() && v_clr.is_some() {
            for vertex in self.vertex.iter(){
                v_pos.as_mut().unwrap()[vertex.index] = vertex.loc;
                v_clr.as_mut().unwrap()[vertex.index] = vertex.clr;
            }
            mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, v_pos.unwrap());
            mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, v_clr.unwrap());
        } 


        get_mesh_stats(&mesh);
        let entity = commands.spawn((PbrBundle {
            material: materials.add(StandardMaterial{..default()}),
            mesh: meshes.add(mesh),
            transform: Transform::from_translation(self.plane.loc.into()),
            ..default()
            },
            TerrainPlane,
            PickableBundle::default(),
            RaycastPickTarget::default(),
            On::<Pointer<Down>>::send_event::<PickPlane>(),
            self.plane.clone(),
            self.plane.get_aabb()
        )).id();

        return entity;

    }
}



pub fn write_data(vertex: Query<&Vertex>,
                  planes: Query<(&PlaneData, &Children)>,
                  ioname: Res<IOName>) {

    info!("Eriting data to {}", ioname.data);
    let mut v: Vec<SavePlaneData> = Vec::new();
    for (pd, children) in planes.iter(){
        let mut spd = SavePlaneData::from_pd(pd);
        for child in children.iter(){
            if let Ok(p_vertex) = vertex.get(*child){
                spd.vertex.push(*p_vertex);
            }
        }
        v.push(spd);
    }

    let f = File::create(format!("./assets/saves/{}.json", ioname.data)).ok().unwrap();
    let mut writer = BufWriter::new(f);
    let _res = serde_json::to_writer(&mut writer, &v);
    let _res = writer.flush();

}

pub fn load_data(mut commands:      Commands,
                 mut meshes:        ResMut<Assets<Mesh>>,
                 mut materials:     ResMut<Assets<StandardMaterial>>,
                 planes:            Query<Entity, With<PlaneData>>,
                 ioname:            Res<IOName>) {

    for entity in planes.iter(){
        commands.entity(entity).despawn_recursive();
    }

    let path: &str = &format!("./assets/saves/{}.json", ioname.data);
    let data: String = fs::read_to_string(path)
                            .expect(&format!("\n [ERROR io.load] Unable to read file {path}  \n"));
    let vspds: Vec<SavePlaneData> = serde_json::from_str(&data)
                                                .expect(&format!("\n [ERROR io.load] Unable to get data from {path} \n"));
    

    for spd in vspds.iter(){
        spd.spawn(&mut commands, &mut meshes, &mut materials);
    }
                        
}
