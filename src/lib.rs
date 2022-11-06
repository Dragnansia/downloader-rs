//! Librairy to download file or bytes

pub mod downloader;
pub mod error;

use downloader::Downloader;
use futures_util::StreamExt;
use reqwest::{header::USER_AGENT, Client, Response};
use std::{fs::File, io::Write};

pub type Result<T> = std::result::Result<T, error::Error>;

/// Create client, get response and total data size
async fn create_client(url: &str) -> Result<(Client, Response, u64)> {
    let client = Client::new();
    let response = client
        .get(url)
        .header(USER_AGENT, "Downloader")
        .send()
        .await?;
    let total_size = response
        .content_length()
        .ok_or(error::Error::GetTotalSize)?;

    Ok((client, response, total_size))
}

/// Download file on given path
///
/// # Example
/// ```
/// use std::cmp::min;
/// use downloader_rs::{
///     downloader,
///     error::Error,
///     download_file
/// };
///
/// #[derive(Default)]
/// struct Downloader {
///     total_size: u64,
///     total_download: u64,
/// }
///
/// impl downloader::Downloader for Downloader {
///     fn init(&mut self, total_size: u64) {
///         self.total_size = total_size;
///     }
///
///     fn update(&mut self, chunk: &[u8]) {
///         self.total_download = min(self.total_download + (chunk.len() as u64), self.total_size);
///         println!("Total Download: {}", self.total_download);
///     }
/// }
///
/// async fn func() -> Result<(), Error> {
///     let url = "https://my-api.rs/big/file";
///     download_file(url, "destination/path", &mut Downloader::default()).await?;
///     Ok(())
/// }
/// ```
pub async fn download_file<D>(url: &str, path: &str, downloader: &mut D) -> Result<()>
where
    D: Downloader,
{
    let (_, response, total_size) = create_client(url).await?;
    let mut file = File::create(&path)?;
    let mut stream = response.bytes_stream();

    downloader.init(total_size);
    while let Some(item) = stream.next().await {
        let chunk = item?;
        file.write_all(&chunk)?;
        downloader.update(&chunk);
    }

    Ok(())
}

/// Download buffer
///
/// # Example
/// ```
/// use downloader_rs::{
///     downloader,
///     error::Error,
///     download_buffer
/// };
///
/// #[derive(Default)]
/// struct Downloader {}
///
/// impl downloader::Downloader for Downloader {
///     fn init(&mut self, total_size: u64) {}
///
///     fn update(&mut self, chunk: &[u8]) {
///         println!("New buffer part dl {}", chunk.len());
///     }
/// }
///
/// async fn func() -> Result<(), Error> {
///     let url = "https://my-api.rs/big/data";
///     download_buffer(url, &mut Downloader::default()).await?;
///     Ok(())
/// }
/// ```
pub async fn download_buffer<D>(url: &str, downloader: &mut D) -> Result<()>
where
    D: Downloader,
{
    let (_, response, total_size) = create_client(url).await?;
    let mut stream = response.bytes_stream();

    downloader.init(total_size);
    while let Some(item) = stream.next().await {
        downloader.update(&item?);
    }

    Ok(())
}
