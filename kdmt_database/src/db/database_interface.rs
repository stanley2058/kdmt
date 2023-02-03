use std::error::Error;
use std::time::Duration;

use kdmt_structs::data::{
    combined_realtime_data::CombinedRealtimeData, endpoint_data_type::EndpointDataType,
    endpoint_dependency::EndpointDependency, record::Record,
};

pub trait KdmtDatabaseAdaptor<T> {
    fn connect() -> Result<T, Box<dyn Error>>;
    fn save_record(&self, record: &Record, timeout: Option<Duration>)
        -> Result<(), Box<dyn Error>>;
    fn get_records(&self) -> Result<Vec<Record>, Box<dyn Error>>;
    fn save_combined_realtime_data(
        &self,
        crl_data: &Vec<CombinedRealtimeData>,
    ) -> Result<(), Box<dyn Error>>;
    fn get_combined_realtime_data(&self) -> Result<Vec<CombinedRealtimeData>, Box<dyn Error>>;
    fn save_endpoint_datatype(
        &self,
        datatypes: &Vec<EndpointDataType>,
    ) -> Result<(), Box<dyn Error>>;
    fn get_endpoint_datatype(&self) -> Result<Vec<EndpointDataType>, Box<dyn Error>>;
    fn save_endpoint_dependencies(
        &self,
        dependencies: &Vec<EndpointDependency>,
    ) -> Result<(), Box<dyn Error>>;
    fn get_endpoint_dependencies(&self) -> Result<Vec<EndpointDependency>, Box<dyn Error>>;
}
