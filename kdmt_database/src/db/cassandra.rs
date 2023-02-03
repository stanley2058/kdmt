use super::database_interface::KdmtDatabaseAdaptor;
use kdmt_structs::data::{
    combined_realtime_data::CombinedRealtimeData, endpoint_data_type::EndpointDataType,
    endpoint_dependency::EndpointDependency, record::Record,
};
use std::error::Error;
use std::time::Duration;

pub struct Cassandra {}

impl Cassandra {
    pub fn new() -> Self {
        Cassandra {}
    }
}

impl KdmtDatabaseAdaptor for Cassandra {
    fn connect(&mut self) -> Result<(), Box<dyn Error>> {
        todo!()
    }
    fn save_record(
        &self,
        record: &Record,
        timeout: Option<Duration>,
    ) -> Result<(), Box<dyn Error>> {
        todo!()
    }
    fn get_records(&self) -> Result<Vec<Record>, Box<dyn Error>> {
        todo!()
    }
    fn save_combined_realtime_data(
        &self,
        crl_data: &Vec<CombinedRealtimeData>,
    ) -> Result<(), Box<dyn Error>> {
        todo!()
    }
    fn get_combined_realtime_data(&self) -> Result<Vec<CombinedRealtimeData>, Box<dyn Error>> {
        todo!()
    }
    fn save_endpoint_datatype(
        &self,
        datatypes: &Vec<EndpointDataType>,
    ) -> Result<(), Box<dyn Error>> {
        todo!()
    }
    fn get_endpoint_datatype(&self) -> Result<Vec<EndpointDataType>, Box<dyn Error>> {
        todo!()
    }
    fn save_endpoint_dependencies(
        &self,
        dependencies: &Vec<EndpointDependency>,
    ) -> Result<(), Box<dyn Error>> {
        todo!()
    }
    fn get_endpoint_dependencies(&self) -> Result<Vec<EndpointDependency>, Box<dyn Error>> {
        todo!()
    }
}
