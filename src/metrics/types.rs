pub struct RocksDBMetrics {
    pub bytes_written: u64,
    pub bytes_read: u64,
}

impl RocksDBMetrics {
    pub fn new() -> Self {
        RocksDBMetrics {
            bytes_written: 0,
            bytes_read: 0,
        }
    }
}
