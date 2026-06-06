use sdkwork_image_core::{
    create_image_generation_request, validate_image_generation_request, ImageGenerationRequest,
    ImageJobStatus, ImageVisibility, IMAGE_CAPABILITY, IMAGE_DOMAIN, IMAGE_WORKSPACE,
};

#[test]
fn exposes_image_domain_identity() {
    assert_eq!(IMAGE_WORKSPACE, "sdkwork-image");
    assert_eq!(IMAGE_DOMAIN, "image");
    assert_eq!(IMAGE_CAPABILITY, "image");
}

#[test]
fn validates_image_generation_request_boundaries() {
    let request = create_image_generation_request(
        "tenant-1",
        Some("org-1"),
        "Premium device beauty shot",
        "1024x1024",
        "studio",
        ImageVisibility::Tenant,
    );

    assert_eq!(request.tenant_id, "tenant-1");
    assert_eq!(request.organization_id.as_deref(), Some("org-1"));
    assert_eq!(request.status, ImageJobStatus::Queued);
    assert!(validate_image_generation_request(&request).is_ok());

    let missing_prompt = ImageGenerationRequest {
        prompt: " ".to_string(),
        ..request.clone()
    };
    assert_eq!(
        validate_image_generation_request(&missing_prompt),
        Err("image generation prompt is required"),
    );

    let invalid_resolution = ImageGenerationRequest {
        resolution: "1024".to_string(),
        ..request
    };
    assert_eq!(
        validate_image_generation_request(&invalid_resolution),
        Err("image generation resolution must use WIDTHxHEIGHT"),
    );
}

#[test]
fn image_status_and_visibility_are_stable_integer_contracts() {
    assert_eq!(ImageJobStatus::Queued as i32, 1);
    assert_eq!(ImageJobStatus::Rendering as i32, 2);
    assert_eq!(ImageJobStatus::Ready as i32, 3);
    assert_eq!(ImageJobStatus::Failed as i32, 4);

    assert_eq!(ImageVisibility::Private as i32, 1);
    assert_eq!(ImageVisibility::Tenant as i32, 2);
    assert_eq!(ImageVisibility::Public as i32, 3);
}
