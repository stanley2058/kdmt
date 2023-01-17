use chrono::prelude::DateTime;
use kdmt_structs::data::record::{Record, RecordType};
use kdmt_structs::data::request_type::RequestType;
use regex::Regex;
use std::str::FromStr;
use std::sync::{Arc, Once};
static INIT: Once = Once::new();

pub struct LogMatcher {
    metadata_matcher: Option<Arc<Regex>>,
    status_matcher: Option<Arc<Regex>>,
    path_matcher: Option<Arc<Regex>>,
    content_type_matcher: Option<Arc<Regex>>,
    body_matcher: Option<Arc<Regex>>,
}

pub enum MatcherType {
    Metadata,
    Status,
    Path,
    ContentType,
    Body,
}

static mut WRAPPER: LogMatcher = LogMatcher {
    metadata_matcher: None,
    status_matcher: None,
    path_matcher: None,
    content_type_matcher: None,
    body_matcher: None,
};

static RE_METADATA: &str =
    r"\[(Request|Response) ([[:alnum:]-_]+)/([[:alnum:]_]+)/([[:alnum:]_]+)/([[:alnum:]_]+)\]";
static RE_STATUS: &str = r"\[Status\] ([0-9]+)";
static RE_PATH: &str = r"(GET|POST|PUT|DELETE|PATCH|HEAD|OPTIONS) ([^\]]+)";
static RE_CONTENT_TYPE: &str = r"\[ContentType ([^\]]*)]";
static RE_BODY: &str = r"\[Body\] (.*)";

impl LogMatcher {
    unsafe fn init() {
        LogMatcher::create_matcher(&mut WRAPPER.metadata_matcher, RE_METADATA);
        LogMatcher::create_matcher(&mut WRAPPER.status_matcher, RE_STATUS);
        LogMatcher::create_matcher(&mut WRAPPER.path_matcher, RE_PATH);
        LogMatcher::create_matcher(&mut WRAPPER.content_type_matcher, RE_CONTENT_TYPE);
        LogMatcher::create_matcher(&mut WRAPPER.body_matcher, RE_BODY);
    }

    fn create_matcher(matcher: &mut Option<Arc<Regex>>, pattern: &str) {
        if matcher.is_some() {
            return;
        }
        let re = Regex::new(pattern).unwrap();
        matcher.replace(Arc::new(re));
    }

    pub fn matcher(r#type: MatcherType) -> Arc<Regex> {
        INIT.call_once(|| unsafe { LogMatcher::init() });
        let matcher = unsafe {
            match r#type {
                MatcherType::Metadata => WRAPPER.metadata_matcher.as_ref().unwrap(),
                MatcherType::Status => WRAPPER.status_matcher.as_ref().unwrap(),
                MatcherType::Path => WRAPPER.path_matcher.as_ref().unwrap(),
                MatcherType::ContentType => WRAPPER.content_type_matcher.as_ref().unwrap(),
                MatcherType::Body => WRAPPER.body_matcher.as_ref().unwrap(),
            }
        };
        Arc::clone(matcher)
    }

    pub fn pattern_match(matcher: Arc<Regex>, str: &str) -> Vec<&str> {
        let captures = matcher.captures(str);
        if captures.is_none() {
            return Vec::new();
        }
        captures
            .unwrap()
            .iter()
            .map(|m| m.unwrap().as_str())
            .collect::<Vec<&str>>()
    }

    pub fn parse_log(log: String) -> Result<Record, ()> {
        let splits = log.split("\t").collect::<Vec<&str>>();
        if splits.len() < 4 {
            return Err(());
        }
        let time = DateTime::parse_from_rfc3339(splits[0]);
        if time.is_err() {
            return Err(());
        }
        let namespace = String::from(splits[1]);
        let pod_name = String::from(splits[2]);

        let log_body = splits[3];
        let metadata =
            LogMatcher::pattern_match(LogMatcher::matcher(MatcherType::Metadata), log_body);

        if metadata.len() < 6 {
            return Err(());
        }
        let record_type = RecordType::from_str(metadata[1]);
        if record_type.is_err() {
            return Err(());
        }

        let body = Self::pattern_match(Self::matcher(MatcherType::Body), log_body)
            .iter()
            .map(|x| String::from(*x))
            .nth(1);
        let content_type = Self::pattern_match(Self::matcher(MatcherType::ContentType), log_body)
            .iter()
            .map(|x| String::from(*x))
            .nth(1);
        let status = Self::pattern_match(Self::matcher(MatcherType::Status), log_body)
            .iter()
            .map(|x| String::from(*x))
            .nth(1);
        let tmp_method_and_path = Self::pattern_match(Self::matcher(MatcherType::Path), log_body);
        let mut method_and_path = tmp_method_and_path
            .iter()
            .map(|x| String::from(*x))
            .into_iter();

        let method = if let Some(m) = method_and_path.nth(1) {
            Some(RequestType::from_str(&m).unwrap())
        } else {
            None
        };

        let record = Record {
            namespace,
            pod_name,
            r#type: record_type.unwrap(),
            request_id: String::from(metadata[2]),
            trace_id: String::from(metadata[3]),
            span_id: String::from(metadata[4]),
            parent_span_id: String::from(metadata[5]),
            timestamp: time.unwrap().timestamp_millis(),
            body,
            content_type,
            status,
            method,
            path: method_and_path.nth(0),
        };

        Ok(record)
    }
}
