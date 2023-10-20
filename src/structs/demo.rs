use crate::structs::demo_header::DemoHeader;
use crate::structs::packet::Packet;
use crate::structs::data_manager::DataManager;

// struct to store all information on the current demo

pub struct Demo {
    pub header: DemoHeader,
    pub packets: Vec<Packet>,
    pub data_manager: DataManager,
}

impl Demo {
    pub fn new() -> Self {
        Self { header: DemoHeader::new(), packets: Vec::new(), data_manager: DataManager::new() }
    }
}
