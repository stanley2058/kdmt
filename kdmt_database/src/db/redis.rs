use super::database_interface::KdmtDatabaseAdaptor;
use kdmt_structs::data::base::ToEndpoint;
use kdmt_structs::data::{
    combined_realtime_data::CombinedRealtimeData, endpoint_data_type::EndpointDataType,
    endpoint_dependency::EndpointDependency, record::Record,
};
use redis::{Client, Commands, Connection};
use serde::de::DeserializeOwned;
use std::collections::HashSet;
use std::error::Error;
use std::sync::{Arc, Mutex, MutexGuard};
use std::time::Duration;
use uuid::Uuid;

static PREFIX_RECORD: &str = "kdmt_record:";
static PREFIX_CRL: &str = "kdmt_crl:";
static PREFIX_DATATYPE: &str = "kdmt_datatype:";
static PREFIX_DEPENDENCY: &str = "kdmt_dependency:";

pub struct Redis {
    connection: Arc<Mutex<Connection>>,
}

impl Redis {
    fn keys_to_values<T: DeserializeOwned>(
        &self,
        connection: &mut MutexGuard<Connection>,
        keys: &Vec<String>,
    ) -> Result<Vec<T>, Box<dyn Error>> {
        let res = connection
            .mget::<_, Vec<String>>(keys)?
            .into_iter()
            .map(|r| serde_json::from_str::<T>(&r))
            .filter(|r| r.is_ok())
            .map(|r| r.unwrap())
            .collect::<Vec<_>>();
        Ok(res)
    }

    fn delete_difference_set<T: ToEndpoint>(
        &self,
        connection: &mut MutexGuard<Connection>,
        prefix: String,
        new_data: &Vec<T>,
    ) -> Result<(), Box<dyn Error>> {
        let existing_keys = connection
            .scan_match::<_, String>(prefix)?
            .collect::<HashSet<_>>();
        let new_keys = new_data
            .iter()
            .map(|d| d.to_endpoint().unique_endpoint_name.clone())
            .collect::<HashSet<_>>();
        let to_remove = &existing_keys - &new_keys;
        let _: () = connection.del(to_remove)?;
        Ok(())
    }
}

impl KdmtDatabaseAdaptor<Redis> for Redis {
    fn connect() -> Result<Redis, Box<dyn Error>> {
        let url = std::env::var("REDIS_URI")?;
        let client = Client::open(url)?;
        let connection = client.get_connection()?;

        Ok(Redis {
            connection: Arc::new(Mutex::new(connection)),
        })
    }

    fn save_record(
        &self,
        record: &Record,
        timeout: Option<Duration>,
    ) -> Result<(), Box<dyn Error>> {
        let connection = Arc::clone(&self.connection);
        let mut con = connection.lock().unwrap();
        let json = serde_json::to_string(record)?;
        let id = format!("{}{}", PREFIX_RECORD, Uuid::new_v4());
        let timeout_secs = if let Some(t) = timeout {
            t.as_secs() as usize
        } else {
            30
        };
        let _: () = con.set_ex(id, json, timeout_secs)?;
        Ok(())
    }

    fn get_records(&self) -> Result<Vec<Record>, Box<dyn Error>> {
        let connection = Arc::clone(&self.connection);
        let mut con = connection.lock().unwrap();
        let keys = con
            .scan_match::<_, String>(format!("{}*", PREFIX_RECORD))?
            .collect::<Vec<_>>();
        Ok(self.keys_to_values::<Record>(&mut con, &keys)?)
    }

    fn save_combined_realtime_data(
        &self,
        crl_data: &Vec<CombinedRealtimeData>,
    ) -> Result<(), Box<dyn Error>> {
        let connection = Arc::clone(&self.connection);
        let mut con = connection.lock().unwrap();

        self.delete_difference_set(&mut con, format!("{}*", PREFIX_CRL), crl_data)?;

        for data in crl_data.iter() {
            let id = format!("{}{}", PREFIX_CRL, data.unique_endpoint_name);
            let _: () = con.set(id, serde_json::to_string(&data)?)?;
        }
        Ok(())
    }

    fn get_combined_realtime_data(&self) -> Result<Vec<CombinedRealtimeData>, Box<dyn Error>> {
        let connection = Arc::clone(&self.connection);
        let mut con = connection.lock().unwrap();
        let keys = con
            .scan_match::<_, String>(format!("{}*", PREFIX_CRL))?
            .collect::<Vec<_>>();
        Ok(self.keys_to_values::<CombinedRealtimeData>(&mut con, &keys)?)
    }

    fn save_endpoint_datatype(
        &self,
        datatypes: &Vec<EndpointDataType>,
    ) -> Result<(), Box<dyn Error>> {
        let connection = Arc::clone(&self.connection);
        let mut con = connection.lock().unwrap();

        self.delete_difference_set(&mut con, format!("{}*", PREFIX_DATATYPE), datatypes)?;

        for data in datatypes.iter() {
            let id = format!("{}{}", PREFIX_DATATYPE, data.unique_endpoint_name);
            let _: () = con.set(id, serde_json::to_string(&data)?)?;
        }
        Ok(())
    }

    fn get_endpoint_datatype(&self) -> Result<Vec<EndpointDataType>, Box<dyn Error>> {
        let connection = Arc::clone(&self.connection);
        let mut con = connection.lock().unwrap();
        let keys = con
            .scan_match::<_, String>(format!("{}*", PREFIX_DATATYPE))?
            .collect::<Vec<_>>();
        Ok(self.keys_to_values::<EndpointDataType>(&mut con, &keys)?)
    }

    fn save_endpoint_dependencies(
        &self,
        dependencies: &Vec<EndpointDependency>,
    ) -> Result<(), Box<dyn Error>> {
        let connection = Arc::clone(&self.connection);
        let mut con = connection.lock().unwrap();

        self.delete_difference_set(
            &mut con,
            format!("{}*", PREFIX_DEPENDENCY),
            &dependencies
                .iter()
                .map(|d| d.endpoint.clone())
                .collect::<Vec<_>>(),
        )?;

        for data in dependencies.iter() {
            let id = format!(
                "{}{}",
                PREFIX_DEPENDENCY, data.endpoint.unique_endpoint_name
            );
            let _: () = con.set(id, serde_json::to_string(&data)?)?;
        }
        Ok(())
    }

    fn get_endpoint_dependencies(&self) -> Result<Vec<EndpointDependency>, Box<dyn Error>> {
        let connection = Arc::clone(&self.connection);
        let mut con = connection.lock().unwrap();
        let keys = con
            .scan_match::<_, String>(format!("{}*", PREFIX_DEPENDENCY))?
            .collect::<Vec<_>>();
        Ok(self.keys_to_values::<EndpointDependency>(&mut con, &keys)?)
    }
}
