use utoipa::{
    Modify, OpenApi,
    openapi::{
        Server,
        security::{HttpAuthScheme, HttpBuilder, SecurityScheme},
    },
};

#[derive(OpenApi)]
#[openapi(
    modifiers(&SecurityAddon, &ServerAddon),
)]
pub struct ApiDoc;

struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "session",
                SecurityScheme::Http(HttpBuilder::new().scheme(HttpAuthScheme::Bearer).build()),
            );
        }
    }
}

struct ServerAddon;

impl Modify for ServerAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        openapi.servers = Some(vec![Server::new("/"), Server::new("example.com")])
    }
}
