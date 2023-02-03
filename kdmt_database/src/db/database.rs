use super::{cassandra::Cassandra, database_interface::KdmtDatabaseAdaptor, redis::Redis};

pub struct KdmtDatabase {
    database: Box<dyn KdmtDatabaseAdaptor>,
}

impl KdmtDatabase {
    fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let mut redis = Redis::new();
        let mut cassandra = Cassandra::new();

        let database: Box<dyn KdmtDatabaseAdaptor> = if let Ok(_) = redis.connect() {
            Box::new(redis)
        } else if let Ok(_) = cassandra.connect() {
            Box::new(cassandra)
        } else {
            panic!()
        };

        Ok(KdmtDatabase { database })
    }
}
