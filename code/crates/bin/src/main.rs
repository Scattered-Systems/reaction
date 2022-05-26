mod api;

#[tokio::main]
async fn main() {
    let settings = match crate::api::settings::Settings::new() {
        Ok(value) => value,
        Err(err) => panic!("ConfigurationError: {:#?}", err)
    };
    let app = crate::api::interface::Interface::new(settings).await;
    println!("{:#?}", app.context.settings)
}