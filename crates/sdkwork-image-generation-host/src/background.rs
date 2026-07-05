use std::sync::Arc;
use std::time::Duration;

use reqwest::Url;
use sdkwork_image_generation_repository_sqlx::{
    ImageGenerationBackgroundRepository, PendingNotificationRow,
};

use crate::service::ImageGenerationService;
use crate::subject::RuntimeSubject;
use crate::wire::ImageGenerationRefreshCommandWire;

const DEFAULT_POLL_INTERVAL_SECONDS: i64 = 30;
const DEFAULT_NOTIFICATION_RETRY_SECONDS: i64 = 60;
const DEFAULT_BATCH_LIMIT: i64 = 16;

pub fn image_background_processor_enabled_from_env() -> bool {
    match std::env::var("IMAGE_BACKGROUND_PROCESSOR_ENABLED")
        .ok()
        .map(|value| value.trim().to_ascii_lowercase())
    {
        Some(value) if matches!(value.as_str(), "0" | "false" | "no" | "off") => false,
        _ => true,
    }
}

pub fn spawn_image_generation_background_processor(
    service: Arc<ImageGenerationService>,
    background: Arc<dyn ImageGenerationBackgroundRepository>,
) -> tokio::task::JoinHandle<()> {
    tokio::spawn(async move {
        let poll_interval = background_poll_interval_from_env();
        let mut ticker = tokio::time::interval(poll_interval);
        ticker.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);
        loop {
            ticker.tick().await;
            if let Err(error) = run_provider_poll_batch(&service, background.as_ref()).await {
                eprintln!("image provider poll batch failed: {error}");
            }
            if let Err(error) = run_notification_delivery_batch(background.as_ref()).await {
                eprintln!("image notification delivery batch failed: {error}");
            }
        }
    })
}

fn background_poll_interval_from_env() -> Duration {
    let seconds = std::env::var("IMAGE_BACKGROUND_POLL_INTERVAL_SECONDS")
        .ok()
        .and_then(|value| value.trim().parse::<u64>().ok())
        .unwrap_or(DEFAULT_POLL_INTERVAL_SECONDS as u64)
        .clamp(5, 3600);
    Duration::from_secs(seconds)
}

async fn run_provider_poll_batch(
    service: &ImageGenerationService,
    background: &dyn ImageGenerationBackgroundRepository,
) -> Result<(), String> {
    let tasks = background
        .find_due_provider_tasks(DEFAULT_BATCH_LIMIT)
        .await
        .map_err(|error| error.to_string())?;
    for task in tasks {
        let subject = RuntimeSubject {
            tenant_id: task.tenant_id.to_string(),
            organization_id: if task.organization_id == 0 {
                None
            } else {
                Some(task.organization_id.to_string())
            },
            user_id: if task.user_id > 0 {
                task.user_id.to_string()
            } else {
                "0".to_string()
            },
        };
        let refresh_result = service
            .refresh_generation(
                &subject,
                &task.generation_uuid,
                ImageGenerationRefreshCommandWire {
                    provider_task_id: Some(task.provider_task_id),
                },
            )
            .await;
        let poll_interval = std::env::var("IMAGE_PROVIDER_POLL_INTERVAL_SECONDS")
            .ok()
            .and_then(|value| value.trim().parse::<i64>().ok())
            .unwrap_or(DEFAULT_POLL_INTERVAL_SECONDS)
            .clamp(5, 3600);
        if let Err(error) = background
            .schedule_provider_task_poll(&task.task_uuid, poll_interval)
            .await
        {
            eprintln!(
                "failed to schedule provider task poll for {}: {error}",
                task.task_uuid
            );
        }
        if let Err(error) = refresh_result {
            eprintln!(
                "background refresh failed for generation {}: {error}",
                task.generation_uuid
            );
        }
    }
    Ok(())
}

async fn run_notification_delivery_batch(
    background: &dyn ImageGenerationBackgroundRepository,
) -> Result<(), String> {
    let notifications = background
        .find_pending_notifications(DEFAULT_BATCH_LIMIT)
        .await
        .map_err(|error| error.to_string())?;
    for notification in notifications {
        deliver_notification(background, notification).await;
    }
    Ok(())
}

async fn deliver_notification(
    background: &dyn ImageGenerationBackgroundRepository,
    notification: PendingNotificationRow,
) {
    let callback_url = notification
        .metadata
        .get("callbackUrl")
        .and_then(|value| value.as_str())
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(str::to_owned);
    let Some(callback_url) = callback_url else {
        let _ = background
            .mark_notification_delivered(&notification.outbox_uuid)
            .await;
        return;
    };
    let Ok(parsed_url) = Url::parse(&callback_url) else {
        let _ = background
            .mark_notification_failed(
                &notification.outbox_uuid,
                "callback url is invalid",
                DEFAULT_NOTIFICATION_RETRY_SECONDS,
            )
            .await;
        return;
    };
    if parsed_url.scheme() != "https" && parsed_url.scheme() != "http" {
        let _ = background
            .mark_notification_failed(
                &notification.outbox_uuid,
                "callback url must use http or https",
                DEFAULT_NOTIFICATION_RETRY_SECONDS,
            )
            .await;
        return;
    }
    let client = match reqwest::Client::builder()
        .timeout(Duration::from_secs(15))
        .build()
    {
        Ok(client) => client,
        Err(error) => {
            let _ = background
                .mark_notification_failed(
                    &notification.outbox_uuid,
                    &format!("http client init failed: {error}"),
                    DEFAULT_NOTIFICATION_RETRY_SECONDS,
                )
                .await;
            return;
        }
    };
    let response = client
        .post(parsed_url)
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .json(&notification.payload_snapshot)
        .send()
        .await;
    match response {
        Ok(response) if response.status().is_success() => {
            let _ = background
                .mark_notification_delivered(&notification.outbox_uuid)
                .await;
        }
        Ok(response) => {
            let _ = background
                .mark_notification_failed(
                    &notification.outbox_uuid,
                    &format!("callback returned HTTP {}", response.status()),
                    DEFAULT_NOTIFICATION_RETRY_SECONDS,
                )
                .await;
        }
        Err(error) => {
            let _ = background
                .mark_notification_failed(
                    &notification.outbox_uuid,
                    &format!("callback delivery failed: {error}"),
                    DEFAULT_NOTIFICATION_RETRY_SECONDS,
                )
                .await;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn background_processor_enabled_by_default() {
        std::env::remove_var("IMAGE_BACKGROUND_PROCESSOR_ENABLED");
        assert!(image_background_processor_enabled_from_env());
    }
}
