use async_trait::async_trait;
use ippper::server::serve_ipp;
use ippper::service::simple::{
    PrinterInfoBuilder, SimpleIppDocument, SimpleIppService, SimpleIppServiceHandler,
};
use std::env::temp_dir;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::sync::Arc;
use tokio::fs::{remove_file, File};
use tokio::io;
use tokio_util::compat::*;
use uuid::Uuid;
use log::info;

mod print;

struct MyHandler {}
impl MyHandler {
    fn new() -> Self {
        Self {}
    }
}
#[async_trait]
impl SimpleIppServiceHandler for MyHandler {
    async fn handle_document(&self, document: SimpleIppDocument) -> anyhow::Result<()> {
        let current_time_secs = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let mime_type = document.format.clone().or(Some("application/octet-stream".to_string())).unwrap();
        let file_extension = mime_type.split("/").last().unwrap();

        let unique_file_name = format!("./{}.print.{}", current_time_secs, file_extension);
        let file_path = temp_dir().join(&unique_file_name);

        info!("Printing document: {:?} {:?}", document.format, file_path.display());

        let mut file = File::create(&file_path).await?;
        io::copy(&mut document.payload.compat(), &mut file).await?;
        file.sync_all().await?;
        if let Err(e) = print::print_file(&file_path) {
            eprintln!("Error: {}", e);
        }
        remove_file(&file_path).await?;

        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), 631);
    let mut ipp_service = SimpleIppService::new(MyHandler::new());
    ipp_service.set_info(
        PrinterInfoBuilder::default()
            .uuid(Some(
                // Change it if you are building a ipp service
                // Make it unique for each instance
                Uuid::parse_str("786a551c-65a3-43ce-89ba-33c51bae9bc2").unwrap(),
            ))
            .build()
            .unwrap(),
    );
    serve_ipp(addr, Arc::new(ipp_service)).await
}