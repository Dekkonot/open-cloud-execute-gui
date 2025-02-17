use std::{mem::take, sync::LazyLock, time::Duration};

use reqwest::{header, Client, ClientBuilder, Method};
use tokio::time::{sleep_until, timeout, Instant};

use crate::open_cloud::{
    FullOpenCloudExecutionTask, OpenCloudError, OpenCloudExecutionTask, OpenCloudLogs,
    OpenCloudState, OpenCloudTaskUpload, StructuredMessage,
};
use crate::{Error, Result};

const MAX_TIMEOUT_SECS: u64 = 60 * 5;
const TIMEOUT: Duration = Duration::from_secs(MAX_TIMEOUT_SECS);
const RETRY_DELAY: Duration = Duration::from_secs(1);
const MAX_DELAY: Duration = Duration::from_secs(60);
const BACKOFF_MULTIPLIER: f32 = 1.5;

const USER_AGENT: &str = "Dekkonot/OpenCloudExecutionApp 0.0.0";
const BASE_URL: &str = "https://apis.roblox.com/cloud/v2";

static CLIENT: LazyLock<Client> = LazyLock::new(|| {
    let mut headers = header::HeaderMap::with_capacity(1);
    headers.insert(
        header::CONTENT_TYPE,
        header::HeaderValue::from_static("application/json"),
    );
    // TODO: Force Rust TLS
    ClientBuilder::new()
        .user_agent(USER_AGENT)
        .default_headers(headers)
        .build()
        .unwrap()
});

fn api_header(api_key: &str) -> Result<header::HeaderValue> {
    let mut header = header::HeaderValue::from_str(api_key)?;
    header.set_sensitive(true);
    Ok(header)
}

pub fn create_task_url(place_id: &str, universe_id: &str, version_number: Option<&str>) -> String {
    match version_number {
        Some(number) => {
            format!("{BASE_URL}/universes/{universe_id}/places/{place_id}/versions/{number}/luau-execution-session-tasks")
        }
        None => {
            format!(
                "{BASE_URL}/universes/{universe_id}/places/{place_id}/luau-execution-session-tasks"
            )
        }
    }
}

pub async fn create_task(
    api_key: &str,
    url: &str,
    script: &str,
    script_timeout: Option<f64>,
) -> Result<OpenCloudExecutionTask> {
    let request_body = serde_json::to_string(&OpenCloudTaskUpload {
        script: script.to_string(),
        timeout: script_timeout.unwrap_or(MAX_TIMEOUT_SECS as f64),
    })?;

    let result = CLIENT
        .request(Method::POST, url)
        .header("x-api-key", api_header(api_key)?)
        .body(request_body)
        .send()
        .await?;

    if result.status().is_success() {
        println!("[SUCCESS] {} {}", result.status(), url);
        let json: OpenCloudExecutionTask = result.json().await?;
        Ok(json)
    } else {
        println!("[ERROR  ] {} {}", result.status(), url);
        let error: OpenCloudError = result.json().await?;
        Err(Error::new(format!(
            "Creating OpenCloud task failed:\n{}",
            error.message
        )))
    }
}

async fn query_task(api_key: &str, path: &str) -> Result<FullOpenCloudExecutionTask> {
    let url = format!("{BASE_URL}/{}", path);
    let result = CLIENT
        .request(Method::GET, &url)
        .header("x-api-key", api_header(api_key)?)
        .send()
        .await?;

    if result.status().is_success() {
        println!("[SUCCESS] {} {}", result.status(), &url);
        let json: FullOpenCloudExecutionTask = result.json().await?;
        Ok(json)
    } else {
        println!("[ERROR  ] {} {}", result.status(), &url);
        let error: OpenCloudError = result.json().await?;
        Err(Error::new(format!(
            "Querying OpenCloud task failed:\n{}",
            error.message
        )))
    }
}

pub async fn await_task(api_key: &str, path: &str) -> Result<FullOpenCloudExecutionTask> {
    async fn await_task_inner(api_key: &str, path: &str) -> Result<FullOpenCloudExecutionTask> {
        let mut retry_time = RETRY_DELAY;

        let mut task_status = query_task(api_key, path).await?;
        while task_status.base_task.state == OpenCloudState::Processing {
            sleep_until(Instant::now() + retry_time).await;
            task_status = query_task(api_key, path).await?;

            retry_time = retry_time.mul_f32(BACKOFF_MULTIPLIER).min(MAX_DELAY);
        }
        Ok(task_status)
    }

    if let Ok(final_task) = timeout(TIMEOUT, await_task_inner(api_key, path)).await {
        final_task
    } else {
        Err(Error::new("The provided script took too long to finish!"))
    }
}

// Right now, Roblox does not have pagination support in their logs endpoint
// despite it being seemingly supported. That may change, and when it does
// we should respond accordingly. For now though... We don't care.
pub async fn get_logs_flat(api_key: &str, path: &str) -> Result<Vec<String>> {
    let url = format!("{BASE_URL}/{}/logs?view=FLAT", path);
    let result = CLIENT
        .request(Method::GET, &url)
        .header("x-api-key", api_header(api_key)?)
        .send()
        .await?;

    if result.status().is_success() {
        println!("[SUCCESS] {} {}", result.status(), &url);
        let mut json: OpenCloudLogs = result.json().await?;
        Ok(json
            .task_logs
            .first_mut()
            .map(|log| take(&mut log.messages))
            .unwrap_or_default())
    } else {
        println!("[ERROR  ] {} {}", result.status(), &url);
        let error: OpenCloudError = result.json().await?;
        Err(Error::new(format!(
            "Getting OpenCloud task logs failed:\n{}",
            error.message
        )))
    }
}

// See get_logs_flat for note on pagination
pub async fn get_logs_structured(api_key: &str, path: &str) -> Result<Vec<StructuredMessage>> {
    let url = format!("{BASE_URL}/{}/logs?view=STRUCTURED", path);
    let result = CLIENT
        .request(Method::GET, &url)
        .header("x-api-key", api_header(api_key)?)
        .send()
        .await?;

    if result.status().is_success() {
        println!("[SUCCESS] {} {}", result.status(), &url);
        let mut json: OpenCloudLogs = result.json().await?;
        Ok(json
            .task_logs
            .first_mut()
            .and_then(|s| take(&mut s.structured_messages))
            .unwrap_or_default())
    } else {
        println!("[ERROR  ] {} {}", result.status(), &url);
        let error: OpenCloudError = result.json().await?;
        Err(Error::new(format!(
            "Getting OpenCloud task structured logs failed:\n{}",
            error.message
        )))
    }
}
