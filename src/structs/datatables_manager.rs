use std::collections::{HashMap, HashSet};
use crate::structs::{
    packet_data_types::DataTables,
    send_table::{
        SendTable, SendPropType, PropFlag, SendTableProp
    },
    data_manager::DataManager, utils::ServerClass
};

// fucking kill me

// https://github.com/UncraftedName/UntitledParser/blob/master/DemoParser/src/Parser/GameState/DataTablesManager.cs#L34

pub struct DataTablesManager {
    pub datatables: DataTables,
    pub server_class_bits: i32,
    pub prop_lookup: Vec<(ServerClass, Vec<FlattenedProp>)>
}

impl DataTablesManager {
    pub fn new(datatables: DataTables, server_class_bits: i32) -> Self {
        Self { datatables, server_class_bits, prop_lookup: Vec::new() }
    }

    pub fn flatten_classes(&mut self, data_mgr: &DataManager) {
        let mut table_lookup: HashMap<&str, &SendTable> = HashMap::new();
        for table in &self.datatables.send_tables {
            table_lookup.insert(&table.name, table);
        }
    
        println!("{:#?}", table_lookup);
        // this shit doesnt work
        // for server_class in &data_mgr.server_class_info {
        //     let table: &SendTable = &self.datatables.send_tables[server_class.datatable_id as usize];
        //     let gathered_excludes = self.gather_excludes(&table_lookup, &table);
        //     self.gather_props(&table_lookup, gathered_excludes, table, server_class);
        // }
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
    
    fn gather_props(&mut self, table_lookup: &HashMap<&str, &SendTable>, excludes: HashSet<(String, String)>, table: &SendTable, server_class: &ServerClass) {
        
    }
}
pub struct FlattenedProp {
    pub name: String,
    pub prop_info: SendTableProp,
    pub array_element_prop_info: Option<SendTableProp>
}
