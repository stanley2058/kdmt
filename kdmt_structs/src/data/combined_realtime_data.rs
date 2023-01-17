use super::request_type::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
struct CombinedLatency {
    mean: f64,
    div_base: f64,
    cv: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
struct CombinedRealtimeData {
    _id: Option<String>,
    unique_service_name: String,
    unique_endpoint_name: String,
    latest_timestamp: i64,
    method: RequestType,
    service: String,
    namespace: String,
    version: String,
    latency: CombinedLatency,
    status: String,
    request_body: Option<String>,
    request_schema: Option<String>,
    request_content_type: Option<String>,
    response_body: Option<String>,
    response_schema: Option<String>,
    response_content_type: Option<String>,
    avg_replica: f64,
}
