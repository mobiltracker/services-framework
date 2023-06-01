pub trait ServiceDefinition {
    fn service_id(&self) -> &str;
    fn service_deps(&self) -> &[&str];
    fn service_resources(&self) -> &[&str];
}

struct HelloService {}

impl ServiceDefinition for HelloService {
    fn service_id(&self) -> &str {
        "hello-service"
    }

    fn service_deps(&self) -> &[&str] {
        &[]
    }

    fn service_resources(&self) -> &[&str] {
        &[]
    }
}
