use std::sync::Arc;

use rmcp::{
    handler::server::{router::tool::ToolRouter, wrapper::Parameters},
    model::{
        CallToolResult, ErrorData, GetPromptRequestParams, GetPromptResult, Implementation,
        ListPromptsResult, ListResourcesResult, PaginatedRequestParams, ReadResourceRequestParams,
        ReadResourceResult, ServerCapabilities, ServerInfo, Tool,
    },
    service::RequestContext,
    tool, tool_handler, tool_router, Json, RoleServer, ServerHandler,
};
use sdkwork_image_generation_service::{
    ImageGenerationProviderDescriptor, ImageGenerationServicePort,
};

use crate::{
    dto::GenerateImageInput, ImageGenerationMcpTaskContext, ImageGenerationMcpTaskStore,
    ImageGenerationResult, ImageTaskInput, InMemoryImageGenerationMcpTaskStore, McpToolError,
};

#[derive(Clone)]
pub struct ImageGenerationMcpService {
    generation_service: Arc<dyn ImageGenerationServicePort>,
    task_store: Arc<dyn ImageGenerationMcpTaskStore>,
    tool_router: ToolRouter<Self>,
}

impl ImageGenerationMcpService {
    pub fn new(generation_service: Arc<dyn ImageGenerationServicePort>) -> Self {
        Self::with_task_store(
            generation_service,
            InMemoryImageGenerationMcpTaskStore::shared_default(),
        )
    }

    pub fn with_task_store(
        generation_service: Arc<dyn ImageGenerationServicePort>,
        task_store: Arc<dyn ImageGenerationMcpTaskStore>,
    ) -> Self {
        Self {
            generation_service,
            task_store,
            tool_router: Self::tool_router(),
        }
    }

    pub fn tools(&self) -> Vec<Tool> {
        self.tool_router.list_all()
    }

    pub fn provider_descriptors(&self) -> Vec<ImageGenerationProviderDescriptor> {
        self.generation_service.provider_descriptors()
    }
}

#[tool_router]
impl ImageGenerationMcpService {
    #[tool(
        name = "image.generate",
        description = "Generate images through the unified image generation service."
    )]
    async fn generate(
        &self,
        Parameters(input): Parameters<GenerateImageInput>,
    ) -> Result<Json<ImageGenerationResult>, Json<McpToolError>> {
        let command = input.try_into().map_err(Json)?;
        let submission = self
            .generation_service
            .generate(command)
            .await
            .map_err(|error| Json(error.into()))?;
        let task_handle = match submission.result.provider_task_id.as_deref() {
            Some(provider_task_id) => Some(
                self.task_store
                    .save(ImageGenerationMcpTaskContext {
                        dispatch_plan: submission.dispatch_plan.clone(),
                        provider_task_id: provider_task_id.to_string(),
                    })
                    .map_err(Json)?,
            ),
            None => None,
        };
        Ok(Json(ImageGenerationResult::from_submission(
            &submission,
            task_handle,
        )))
    }

    #[tool(
        name = "image.retrieve",
        description = "Retrieve an image generation task by the task handle returned from image.generate."
    )]
    async fn retrieve(
        &self,
        Parameters(input): Parameters<ImageTaskInput>,
    ) -> Result<Json<ImageGenerationResult>, Json<McpToolError>> {
        let context = self.task_context(&input.task_handle).map_err(Json)?;
        let result = self
            .generation_service
            .retrieve(&context.dispatch_plan, &context.provider_task_id)
            .await
            .map_err(|error| Json(error.into()))?;
        Ok(Json(ImageGenerationResult::from_normalized(
            &result,
            Some(input.task_handle),
        )))
    }

    #[tool(
        name = "image.cancel",
        description = "Cancel an image generation task by the task handle returned from image.generate."
    )]
    async fn cancel(
        &self,
        Parameters(input): Parameters<ImageTaskInput>,
    ) -> Result<Json<ImageGenerationResult>, Json<McpToolError>> {
        let context = self.task_context(&input.task_handle).map_err(Json)?;
        let result = self
            .generation_service
            .cancel(&context.dispatch_plan, &context.provider_task_id)
            .await
            .map_err(|error| Json(error.into()))?;
        Ok(Json(ImageGenerationResult::from_normalized(
            &result,
            Some(input.task_handle),
        )))
    }

    #[tool(
        name = "image.capabilities",
        description = "List registered image generation vendors and provider capabilities."
    )]
    async fn capabilities(&self) -> CallToolResult {
        CallToolResult::structured(crate::catalog::catalog_value(self.provider_descriptors()))
    }
}

impl ImageGenerationMcpService {
    fn task_context(&self, handle: &str) -> Result<ImageGenerationMcpTaskContext, McpToolError> {
        let handle = handle.trim();
        if handle.is_empty() {
            return Err(McpToolError::invalid_request("taskHandle is required"));
        }
        self.task_store
            .load(handle)?
            .ok_or_else(|| McpToolError::task_not_found(handle))
    }
}

#[tool_handler(router = self.tool_router)]
impl ServerHandler for ImageGenerationMcpService {
    fn get_info(&self) -> ServerInfo {
        ServerInfo::new(
            ServerCapabilities::builder()
                .enable_tools()
                .enable_resources()
                .enable_prompts()
                .build(),
        )
        .with_server_info(Implementation::new(
            "sdkwork-image-generation-mcp-service",
            env!("CARGO_PKG_VERSION"),
        ))
        .with_instructions(
            "Use image generation tools through provider-neutral inputs. Read capability resources before selecting optional vendor-specific parameters.",
        )
    }

    async fn list_resources(
        &self,
        _request: Option<PaginatedRequestParams>,
        _context: RequestContext<RoleServer>,
    ) -> Result<ListResourcesResult, ErrorData> {
        Ok(crate::catalog::resource_list())
    }

    async fn read_resource(
        &self,
        request: ReadResourceRequestParams,
        _context: RequestContext<RoleServer>,
    ) -> Result<ReadResourceResult, ErrorData> {
        crate::catalog::read_resource(&request.uri, self.provider_descriptors())
            .map(|contents| ReadResourceResult::new(vec![contents]))
            .ok_or_else(|| ErrorData::resource_not_found("image MCP resource was not found", None))
    }

    async fn list_prompts(
        &self,
        _request: Option<PaginatedRequestParams>,
        _context: RequestContext<RoleServer>,
    ) -> Result<ListPromptsResult, ErrorData> {
        Ok(crate::catalog::prompts())
    }

    async fn get_prompt(
        &self,
        request: GetPromptRequestParams,
        _context: RequestContext<RoleServer>,
    ) -> Result<GetPromptResult, ErrorData> {
        if request.name == crate::catalog::GENERATION_PROMPT {
            Ok(crate::catalog::generation_prompt())
        } else {
            Err(ErrorData::invalid_params(
                "image MCP prompt was not found",
                None,
            ))
        }
    }
}
