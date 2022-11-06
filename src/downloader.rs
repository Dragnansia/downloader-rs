/// Trait to implement a custom Downloader
///
/// # Example
/// ```
/// use downloader_rs::downloader;
/// use std::cmp::min;
///
/// struct Downloader {
///     total_size: u64,
///     total_download: u64
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
/// ```
pub trait Downloader {
    fn init(&mut self, total_size: u64);

    fn update(&mut self, chunk: &[u8]);
}
