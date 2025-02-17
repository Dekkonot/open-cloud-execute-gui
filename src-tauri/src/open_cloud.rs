use serde::{Deserialize, Serialize, Serializer};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OpenCloudExecutionTask {
    pub path: String,
    pub user: String,
    pub state: OpenCloudState,
    pub script: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FullOpenCloudExecutionTask {
    #[serde(flatten)]
    pub base_task: OpenCloudExecutionTask,

    pub create_time: String,
    pub update_time: String,

    pub output: Option<OpenCloudOutput>,
    pub error: Option<OpenCloudError>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OpenCloudOutput {
    results: Vec<Value>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OpenCloudError {
    pub code: String,
    pub message: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OpenCloudTaskUpload {
    pub script: String,
    #[serde(serialize_with = "duration_serializer")]
    pub timeout: f64,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OpenCloudState {
    StateUnspecified,
    Queued,
    Processing,
    Cancelled,
    Complete,
    Failed,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct OpenCloudLogs {
    #[serde(rename = "luauExecutionSessionTaskLogs")]
    pub task_logs: Vec<OpenCloudLog>,
    pub next_page_token: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct OpenCloudLog {
    pub path: String,
    pub messages: Vec<String>,
    pub structured_messages: Option<Vec<StructuredMessage>>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct StructuredMessage {
    pub message: String,
    pub create_time: String,
    pub message_type: MessageType,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MessageType {
    Info,
    Output,
    Warning,
    Error,
}

fn duration_serializer<S: Serializer>(value: &f64, serializer: S) -> Result<S::Ok, S::Error> {
    serializer.serialize_str(&format!("{value:.9}s"))
}
