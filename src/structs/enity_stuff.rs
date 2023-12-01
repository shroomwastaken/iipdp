// this entire file is pretty much the programming equivalent of a war crime

use crate::structs::utils::ServerClass;
use crate::structs::datatables_manager::FlattenedProp;
use crate::structs::utils::{Vec2, Vec3};
use crate::structs::datatables_manager::DataTablesManager;
use crate::structs::netsvc_types::SvcPacketEntities;

#[derive(Debug, Clone, PartialEq)]
// (Single/ArrEntProp, f_prop, offset, bitlength)
pub enum EntityProperty {
    None,
    SingleEntProp(SingleEntProp, FlattenedProp, i32, i32),
    ArrEntProp(ArrEntProp, FlattenedProp, i32, i32),
}

impl EntityProperty {
    pub fn update_array_prop(&mut self, other: &mut EntityProperty) {
        match (self, other) {
            (EntityProperty::ArrEntProp(arr1, _, _, _), EntityProperty::ArrEntProp(arr2, _, _, _)) => {
                match (arr1, arr2) {
                    (ArrEntProp::IntProp(v1), ArrEntProp::IntProp(ref mut v2)) => {
                        v2.append(v1);
                    }
                    (ArrEntProp::FloatProp(v1), ArrEntProp::FloatProp(ref mut v2)) => {
                        v2.append(v1);
                    }
                    (ArrEntProp::Vector2Prop(v1), ArrEntProp::Vector2Prop(ref mut v2)) => {
                        v2.append(v1);
                    }
                    (ArrEntProp::Vector3Prop(v1), ArrEntProp::Vector3Prop(ref mut v2)) => {
                        v2.append(v1);
                    }
                    (ArrEntProp::StringProp(v1), ArrEntProp::StringProp(ref mut v2)) => {
                        v2.append(v1);
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }

    pub fn copy_array_prop(&self) -> EntityProperty {
        match self {
            EntityProperty::ArrEntProp(v, f, offset, bitlength) => {
                return EntityProperty::ArrEntProp(v.clone(), f.clone(), *offset, *bitlength);
            },
            _ => { panic!("WHAT IS HAPPENING AHHHHHHHH") },
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum SingleEntProp {
    IntProp(i32),
    FloatProp(f32),
    Vector2Prop(Vec2),
    Vector3Prop(Vec3),
    StringProp(String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum ArrEntProp {
    IntProp(Vec<i32>),
    FloatProp(Vec<f32>),
    Vector2Prop(Vec<Vec2>),
    Vector3Prop(Vec<Vec3>),
    StringProp(Vec<String>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Entity {
    pub server_class: ServerClass,
    pub serial: i32,
    pub props: Vec<EntityProperty>,
    pub in_pvs: bool,
}

impl Entity {
    pub fn new() -> Self {
        Self { server_class: ServerClass::new(), serial: 0, props: Vec::new(), in_pvs: true }
    }
}

pub struct EntityBaselines {
    pub baselines: Vec<(ServerClass, Vec<EntityProperty>)>,
}

impl EntityBaselines {
    pub fn new(cap: i32) -> Self {
        let mut new_self = EntityBaselines { baselines: Vec::new() };
        new_self.clear_baseline_state(cap);
        return new_self;
    }

    pub fn clear_baseline_state(&mut self, new_cap: i32) {
        self.baselines.resize(new_cap as usize, (ServerClass::new(), Vec::new()));
    }

    // this is the best and the worst thing ive ever written at the same time
    pub fn update_baseline(&mut self, server_class: ServerClass, props: Vec<(i32, EntityProperty)>, ent_prop_count: i32) {
        let i = server_class.datatable_id as usize;
        if self.baselines[i] == (ServerClass::new(), Vec::new()) {
            self.baselines[i].0 = server_class;
            let mut new_vec: Vec<EntityProperty> = Vec::new();
            new_vec.resize(ent_prop_count as usize, EntityProperty::None);
            self.baselines[i].1 = new_vec;
        }
        for (prop_index, mut from) in props {
            let to: &mut EntityProperty = &mut self.baselines[i].1[prop_index as usize];
            if let EntityProperty::ArrEntProp(..) = to {
                match from {
                    EntityProperty::ArrEntProp(..)=> {
                        from.update_array_prop(to);
                    },
                    _ => {}
                }
            } else if let EntityProperty::ArrEntProp(..) = from {
                match to {
                    EntityProperty::ArrEntProp(..) => {
                        *to = from.copy_array_prop();
                    },
                    _ => {}
                }
            } else {
                *to = from;
            }
        }
    }

    pub fn ent_from_baseline(&mut self, server_class: ServerClass, serial: i32, dt_mgr: &DataTablesManager) -> Entity {
        let class_index = server_class.datatable_id as usize;
        if self.baselines[class_index].1.is_empty() {
            let f_props = &dt_mgr.prop_lookup[class_index].1;
            let mut new_vec: Vec<EntityProperty> = Vec::new();
            new_vec.resize(f_props.len(), EntityProperty::None);
            self.baselines[class_index].1 = new_vec;
        }
        let mut new_props: Vec<EntityProperty> = Vec::new();
        new_props.resize(self.baselines[class_index].1.len(), EntityProperty::None);
        for i in 0..new_props.len() {
            if let EntityProperty::ArrEntProp(..) = self.baselines[class_index].1[i] {
                new_props[i] = self.baselines[class_index].1[i].copy_array_prop();
            } else {
                new_props[i] = self.baselines[class_index].1[i].clone();
            }
        }
        
        return Entity { server_class, serial, props: new_props, in_pvs: true }
    }
}

// video about what a pvs is: https://youtu.be/IfCRHSIg6zo

pub enum EntityUpdate {
    Delta(ServerClass, i32, Vec<(i32, EntityProperty)>), // server_class, ent_index, props
    EnterPvs(ServerClass, i32, bool, i32, Vec<(i32, EntityProperty)>), // server_class, serial, new, ent_index, props
    LeavePvs(ServerClass, i32, bool), // server_class, index, delete
}

pub struct EntitySnapshot {
    pub ents: Vec<Entity>,
    pub engine_tick: i32,
}

impl EntitySnapshot {
    pub fn new() -> Self {
        let mut new_self = EntitySnapshot { ents: Vec::new(), engine_tick: 0 };
        new_self.clear_entity_state();
        return new_self;
    }

    pub fn clear_entity_state(&mut self) {
        let prev_ents_len = self.ents.len();
        self.ents = Vec::new();
        self.ents.resize(prev_ents_len, Entity::new());
    }

    pub fn process_updates(&mut self, dt_mgr: &DataTablesManager, baselines: &mut EntityBaselines, msg: &SvcPacketEntities, update: EntityUpdate) {
        match update {
            EntityUpdate::EnterPvs(server_class, serial, new, ent_index, props) => {
                if new {
                    self.ents.insert(ent_index as usize, baselines.ent_from_baseline(server_class.clone(), serial, dt_mgr))
                }
                self.ents[ent_index as usize].in_pvs = true;
                self.process_delta((ent_index, props));
                if msg.update_baseline { // if update baseline then set the current baseline to the ent props
                    let mut non_null_props: Vec<(i32, EntityProperty)> = Vec::new();
                    for i in 0..self.ents[ent_index as usize].props.len() {
                        if self.ents[ent_index as usize].props[i] == EntityProperty::None {
                            non_null_props.push((i as i32, self.ents[ent_index as usize].props[i].clone()));
                        }
                    }
                    baselines.update_baseline(
                        server_class,
                        non_null_props,
                        self.ents[ent_index as usize].props.len() as i32,
                    );
                }
            },
            EntityUpdate::Delta(_, ent_index, props) => {
                self.process_delta((ent_index, props))
            },
            EntityUpdate::LeavePvs(_, index, delete) => {
                if delete {
                    self.ents[index as usize] = Entity::new();
                } else {
                    self.ents[index as usize].in_pvs = false;
                }
            }
        }
    }

    pub fn process_delta(&mut self, delta: (i32, Vec<(i32, EntityProperty)>)) {
        for (prop_index, mut prop) in delta.1 {
            let old: &mut EntityProperty = &mut self.ents[delta.0 as usize].props[prop_index as usize];
            if let EntityProperty::ArrEntProp(..) = prop {
                if *old == EntityProperty::None {
                    *old = prop.copy_array_prop();
                } else {
                    prop.update_array_prop(old);
                }
            } else {
                *old = prop;
            }
        }
    }
}