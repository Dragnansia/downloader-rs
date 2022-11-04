pub trait Downloader {
    fn init(&mut self, total_size: u64);

    fn update(&mut self, chunk: &[u8]);
}
