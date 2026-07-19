use rmcp::model::{
    GetPromptResult, ListPromptsResult, ListResourcesResult, Prompt, PromptMessage, Resource,
    ResourceContents, Role,
};
use sdkwork_image_generation_service::{
    ImageGenerationProviderCapability, ImageGenerationProviderDescriptor,
};
use serde::Serialize;

pub(crate) const CAPABILITIES_URI: &str = "sdkwork://image/generation/capabilities";
pub(crate) const VENDORS_URI: &str = "sdkwork://image/generation/vendors";
pub(crate) const GENERATION_PROMPT: &str = "image.generation.request";

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct ProviderCatalog {
    domain: &'static str,
    tools: [&'static str; 4],
    transports: [&'static str; 2],
    providers: Vec<ProviderDescriptor>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct ProviderDescriptor {
    vendors: Vec<String>,
    capabilities: Vec<&'static str>,
}

pub(crate) fn resources() -> Vec<Resource> {
    vec![
        Resource::new(CAPABILITIES_URI, "image-generation-capabilities")
            .with_title("Image generation capabilities")
            .with_description("MCP tools and transports exposed by the image generation adapter.")
            .with_mime_type("application/json"),
        Resource::new(VENDORS_URI, "image-generation-vendors")
            .with_title("Image generation vendors")
            .with_description("Currently registered image generation vendors and capabilities.")
            .with_mime_type("application/json"),
    ]
}

pub(crate) fn read_resource(
    uri: &str,
    descriptors: Vec<ImageGenerationProviderDescriptor>,
) -> Option<ResourceContents> {
    let catalog = catalog_value(descriptors);
    let value = match uri {
        CAPABILITIES_URI => catalog,
        VENDORS_URI => catalog.get("providers")?.clone(),
        _ => return None,
    };
    Some(
        ResourceContents::text(serde_json::to_string_pretty(&value).ok()?, uri)
            .with_mime_type("application/json"),
    )
}

pub(crate) fn catalog_value(
    descriptors: Vec<ImageGenerationProviderDescriptor>,
) -> serde_json::Value {
    let providers = descriptors
        .into_iter()
        .map(|descriptor| ProviderDescriptor {
            vendors: descriptor
                .vendors
                .into_iter()
                .map(|vendor| vendor.to_string())
                .collect(),
            capabilities: descriptor
                .capabilities
                .into_iter()
                .map(capability_name)
                .collect(),
        })
        .collect::<Vec<_>>();
    serde_json::to_value(ProviderCatalog {
        domain: "image",
        tools: [
            "image.generate",
            "image.retrieve",
            "image.cancel",
            "image.capabilities",
        ],
        transports: ["stdio", "streamable-http-sse"],
        providers,
    })
    .expect("image MCP provider catalog is serializable")
}

pub(crate) fn prompts() -> ListPromptsResult {
    ListPromptsResult::with_all_items(vec![Prompt::new(
        GENERATION_PROMPT,
        Some("Prepare a provider-neutral image generation request for image.generate."),
        None,
    )])
}

pub(crate) fn generation_prompt() -> GetPromptResult {
    GetPromptResult::new(vec![PromptMessage::new_text(
        Role::User,
        "Create an image generation request. Select vendor and model explicitly when required, keep provider-only fields inside vendorParameters with its schema identifier, inspect sdkwork://image/generation/vendors when capabilities are uncertain, and invoke image.generate.",
    )])
    .with_description("Provider-neutral image generation request workflow")
}

pub(crate) fn resource_list() -> ListResourcesResult {
    ListResourcesResult::with_all_items(resources())
}

fn capability_name(capability: ImageGenerationProviderCapability) -> &'static str {
    match capability {
        ImageGenerationProviderCapability::TextToImage => "text-to-image",
        ImageGenerationProviderCapability::ReferenceToImage => "reference-to-image",
        ImageGenerationProviderCapability::MultipleOutputs => "multiple-outputs",
        ImageGenerationProviderCapability::NegativePrompt => "negative-prompt",
        ImageGenerationProviderCapability::Seed => "seed",
        ImageGenerationProviderCapability::Polling => "polling",
        ImageGenerationProviderCapability::Webhook => "webhook",
        ImageGenerationProviderCapability::Cancellation => "cancellation",
    }
}
