use sdkwork_routes_image_open_api::{open_api_routes, open_route_manifest};
use sdkwork_web_core::RouteAuth;

#[test]
fn image_open_route_manifest_matches_metadata_routes() {
    let manifest = open_route_manifest();
    let routes = open_api_routes();
    assert_eq!(routes.len(), 1);
    for entry in &routes {
        let method = match entry.method {
            sdkwork_routes_image_open_api::HttpMethod::Get => "GET",
            sdkwork_routes_image_open_api::HttpMethod::Post => "POST",
            sdkwork_routes_image_open_api::HttpMethod::Patch => "PATCH",
            sdkwork_routes_image_open_api::HttpMethod::Delete => "DELETE",
            sdkwork_routes_image_open_api::HttpMethod::Put => "PUT",
        };
        let matched = manifest
            .match_route(method, entry.path)
            .unwrap_or_else(|| panic!("missing http route manifest for {method} {}", entry.path));
        assert_eq!(matched.auth, RouteAuth::ApiKey);
        assert_eq!(matched.operation_id, entry.operation_id);
    }
}
