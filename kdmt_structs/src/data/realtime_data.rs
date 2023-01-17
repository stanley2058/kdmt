use super::request_type::RequestType;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
struct RealtimeData {
    unique_service_name: String,
    unique_endpoint_name: String,
    timestamp: i64,
    method: RequestType,
    service: String,
    namespace: String,
    version: String,
    latency: u64,
    status: String,
    request_body: Option<String>,
    request_content_type: Option<String>,
    response_body: Option<String>,
    response_content_type: Option<String>,
    replica: u32,
}
