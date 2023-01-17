use crate::log_matcher::matcher::LogMatcher;
use rocket::http::Status;

#[post("/log", data = "<log>")]
pub fn create_log(log: String) -> Status {
    let record = LogMatcher::parse_log(log);

    // TODO: push to a queue to save to the database
    println!("{:#?}", record);

    if record.is_err() {
        return Status::BadRequest;
    }
    Status::Ok
}

#[test]
fn test_create_log() {
    let res = create_log(String::from("2023-01-03T06:03:38.005654Z\tpdas\tuser-service-abc123-def456\t[Request 669084db-e52d-9825-8d03-aab35afa6f4a/dad62e0cb93a980cc6bba3d0762fefc8/d40b8bb597882141/c6bba3d0762fefc8] [GET /internal/user/verify] [ContentType application/json]"));
    create_log(String::from("2023-01-03T06:03:38.005671Z\tpdas\tuser-service-abc123-def456\t[Response 669084db-e52d-9825-8d03-aab35afa6f4a/dad62e0cb93a980cc6bba3d0762fefc8/3f0ebe8b94ab3156/ab22aec8ee300093] [Status] 200 [ContentType application/json] [Body] null"));
    assert_eq!(res, Status::Ok);
}
