use std::collections::{HashMap, HashSet};
use crate::structs::{
    packet_data_types::DataTables,
    send_table::{
        SendTable, SendPropType, PropFlag, SendTableProp
    },
    data_manager::DataManager, utils::ServerClass
};

// fucking kill me
// this code sucks dont look at it
// so many damn clone() calls

// https://github.com/UncraftedName/UntitledParser/blob/master/DemoParser/src/Parser/GameState/DataTablesManager.cs#L34

pub struct DataTablesManager {
    pub datatables: DataTables,
    pub server_class_bits: i32,
    pub prop_lookup: Vec<(ServerClass, Vec<FlattenedProp>)>,
}

impl DataTablesManager {
    pub fn new(datatables: DataTables, server_class_bits: i32) -> Self {
        Self { datatables, server_class_bits, prop_lookup: Vec::new() }
    }

    pub fn flatten_classes(&mut self, data_mgr: &DataManager) {
        let mut table_lookup: HashMap<&str, &SendTable> = HashMap::new();
        // cloning this so it lives for the entire scope of this function
        let send_tables_clone = self.datatables.send_tables.clone(); // i hate this but it has to be done
        for table in &send_tables_clone {
            table_lookup.insert(&table.name, &table);
        }
    
        for server_class in &data_mgr.server_class_info {
            let table: &SendTable = &send_tables_clone[server_class.datatable_id as usize];
            let exc = self.gather_excludes(&table_lookup, &table);
            self.gather_props(&table_lookup, &exc, &table, server_class);
            let mut f_props = self.prop_lookup[server_class.datatable_id as usize].1.clone();
            self.sort_props(&mut f_props);
        }
    }
    
    fn gather_excludes(&mut self, table_lookup: &HashMap<&str, &SendTable>, table: &SendTable) -> HashSet<(String, String)> {
        let mut excludes: HashSet<(String, String)> = HashSet::new();
    
        for prop in &table.prop_list {
            if prop.send_prop_type == SendPropType::DataTable {
                excludes.extend(self.gather_excludes(table_lookup, &table_lookup[prop.exclude_dt_name.clone().unwrap().as_str()]));
            } else if prop.flags.contains(PropFlag::Exclude) {
                excludes.insert((prop.name.clone(), prop.exclude_dt_name.clone().unwrap()));
            }
        }
    
        return excludes;
    }
    
    fn iterate_props(&mut self, table_lookup: &HashMap<&str, &SendTable>, table: &SendTable, excludes: &HashSet<(String, String)>, server_class: &ServerClass, f_props: &mut Vec<FlattenedProp>) {
        for i in 0..table.prop_list.len() {
            let prop = &table.prop_list[i];
            if prop.flags.contains(PropFlag::Exclude) || prop.flags.contains(PropFlag::InsideArray) || excludes.contains(&(prop.name.clone(), table.name.clone())) {
                continue;
            }
            if prop.send_prop_type == SendPropType::DataTable {
                let sub_table = table_lookup[prop.exclude_dt_name.clone().unwrap().as_str()];
                if prop.flags.contains(PropFlag::Collapsible) {
                    self.iterate_props(table_lookup, sub_table, excludes, server_class, f_props);
                } else {
                    self.gather_props(table_lookup, excludes, sub_table, server_class);
                }
            } else {
                f_props.push(
                    FlattenedProp::new(
                        prop.name.clone(),
                        prop.clone(),
                        if prop.send_prop_type == SendPropType::Array { Some(table.prop_list[i - 1].clone()) } else { None }
                    )
                );
            }
        }
    }

    fn gather_props(&mut self, table_lookup: &HashMap<&str, &SendTable>, excludes: &HashSet<(String, String)>, table: &SendTable, server_class: &ServerClass) {
        let mut f_props: Vec<FlattenedProp> = Vec::new();
        self.iterate_props(table_lookup, table, excludes, server_class, &mut f_props);
        if server_class.datatable_id == self.prop_lookup.len() as i32 {
            self.prop_lookup.push((server_class.clone(), Vec::new()));
        }
        self.prop_lookup[server_class.datatable_id as usize].1.append(&mut f_props);
    }

    fn sort_props(&mut self, f_props: &mut Vec<FlattenedProp>) {
        // theres some magic if the demo has demo protocol 4 buuuuut
        // im not doing that yet so itll have to wait :)
        let mut start = 0;
        for i in 0..f_props.len() {
            if f_props[i].prop_info.flags.contains(PropFlag::ChangesOften) {
                f_props.swap(i, start);
                start += 1;
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct FlattenedProp {
    pub name: String,
    pub prop_info: SendTableProp,
    pub array_element_prop_info: Option<SendTableProp>
}

impl FlattenedProp {
    pub fn new(name: String, prop_info: SendTableProp, array_element_prop_info: Option<SendTableProp>) -> Self {
        Self {
            name,
            prop_info,
            array_element_prop_info,
        }
    }
}