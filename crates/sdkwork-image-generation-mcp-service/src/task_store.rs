use std::{
    collections::{HashMap, VecDeque},
    sync::{
        atomic::{AtomicU64, Ordering},
        Arc, Mutex,
    },
};

use sdkwork_image_generation_service::ImageProviderDispatchPlan;

use crate::McpToolError;

#[derive(Clone, Debug)]
pub struct ImageGenerationMcpTaskContext {
    pub dispatch_plan: ImageProviderDispatchPlan,
    pub provider_task_id: String,
}

pub trait ImageGenerationMcpTaskStore: Send + Sync {
    fn save(&self, context: ImageGenerationMcpTaskContext) -> Result<String, McpToolError>;
    fn load(&self, handle: &str) -> Result<Option<ImageGenerationMcpTaskContext>, McpToolError>;
}

pub struct InMemoryImageGenerationMcpTaskStore {
    capacity: usize,
    sequence: AtomicU64,
    state: Mutex<TaskStoreState>,
}

#[derive(Default)]
struct TaskStoreState {
    order: VecDeque<String>,
    contexts: HashMap<String, ImageGenerationMcpTaskContext>,
}

impl InMemoryImageGenerationMcpTaskStore {
    pub const DEFAULT_CAPACITY: usize = 2_048;

    pub fn new(capacity: usize) -> Result<Self, McpToolError> {
        if capacity == 0 {
            return Err(McpToolError::invalid_request(
                "image MCP task store capacity must be greater than zero",
            ));
        }
        Ok(Self {
            capacity,
            sequence: AtomicU64::new(1),
            state: Mutex::new(TaskStoreState::default()),
        })
    }

    pub fn shared_default() -> Arc<dyn ImageGenerationMcpTaskStore> {
        Arc::new(
            Self::new(Self::DEFAULT_CAPACITY)
                .expect("the image MCP default task store capacity is valid"),
        )
    }
}

impl ImageGenerationMcpTaskStore for InMemoryImageGenerationMcpTaskStore {
    fn save(&self, context: ImageGenerationMcpTaskContext) -> Result<String, McpToolError> {
        let handle = format!(
            "image-task-{}",
            self.sequence.fetch_add(1, Ordering::Relaxed)
        );
        let mut state = self.state.lock().map_err(|_| McpToolError {
            code: "task_store_unavailable".to_string(),
            message: "image MCP task store lock is unavailable".to_string(),
            retryable: true,
        })?;
        while state.contexts.len() >= self.capacity {
            if let Some(expired) = state.order.pop_front() {
                state.contexts.remove(&expired);
            }
        }
        state.order.push_back(handle.clone());
        state.contexts.insert(handle.clone(), context);
        Ok(handle)
    }

    fn load(&self, handle: &str) -> Result<Option<ImageGenerationMcpTaskContext>, McpToolError> {
        let state = self.state.lock().map_err(|_| McpToolError {
            code: "task_store_unavailable".to_string(),
            message: "image MCP task store lock is unavailable".to_string(),
            retryable: true,
        })?;
        Ok(state.contexts.get(handle.trim()).cloned())
    }
}
