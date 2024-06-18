pub mod middleware;
pub mod rocksdb;
pub mod types;

use std::env;

use lazy_static::lazy_static;

use prometheus::{self, GaugeVec, HistogramVec, IntCounterVec};
use prometheus::{opts, register_gauge_vec, register_histogram_vec, register_int_counter_vec};

use prometheus::DEFAULT_BUCKETS;

use std::collections::HashMap;

lazy_static! {
    pub static ref INSTANCE_ID: String = env::var("INSTANCE_ID").unwrap_or("".to_owned());
    pub static ref INDEX_ID: String = env::var("INDEX_ID").unwrap_or("".to_owned());
    pub static ref ORGANIZATION_ID: String = env::var("ORGANIZATION_ID").unwrap_or("".to_owned());
}

// Internal metrics
lazy_static! {
    pub static ref N_REQUESTS: IntCounterVec = register_int_counter_vec!(
        opts!("number_requests", "Number of HTTP requests"),
        &["method", "route", "status", "instance_id", "index_id", "organization_id"]
    )
    .unwrap();
    pub static ref REQUEST_TIME_HISTOGRAM: HistogramVec = register_histogram_vec!(
        "request_time_histogram",
        "Request time histogram : description",
        &["method", "route", "status", "instance_id", "index_id", "organization_id"],
        DEFAULT_BUCKETS.to_vec()
    )
    .unwrap();
}

// ROCKSDB metrics
lazy_static! {
    pub static ref ROCKSDB_COUNTERS: HashMap<String, IntCounterVec> = HashMap::from([
        (
            "rocksdb_blobdb_bytes_read".to_owned(),
            register_int_counter_vec!(
                opts!("rocksdb_blobdb_bytes_read", "rocksdb_blobdb_bytes_read: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_memtable_compaction_count".to_owned(),
            register_int_counter_vec!(
                opts!("rocksdb_memtable_compaction_count", "rocksdb_memtable_compaction_count: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_txn_overhead_mutex_prepare".to_owned(),
            register_int_counter_vec!(
                opts!("rocksdb_txn_overhead_mutex_prepare", "rocksdb_txn_overhead_mutex_prepare: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_l1_hit".to_owned(),
            register_int_counter_vec!(
                opts!("rocksdb_l1_hit", "rocksdb_l1_hit: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_compaction_key_drop_user".to_owned(),
            register_int_counter_vec!(
                opts!("rocksdb_compaction_key_drop_user", "rocksdb_compaction_key_drop_user: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_blobdb_gc_num_files".to_owned(),
            register_int_counter_vec!(
                opts!("rocksdb_blobdb_gc_num_files", "rocksdb_blobdb_gc_num_files: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_error_handler_bg_retryable_io_errro_count".to_owned(),
            register_int_counter_vec!(
                opts!(
                    "rocksdb_error_handler_bg_retryable_io_errro_count",
                    "rocksdb_error_handler_bg_retryable_io_errro_count: description."
                ),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_wal_file_sync_micros".to_owned(),
            register_int_counter_vec!(
                opts!("rocksdb_wal_file_sync_micros", "rocksdb_wal_file_sync_micros: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_sim_block_cache_miss".to_owned(),
            register_int_counter_vec!(
                opts!("rocksdb_sim_block_cache_miss", "rocksdb_sim_block_cache_miss: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_db_mutex_wait_micros".to_owned(),
            register_int_counter_vec!(
                opts!("rocksdb_db_mutex_wait_micros", "rocksdb_db_mutex_wait_micros: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_l2andup_hit".to_owned(),
            register_int_counter_vec!(
                opts!("rocksdb_l2andup_hit", "rocksdb_l2andup_hit: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_read_block_compaction_micros".to_owned(),
            register_int_counter_vec!(
                opts!(
                    "rocksdb_read_block_compaction_micros",
                    "rocksdb_read_block_compaction_micros: description."
                ),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_num_index_and_filter_blocks_read_per_level".to_owned(),
            register_int_counter_vec!(
                opts!(
                    "rocksdb_num_index_and_filter_blocks_read_per_level",
                    "rocksdb_num_index_and_filter_blocks_read_per_level: description."
                ),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_number_db_seek_found".to_owned(),
            register_int_counter_vec!(
                opts!("rocksdb_number_db_seek_found", "rocksdb_number_db_seek_found: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_number_db_next".to_owned(),
            register_int_counter_vec!(
                opts!("rocksdb_number_db_next", "rocksdb_number_db_next: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_number_merge_failures".to_owned(),
            register_int_counter_vec!(
                opts!("rocksdb_number_merge_failures", "rocksdb_number_merge_failures: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_blobdb_next_micros".to_owned(),
            register_int_counter_vec!(
                opts!("rocksdb_blobdb_next_micros", "rocksdb_blobdb_next_micros: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_compact_read_periodic_bytes".to_owned(),
            register_int_counter_vec!(
                opts!("rocksdb_compact_read_periodic_bytes", "rocksdb_compact_read_periodic_bytes: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_block_cache_add".to_owned(),
            register_int_counter_vec!(
                opts!("rocksdb_block_cache_add", "rocksdb_block_cache_add: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_number_db_seek".to_owned(),
            register_int_counter_vec!(
                opts!("rocksdb_number_db_seek", "rocksdb_number_db_seek: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_block_cache_data_miss".to_owned(),
            register_int_counter_vec!(
                opts!("rocksdb_block_cache_data_miss", "rocksdb_block_cache_data_miss: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_block_cache_data_add".to_owned(),
            register_int_counter_vec!(
                opts!("rocksdb_block_cache_data_add", "rocksdb_block_cache_data_add: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_bloom_filter_full_true_positive".to_owned(),
            register_int_counter_vec!(
                opts!(
                    "rocksdb_bloom_filter_full_true_positive",
                    "rocksdb_bloom_filter_full_true_positive: description."
                ),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_blobdb_gc_bytes_relocated".to_owned(),
            register_int_counter_vec!(
                opts!("rocksdb_blobdb_gc_bytes_relocated", "rocksdb_blobdb_gc_bytes_relocated: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_blobdb_num_write".to_owned(),
            register_int_counter_vec!(
                opts!("rocksdb_blobdb_num_write", "rocksdb_blobdb_num_write: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_merge_operation_time_nanos".to_owned(),
            register_int_counter_vec!(
                opts!("rocksdb_merge_operation_time_nanos", "rocksdb_merge_operation_time_nanos: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_num_iterator_deleted".to_owned(),
            register_int_counter_vec!(
                opts!("rocksdb_num_iterator_deleted", "rocksdb_num_iterator_deleted: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_db_get_micros".to_owned(),
            register_int_counter_vec!(
                opts!("rocksdb_db_get_micros", "rocksdb_db_get_micros: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_blobdb_multiget_micros".to_owned(),
            register_int_counter_vec!(
                opts!("rocksdb_blobdb_multiget_micros", "rocksdb_blobdb_multiget_micros: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_getupdatessince_calls".to_owned(),
            register_int_counter_vec!(
                opts!("rocksdb_getupdatessince_calls", "rocksdb_getupdatessince_calls: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_block_cache_index_miss".to_owned(),
            register_int_counter_vec!(
                opts!("rocksdb_block_cache_index_miss", "rocksdb_block_cache_index_miss: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_blobdb_blob_index_evicted_size".to_owned(),
            register_int_counter_vec!(
                opts!(
                    "rocksdb_blobdb_blob_index_evicted_size",
                    "rocksdb_blobdb_blob_index_evicted_size: description."
                ),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_blobdb_fifo_bytes_evicted".to_owned(),
            register_int_counter_vec!(
                opts!("rocksdb_blobdb_fifo_bytes_evicted", "rocksdb_blobdb_fifo_bytes_evicted: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_block_cachecompressed_add_failures".to_owned(),
            register_int_counter_vec!(
                opts!(
                    "rocksdb_block_cachecompressed_add_failures",
                    "rocksdb_block_cachecompressed_add_failures: description."
                ),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_txn_overhead_mutex_old_commit_map".to_owned(),
            register_int_counter_vec!(
                opts!(
                    "rocksdb_txn_overhead_mutex_old_commit_map",
                    "rocksdb_txn_overhead_mutex_old_commit_map: description."
                ),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_files_marked_trash".to_owned(),
            register_int_counter_vec!(
                opts!("rocksdb_files_marked_trash", "rocksdb_files_marked_trash: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_block_cachecompressed_hit".to_owned(),
            register_int_counter_vec!(
                opts!("rocksdb_block_cachecompressed_hit", "rocksdb_block_cachecompressed_hit: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_number_multiget_keys_found".to_owned(),
            register_int_counter_vec!(
                opts!("rocksdb_number_multiget_keys_found", "rocksdb_number_multiget_keys_found: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_block_cache_data_add_redundant".to_owned(),
            register_int_counter_vec!(
                opts!(
                    "rocksdb_block_cache_data_add_redundant",
                    "rocksdb_block_cache_data_add_redundant: description."
                ),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_manifest_file_sync_micros".to_owned(),
            register_int_counter_vec!(
                opts!("rocksdb_manifest_file_sync_micros", "rocksdb_manifest_file_sync_micros: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_soft_rate_limit_delay_count".to_owned(),
            register_int_counter_vec!(
                opts!("rocksdb_soft_rate_limit_delay_count", "rocksdb_soft_rate_limit_delay_count: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_blobdb_blob_index_evicted_count".to_owned(),
            register_int_counter_vec!(
                opts!(
                    "rocksdb_blobdb_blob_index_evicted_count",
                    "rocksdb_blobdb_blob_index_evicted_count: description."
                ),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_memtable_miss".to_owned(),
            register_int_counter_vec!(
                opts!("rocksdb_memtable_miss", "rocksdb_memtable_miss: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_write_raw_block_micros".to_owned(),
            register_int_counter_vec!(
                opts!("rocksdb_write_raw_block_micros", "rocksdb_write_raw_block_micros: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_blobdb_blob_file_sync_micros".to_owned(),
            register_int_counter_vec!(
                opts!(
                    "rocksdb_blobdb_blob_file_sync_micros",
                    "rocksdb_blobdb_blob_file_sync_micros: description."
                ),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_blobdb_decompression_micros".to_owned(),
            register_int_counter_vec!(
                opts!("rocksdb_blobdb_decompression_micros", "rocksdb_blobdb_decompression_micros: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_blobdb_blob_index_expired_count".to_owned(),
            register_int_counter_vec!(
                opts!(
                    "rocksdb_blobdb_blob_index_expired_count",
                    "rocksdb_blobdb_blob_index_expired_count: description."
                ),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_no_file_closes".to_owned(),
            register_int_counter_vec!(
                opts!("rocksdb_no_file_closes", "rocksdb_no_file_closes: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_blobdb_gc_failures".to_owned(),
            register_int_counter_vec!(
                opts!("rocksdb_blobdb_gc_failures", "rocksdb_blobdb_gc_failures: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_block_cache_miss".to_owned(),
            register_int_counter_vec!(
                opts!("rocksdb_block_cache_miss", "rocksdb_block_cache_miss: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_files_deleted_immediately".to_owned(),
            register_int_counter_vec!(
                opts!("rocksdb_files_deleted_immediately", "rocksdb_files_deleted_immediately: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_read_block_get_micros".to_owned(),
            register_int_counter_vec!(
                opts!("rocksdb_read_block_get_micros", "rocksdb_read_block_get_micros: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_blobdb_fifo_num_keys_evicted".to_owned(),
            register_int_counter_vec!(
                opts!(
                    "rocksdb_blobdb_fifo_num_keys_evicted",
                    "rocksdb_blobdb_fifo_num_keys_evicted: description."
                ),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_compaction_cancelled".to_owned(),
            register_int_counter_vec!(
                opts!("rocksdb_compaction_cancelled", "rocksdb_compaction_cancelled: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_num_iterators".to_owned(),
            register_int_counter_vec!(
                opts!("rocksdb_num_iterators", "rocksdb_num_iterators: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_blobdb_bytes_written".to_owned(),
            register_int_counter_vec!(
                opts!("rocksdb_blobdb_bytes_written", "rocksdb_blobdb_bytes_written: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_row_cache_hit".to_owned(),
            register_int_counter_vec!(
                opts!("rocksdb_row_cache_hit", "rocksdb_row_cache_hit: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_block_cache_hit".to_owned(),
            register_int_counter_vec!(
                opts!("rocksdb_block_cache_hit", "rocksdb_block_cache_hit: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_subcompaction_setup_times_micros".to_owned(),
            register_int_counter_vec!(
                opts!(
                    "rocksdb_subcompaction_setup_times_micros",
                    "rocksdb_subcompaction_setup_times_micros: description."
                ),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_compact_write_ttl_bytes".to_owned(),
            register_int_counter_vec!(
                opts!("rocksdb_compact_write_ttl_bytes", "rocksdb_compact_write_ttl_bytes: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_block_cache_index_bytes_evict".to_owned(),
            register_int_counter_vec!(
                opts!(
                    "rocksdb_block_cache_index_bytes_evict",
                    "rocksdb_block_cache_index_bytes_evict: description."
                ),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_number_block_not_compressed".to_owned(),
            register_int_counter_vec!(
                opts!("rocksdb_number_block_not_compressed", "rocksdb_number_block_not_compressed: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_write_wal".to_owned(),
            register_int_counter_vec!(
                opts!("rocksdb_write_wal", "rocksdb_write_wal: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_txn_get_tryagain".to_owned(),
            register_int_counter_vec!(
                opts!("rocksdb_txn_get_tryagain", "rocksdb_txn_get_tryagain: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_blobdb_num_put".to_owned(),
            register_int_counter_vec!(
                opts!("rocksdb_blobdb_num_put", "rocksdb_blobdb_num_put: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_sim_block_cache_hit".to_owned(),
            register_int_counter_vec!(
                opts!("rocksdb_sim_block_cache_hit", "rocksdb_sim_block_cache_hit: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_persistent_cache_miss".to_owned(),
            register_int_counter_vec!(
                opts!("rocksdb_persistent_cache_miss", "rocksdb_persistent_cache_miss: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_number_multiget_get".to_owned(),
            register_int_counter_vec!(
                opts!("rocksdb_number_multiget_get", "rocksdb_number_multiget_get: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_compact_read_ttl_bytes".to_owned(),
            register_int_counter_vec!(
                opts!("rocksdb_compact_read_ttl_bytes", "rocksdb_compact_read_ttl_bytes: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_number_superversion_cleanups".to_owned(),
            register_int_counter_vec!(
                opts!(
                    "rocksdb_number_superversion_cleanups",
                    "rocksdb_number_superversion_cleanups: description."
                ),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_blobdb_fifo_num_files_evicted".to_owned(),
            register_int_counter_vec!(
                opts!(
                    "rocksdb_blobdb_fifo_num_files_evicted",
                    "rocksdb_blobdb_fifo_num_files_evicted: description."
                ),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_block_cache_filter_add_redundant".to_owned(),
            register_int_counter_vec!(
                opts!(
                    "rocksdb_block_cache_filter_add_redundant",
                    "rocksdb_block_cache_filter_add_redundant: description."
                ),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_num_subcompactions_scheduled".to_owned(),
            register_int_counter_vec!(
                opts!(
                    "rocksdb_num_subcompactions_scheduled",
                    "rocksdb_num_subcompactions_scheduled: description."
                ),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_bytes_compressed".to_owned(),
            register_int_counter_vec!(
                opts!("rocksdb_bytes_compressed", "rocksdb_bytes_compressed: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_blobdb_key_size".to_owned(),
            register_int_counter_vec!(
                opts!("rocksdb_blobdb_key_size", "rocksdb_blobdb_key_size: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_blobdb_prev_micros".to_owned(),
            register_int_counter_vec!(
                opts!("rocksdb_blobdb_prev_micros", "rocksdb_blobdb_prev_micros: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_compact_write_periodic_bytes".to_owned(),
            register_int_counter_vec!(
                opts!(
                    "rocksdb_compact_write_periodic_bytes",
                    "rocksdb_compact_write_periodic_bytes: description."
                ),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_block_cachecompressed_add".to_owned(),
            register_int_counter_vec!(
                opts!("rocksdb_block_cachecompressed_add", "rocksdb_block_cachecompressed_add: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_blobdb_write_inlined_ttl".to_owned(),
            register_int_counter_vec!(
                opts!("rocksdb_blobdb_write_inlined_ttl", "rocksdb_blobdb_write_inlined_ttl: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_bloom_filter_full_positive".to_owned(),
            register_int_counter_vec!(
                opts!("rocksdb_bloom_filter_full_positive", "rocksdb_bloom_filter_full_positive: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_number_multiget_bytes_read".to_owned(),
            register_int_counter_vec!(
                opts!("rocksdb_number_multiget_bytes_read", "rocksdb_number_multiget_bytes_read: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_number_superversion_acquires".to_owned(),
            register_int_counter_vec!(
                opts!(
                    "rocksdb_number_superversion_acquires",
                    "rocksdb_number_superversion_acquires: description."
                ),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_persistent_cache_hit".to_owned(),
            register_int_counter_vec!(
                opts!("rocksdb_persistent_cache_hit", "rocksdb_persistent_cache_hit: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_block_cache_index_hit".to_owned(),
            register_int_counter_vec!(
                opts!("rocksdb_block_cache_index_hit", "rocksdb_block_cache_index_hit: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_compaction_key_drop_obsolete".to_owned(),
            register_int_counter_vec!(
                opts!(
                    "rocksdb_compaction_key_drop_obsolete",
                    "rocksdb_compaction_key_drop_obsolete: description."
                ),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_bloom_filter_prefix_useful".to_owned(),
            register_int_counter_vec!(
                opts!("rocksdb_bloom_filter_prefix_useful", "rocksdb_bloom_filter_prefix_useful: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_blobdb_gc_num_keys_relocated".to_owned(),
            register_int_counter_vec!(
                opts!(
                    "rocksdb_blobdb_gc_num_keys_relocated",
                    "rocksdb_blobdb_gc_num_keys_relocated: description."
                ),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_txn_overhead_duplicate_key".to_owned(),
            register_int_counter_vec!(
                opts!("rocksdb_txn_overhead_duplicate_key", "rocksdb_txn_overhead_duplicate_key: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_block_cache_filter_hit".to_owned(),
            register_int_counter_vec!(
                opts!("rocksdb_block_cache_filter_hit", "rocksdb_block_cache_filter_hit: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_number_rate_limiter_drains".to_owned(),
            register_int_counter_vec!(
                opts!("rocksdb_number_rate_limiter_drains", "rocksdb_number_rate_limiter_drains: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_number_keys_read".to_owned(),
            register_int_counter_vec!(
                opts!("rocksdb_number_keys_read", "rocksdb_number_keys_read: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_bloom_filter_prefix_checked".to_owned(),
            register_int_counter_vec!(
                opts!("rocksdb_bloom_filter_prefix_checked", "rocksdb_bloom_filter_prefix_checked: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_blobdb_write_micros".to_owned(),
            register_int_counter_vec!(
                opts!("rocksdb_blobdb_write_micros", "rocksdb_blobdb_write_micros: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_blobdb_write_inlined".to_owned(),
            register_int_counter_vec!(
                opts!("rocksdb_blobdb_write_inlined", "rocksdb_blobdb_write_inlined: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_blobdb_get_micros".to_owned(),
            register_int_counter_vec!(
                opts!("rocksdb_blobdb_get_micros", "rocksdb_blobdb_get_micros: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_db_write_micros".to_owned(),
            register_int_counter_vec!(
                opts!("rocksdb_db_write_micros", "rocksdb_db_write_micros: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_number_db_next_found".to_owned(),
            register_int_counter_vec!(
                opts!("rocksdb_number_db_next_found", "rocksdb_number_db_next_found: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_blobdb_compression_micros".to_owned(),
            register_int_counter_vec!(
                opts!("rocksdb_blobdb_compression_micros", "rocksdb_blobdb_compression_micros: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_no_file_opens".to_owned(),
            register_int_counter_vec!(
                opts!("rocksdb_no_file_opens", "rocksdb_no_file_opens: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_l0_slowdown_count".to_owned(),
            register_int_counter_vec!(
                opts!("rocksdb_l0_slowdown_count", "rocksdb_l0_slowdown_count: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_write_other".to_owned(),
            register_int_counter_vec!(
                opts!("rocksdb_write_other", "rocksdb_write_other: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_read_amp_estimate_useful_bytes".to_owned(),
            register_int_counter_vec!(
                opts!(
                    "rocksdb_read_amp_estimate_useful_bytes",
                    "rocksdb_read_amp_estimate_useful_bytes: description."
                ),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_block_cache_data_bytes_insert".to_owned(),
            register_int_counter_vec!(
                opts!(
                    "rocksdb_block_cache_data_bytes_insert",
                    "rocksdb_block_cache_data_bytes_insert: description."
                ),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_compaction_range_del_drop_obsolete".to_owned(),
            register_int_counter_vec!(
                opts!(
                    "rocksdb_compaction_range_del_drop_obsolete",
                    "rocksdb_compaction_range_del_drop_obsolete: description."
                ),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_blobdb_num_prev".to_owned(),
            register_int_counter_vec!(
                opts!("rocksdb_blobdb_num_prev", "rocksdb_blobdb_num_prev: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_blobdb_gc_num_new_files".to_owned(),
            register_int_counter_vec!(
                opts!("rocksdb_blobdb_gc_num_new_files", "rocksdb_blobdb_gc_num_new_files: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_block_cache_add_failures".to_owned(),
            register_int_counter_vec!(
                opts!("rocksdb_block_cache_add_failures", "rocksdb_block_cache_add_failures: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_number_keys_written".to_owned(),
            register_int_counter_vec!(
                opts!("rocksdb_number_keys_written", "rocksdb_number_keys_written: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_block_cache_filter_add".to_owned(),
            register_int_counter_vec!(
                opts!("rocksdb_block_cache_filter_add", "rocksdb_block_cache_filter_add: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_compact_write_marked_bytes".to_owned(),
            register_int_counter_vec!(
                opts!("rocksdb_compact_write_marked_bytes", "rocksdb_compact_write_marked_bytes: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_number_direct_load_table_properties".to_owned(),
            register_int_counter_vec!(
                opts!(
                    "rocksdb_number_direct_load_table_properties",
                    "rocksdb_number_direct_load_table_properties: description."
                ),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_blobdb_blob_file_synced".to_owned(),
            register_int_counter_vec!(
                opts!("rocksdb_blobdb_blob_file_synced", "rocksdb_blobdb_blob_file_synced: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_table_sync_micros".to_owned(),
            register_int_counter_vec!(
                opts!("rocksdb_table_sync_micros", "rocksdb_table_sync_micros: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_block_cache_index_add_redundant".to_owned(),
            register_int_counter_vec!(
                opts!(
                    "rocksdb_block_cache_index_add_redundant",
                    "rocksdb_block_cache_index_add_redundant: description."
                ),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_db_seek_micros".to_owned(),
            register_int_counter_vec!(
                opts!("rocksdb_db_seek_micros", "rocksdb_db_seek_micros: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_db_write_stall".to_owned(),
            register_int_counter_vec!(
                opts!("rocksdb_db_write_stall", "rocksdb_db_write_stall: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_compression_times_nanos".to_owned(),
            register_int_counter_vec!(
                opts!("rocksdb_compression_times_nanos", "rocksdb_compression_times_nanos: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_read_amp_total_read_bytes".to_owned(),
            register_int_counter_vec!(
                opts!("rocksdb_read_amp_total_read_bytes", "rocksdb_read_amp_total_read_bytes: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_sst_read_micros".to_owned(),
            register_int_counter_vec!(
                opts!("rocksdb_sst_read_micros", "rocksdb_sst_read_micros: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_memtable_hit".to_owned(),
            register_int_counter_vec!(
                opts!("rocksdb_memtable_hit", "rocksdb_memtable_hit: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_block_cachecompressed_miss".to_owned(),
            register_int_counter_vec!(
                opts!("rocksdb_block_cachecompressed_miss", "rocksdb_block_cachecompressed_miss: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_number_db_prev_found".to_owned(),
            register_int_counter_vec!(
                opts!("rocksdb_number_db_prev_found", "rocksdb_number_db_prev_found: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_bytes_decompressed".to_owned(),
            register_int_counter_vec!(
                opts!("rocksdb_bytes_decompressed", "rocksdb_bytes_decompressed: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_read_num_merge_operands".to_owned(),
            register_int_counter_vec!(
                opts!("rocksdb_read_num_merge_operands", "rocksdb_read_num_merge_operands: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_compact_read_marked_bytes".to_owned(),
            register_int_counter_vec!(
                opts!("rocksdb_compact_read_marked_bytes", "rocksdb_compact_read_marked_bytes: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_blobdb_gc_micros".to_owned(),
            register_int_counter_vec!(
                opts!("rocksdb_blobdb_gc_micros", "rocksdb_blobdb_gc_micros: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_block_cache_add_redundant".to_owned(),
            register_int_counter_vec!(
                opts!("rocksdb_block_cache_add_redundant", "rocksdb_block_cache_add_redundant: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_block_cache_compression_dict_bytes_evict".to_owned(),
            register_int_counter_vec!(
                opts!(
                    "rocksdb_block_cache_compression_dict_bytes_evict",
                    "rocksdb_block_cache_compression_dict_bytes_evict: description."
                ),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_l0_hit".to_owned(),
            register_int_counter_vec!(
                opts!("rocksdb_l0_hit", "rocksdb_l0_hit: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_bytes_read".to_owned(),
            register_int_counter_vec!(
                opts!("rocksdb_bytes_read", "rocksdb_bytes_read: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_block_cache_compression_dict_miss".to_owned(),
            register_int_counter_vec!(
                opts!(
                    "rocksdb_block_cache_compression_dict_miss",
                    "rocksdb_block_cache_compression_dict_miss: description."
                ),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_compact_read_bytes".to_owned(),
            register_int_counter_vec!(
                opts!("rocksdb_compact_read_bytes", "rocksdb_compact_read_bytes: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_blobdb_gc_num_keys_overwritten".to_owned(),
            register_int_counter_vec!(
                opts!(
                    "rocksdb_blobdb_gc_num_keys_overwritten",
                    "rocksdb_blobdb_gc_num_keys_overwritten: description."
                ),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_error_handler_autoresume_success_count".to_owned(),
            register_int_counter_vec!(
                opts!(
                    "rocksdb_error_handler_autoresume_success_count",
                    "rocksdb_error_handler_autoresume_success_count: description."
                ),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_num_sst_read_per_level".to_owned(),
            register_int_counter_vec!(
                opts!("rocksdb_num_sst_read_per_level", "rocksdb_num_sst_read_per_level: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_error_handler_autoresume_retry_count".to_owned(),
            register_int_counter_vec!(
                opts!(
                    "rocksdb_error_handler_autoresume_retry_count",
                    "rocksdb_error_handler_autoresume_retry_count: description."
                ),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_no_file_errors".to_owned(),
            register_int_counter_vec!(
                opts!("rocksdb_no_file_errors", "rocksdb_no_file_errors: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_blobdb_write_blob".to_owned(),
            register_int_counter_vec!(
                opts!("rocksdb_blobdb_write_blob", "rocksdb_blobdb_write_blob: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_error_handler_bg_io_errro_count".to_owned(),
            register_int_counter_vec!(
                opts!(
                    "rocksdb_error_handler_bg_io_errro_count",
                    "rocksdb_error_handler_bg_io_errro_count: description."
                ),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_blobdb_seek_micros".to_owned(),
            register_int_counter_vec!(
                opts!("rocksdb_blobdb_seek_micros", "rocksdb_blobdb_seek_micros: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_num_data_blocks_read_per_level".to_owned(),
            register_int_counter_vec!(
                opts!(
                    "rocksdb_num_data_blocks_read_per_level",
                    "rocksdb_num_data_blocks_read_per_level: description."
                ),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_number_iter_skip".to_owned(),
            register_int_counter_vec!(
                opts!("rocksdb_number_iter_skip", "rocksdb_number_iter_skip: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_block_cache_bytes_read".to_owned(),
            register_int_counter_vec!(
                opts!("rocksdb_block_cache_bytes_read", "rocksdb_block_cache_bytes_read: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_table_open_io_micros".to_owned(),
            register_int_counter_vec!(
                opts!("rocksdb_table_open_io_micros", "rocksdb_table_open_io_micros: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_stall_micros".to_owned(),
            register_int_counter_vec!(
                opts!("rocksdb_stall_micros", "rocksdb_stall_micros: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_compaction_key_drop_range_del".to_owned(),
            register_int_counter_vec!(
                opts!(
                    "rocksdb_compaction_key_drop_range_del",
                    "rocksdb_compaction_key_drop_range_del: description."
                ),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_bloom_filter_micros".to_owned(),
            register_int_counter_vec!(
                opts!("rocksdb_bloom_filter_micros", "rocksdb_bloom_filter_micros: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_sst_batch_size".to_owned(),
            register_int_counter_vec!(
                opts!("rocksdb_sst_batch_size", "rocksdb_sst_batch_size: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_bytes_written".to_owned(),
            register_int_counter_vec!(
                opts!("rocksdb_bytes_written", "rocksdb_bytes_written: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_blobdb_num_keys_read".to_owned(),
            register_int_counter_vec!(
                opts!("rocksdb_blobdb_num_keys_read", "rocksdb_blobdb_num_keys_read: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_blobdb_blob_file_bytes_read".to_owned(),
            register_int_counter_vec!(
                opts!("rocksdb_blobdb_blob_file_bytes_read", "rocksdb_blobdb_blob_file_bytes_read: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_error_handler_autoresume_retry_total_count".to_owned(),
            register_int_counter_vec!(
                opts!(
                    "rocksdb_error_handler_autoresume_retry_total_count",
                    "rocksdb_error_handler_autoresume_retry_total_count: description."
                ),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_block_cache_compression_dict_add_redundant".to_owned(),
            register_int_counter_vec!(
                opts!(
                    "rocksdb_block_cache_compression_dict_add_redundant",
                    "rocksdb_block_cache_compression_dict_add_redundant: description."
                ),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_number_multiget_keys_read".to_owned(),
            register_int_counter_vec!(
                opts!("rocksdb_number_multiget_keys_read", "rocksdb_number_multiget_keys_read: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_number_deletes_filtered".to_owned(),
            register_int_counter_vec!(
                opts!("rocksdb_number_deletes_filtered", "rocksdb_number_deletes_filtered: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_compaction_optimized_del_drop_obsolete".to_owned(),
            register_int_counter_vec!(
                opts!(
                    "rocksdb_compaction_optimized_del_drop_obsolete",
                    "rocksdb_compaction_optimized_del_drop_obsolete: description."
                ),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_compact_write_bytes".to_owned(),
            register_int_counter_vec!(
                opts!("rocksdb_compact_write_bytes", "rocksdb_compact_write_bytes: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_db_iter_bytes_read".to_owned(),
            register_int_counter_vec!(
                opts!("rocksdb_db_iter_bytes_read", "rocksdb_db_iter_bytes_read: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_block_cache_index_add".to_owned(),
            register_int_counter_vec!(
                opts!("rocksdb_block_cache_index_add", "rocksdb_block_cache_index_add: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_blobdb_blob_index_expired_size".to_owned(),
            register_int_counter_vec!(
                opts!(
                    "rocksdb_blobdb_blob_index_expired_size",
                    "rocksdb_blobdb_blob_index_expired_size: description."
                ),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_l0_num_files_stall_micros".to_owned(),
            register_int_counter_vec!(
                opts!("rocksdb_l0_num_files_stall_micros", "rocksdb_l0_num_files_stall_micros: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_number_block_compressed".to_owned(),
            register_int_counter_vec!(
                opts!("rocksdb_number_block_compressed", "rocksdb_number_block_compressed: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_bytes_per_write".to_owned(),
            register_int_counter_vec!(
                opts!("rocksdb_bytes_per_write", "rocksdb_bytes_per_write: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_bytes_per_multiget".to_owned(),
            register_int_counter_vec!(
                opts!("rocksdb_bytes_per_multiget", "rocksdb_bytes_per_multiget: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_decompression_times_nanos".to_owned(),
            register_int_counter_vec!(
                opts!("rocksdb_decompression_times_nanos", "rocksdb_decompression_times_nanos: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_blobdb_gc_bytes_overwritten".to_owned(),
            register_int_counter_vec!(
                opts!("rocksdb_blobdb_gc_bytes_overwritten", "rocksdb_blobdb_gc_bytes_overwritten: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_blobdb_gc_bytes_expired".to_owned(),
            register_int_counter_vec!(
                opts!("rocksdb_blobdb_gc_bytes_expired", "rocksdb_blobdb_gc_bytes_expired: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_wal_synced".to_owned(),
            register_int_counter_vec!(
                opts!("rocksdb_wal_synced", "rocksdb_wal_synced: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_blobdb_num_seek".to_owned(),
            register_int_counter_vec!(
                opts!("rocksdb_blobdb_num_seek", "rocksdb_blobdb_num_seek: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_blobdb_num_next".to_owned(),
            register_int_counter_vec!(
                opts!("rocksdb_blobdb_num_next", "rocksdb_blobdb_num_next: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_wal_bytes".to_owned(),
            register_int_counter_vec!(
                opts!("rocksdb_wal_bytes", "rocksdb_wal_bytes: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_compaction_times_micros".to_owned(),
            register_int_counter_vec!(
                opts!("rocksdb_compaction_times_micros", "rocksdb_compaction_times_micros: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_numfiles_in_singlecompaction".to_owned(),
            register_int_counter_vec!(
                opts!(
                    "rocksdb_numfiles_in_singlecompaction",
                    "rocksdb_numfiles_in_singlecompaction: description."
                ),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_bytes_per_read".to_owned(),
            register_int_counter_vec!(
                opts!("rocksdb_bytes_per_read", "rocksdb_bytes_per_read: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_blobdb_value_size".to_owned(),
            register_int_counter_vec!(
                opts!("rocksdb_blobdb_value_size", "rocksdb_blobdb_value_size: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_blobdb_blob_file_write_micros".to_owned(),
            register_int_counter_vec!(
                opts!(
                    "rocksdb_blobdb_blob_file_write_micros",
                    "rocksdb_blobdb_blob_file_write_micros: description."
                ),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_txn_overhead_mutex_snapshot".to_owned(),
            register_int_counter_vec!(
                opts!("rocksdb_txn_overhead_mutex_snapshot", "rocksdb_txn_overhead_mutex_snapshot: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_block_cache_index_bytes_insert".to_owned(),
            register_int_counter_vec!(
                opts!(
                    "rocksdb_block_cache_index_bytes_insert",
                    "rocksdb_block_cache_index_bytes_insert: description."
                ),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_number_superversion_releases".to_owned(),
            register_int_counter_vec!(
                opts!(
                    "rocksdb_number_superversion_releases",
                    "rocksdb_number_superversion_releases: description."
                ),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_blobdb_write_blob_ttl".to_owned(),
            register_int_counter_vec!(
                opts!("rocksdb_blobdb_write_blob_ttl", "rocksdb_blobdb_write_blob_ttl: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_number_keys_updated".to_owned(),
            register_int_counter_vec!(
                opts!("rocksdb_number_keys_updated", "rocksdb_number_keys_updated: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_l0_slowdown_micros".to_owned(),
            register_int_counter_vec!(
                opts!("rocksdb_l0_slowdown_micros", "rocksdb_l0_slowdown_micros: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_number_reseeks_iteration".to_owned(),
            register_int_counter_vec!(
                opts!("rocksdb_number_reseeks_iteration", "rocksdb_number_reseeks_iteration: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_write_self".to_owned(),
            register_int_counter_vec!(
                opts!("rocksdb_write_self", "rocksdb_write_self: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_compaction_key_drop_new".to_owned(),
            register_int_counter_vec!(
                opts!("rocksdb_compaction_key_drop_new", "rocksdb_compaction_key_drop_new: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_write_timeout".to_owned(),
            register_int_counter_vec!(
                opts!("rocksdb_write_timeout", "rocksdb_write_timeout: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_blobdb_blob_file_read_micros".to_owned(),
            register_int_counter_vec!(
                opts!(
                    "rocksdb_blobdb_blob_file_read_micros",
                    "rocksdb_blobdb_blob_file_read_micros: description."
                ),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_block_cache_data_hit".to_owned(),
            register_int_counter_vec!(
                opts!("rocksdb_block_cache_data_hit", "rocksdb_block_cache_data_hit: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_flush_write_bytes".to_owned(),
            register_int_counter_vec!(
                opts!("rocksdb_flush_write_bytes", "rocksdb_flush_write_bytes: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_error_handler_bg_errro_count".to_owned(),
            register_int_counter_vec!(
                opts!(
                    "rocksdb_error_handler_bg_errro_count",
                    "rocksdb_error_handler_bg_errro_count: description."
                ),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_number_db_prev".to_owned(),
            register_int_counter_vec!(
                opts!("rocksdb_number_db_prev", "rocksdb_number_db_prev: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_blobdb_num_get".to_owned(),
            register_int_counter_vec!(
                opts!("rocksdb_blobdb_num_get", "rocksdb_blobdb_num_get: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_num_iterator_created".to_owned(),
            register_int_counter_vec!(
                opts!("rocksdb_num_iterator_created", "rocksdb_num_iterator_created: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_bloom_filter_useful".to_owned(),
            register_int_counter_vec!(
                opts!("rocksdb_bloom_filter_useful", "rocksdb_bloom_filter_useful: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_block_cache_filter_bytes_insert".to_owned(),
            register_int_counter_vec!(
                opts!(
                    "rocksdb_block_cache_filter_bytes_insert",
                    "rocksdb_block_cache_filter_bytes_insert: description."
                ),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_blobdb_blob_file_bytes_written".to_owned(),
            register_int_counter_vec!(
                opts!(
                    "rocksdb_blobdb_blob_file_bytes_written",
                    "rocksdb_blobdb_blob_file_bytes_written: description."
                ),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_num_files_stall_count".to_owned(),
            register_int_counter_vec!(
                opts!("rocksdb_num_files_stall_count", "rocksdb_num_files_stall_count: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_blobdb_num_keys_written".to_owned(),
            register_int_counter_vec!(
                opts!("rocksdb_blobdb_num_keys_written", "rocksdb_blobdb_num_keys_written: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_error_handler_autoresume_count".to_owned(),
            register_int_counter_vec!(
                opts!(
                    "rocksdb_error_handler_autoresume_count",
                    "rocksdb_error_handler_autoresume_count: description."
                ),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_rate_limit_delay_millis".to_owned(),
            register_int_counter_vec!(
                opts!("rocksdb_rate_limit_delay_millis", "rocksdb_rate_limit_delay_millis: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_db_flush_micros".to_owned(),
            register_int_counter_vec!(
                opts!("rocksdb_db_flush_micros", "rocksdb_db_flush_micros: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_block_cache_filter_bytes_evict".to_owned(),
            register_int_counter_vec!(
                opts!(
                    "rocksdb_block_cache_filter_bytes_evict",
                    "rocksdb_block_cache_filter_bytes_evict: description."
                ),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_hard_rate_limit_delay_count".to_owned(),
            register_int_counter_vec!(
                opts!("rocksdb_hard_rate_limit_delay_count", "rocksdb_hard_rate_limit_delay_count: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_compaction_times_cpu_micros".to_owned(),
            register_int_counter_vec!(
                opts!("rocksdb_compaction_times_cpu_micros", "rocksdb_compaction_times_cpu_micros: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_compaction_outfile_sync_micros".to_owned(),
            register_int_counter_vec!(
                opts!(
                    "rocksdb_compaction_outfile_sync_micros",
                    "rocksdb_compaction_outfile_sync_micros: description."
                ),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_blobdb_gc_num_keys_expired".to_owned(),
            register_int_counter_vec!(
                opts!("rocksdb_blobdb_gc_num_keys_expired", "rocksdb_blobdb_gc_num_keys_expired: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_block_cache_filter_miss".to_owned(),
            register_int_counter_vec!(
                opts!("rocksdb_block_cache_filter_miss", "rocksdb_block_cache_filter_miss: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_memtable_compaction_micros".to_owned(),
            register_int_counter_vec!(
                opts!("rocksdb_memtable_compaction_micros", "rocksdb_memtable_compaction_micros: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_blobdb_num_multiget".to_owned(),
            register_int_counter_vec!(
                opts!("rocksdb_blobdb_num_multiget", "rocksdb_blobdb_num_multiget: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_filter_operation_time_nanos".to_owned(),
            register_int_counter_vec!(
                opts!("rocksdb_filter_operation_time_nanos", "rocksdb_filter_operation_time_nanos: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_number_block_decompressed".to_owned(),
            register_int_counter_vec!(
                opts!("rocksdb_number_block_decompressed", "rocksdb_number_block_decompressed: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_row_cache_miss".to_owned(),
            register_int_counter_vec!(
                opts!("rocksdb_row_cache_miss", "rocksdb_row_cache_miss: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_block_cache_compression_dict_hit".to_owned(),
            register_int_counter_vec!(
                opts!(
                    "rocksdb_block_cache_compression_dict_hit",
                    "rocksdb_block_cache_compression_dict_hit: description."
                ),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_block_cache_compression_dict_add".to_owned(),
            register_int_counter_vec!(
                opts!(
                    "rocksdb_block_cache_compression_dict_add",
                    "rocksdb_block_cache_compression_dict_add: description."
                ),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_block_cache_compression_dict_bytes_insert".to_owned(),
            register_int_counter_vec!(
                opts!(
                    "rocksdb_block_cache_compression_dict_bytes_insert",
                    "rocksdb_block_cache_compression_dict_bytes_insert: description."
                ),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_db_multiget_micros".to_owned(),
            register_int_counter_vec!(
                opts!("rocksdb_db_multiget_micros", "rocksdb_db_multiget_micros: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_block_cache_bytes_write".to_owned(),
            register_int_counter_vec!(
                opts!("rocksdb_block_cache_bytes_write", "rocksdb_block_cache_bytes_write: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
    ]);
    pub static ref ROCKSDB_GAUGES: HashMap<String, GaugeVec> = HashMap::from([
        (
            "rocksdb_blobdb_next_micros_p95".to_owned(),
            register_gauge_vec!(
                opts!("rocksdb_blobdb_next_micros_p95", "rocksdb_blobdb_next_micros_p95: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_db_seek_micros_p100".to_owned(),
            register_gauge_vec!(
                opts!("rocksdb_db_seek_micros_p100", "rocksdb_db_seek_micros_p100: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_sst_read_micros_p50".to_owned(),
            register_gauge_vec!(
                opts!("rocksdb_sst_read_micros_p50", "rocksdb_sst_read_micros_p50: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_sst_read_micros_p100".to_owned(),
            register_gauge_vec!(
                opts!("rocksdb_sst_read_micros_p100", "rocksdb_sst_read_micros_p100: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_bytes_per_read_p95".to_owned(),
            register_gauge_vec!(
                opts!("rocksdb_bytes_per_read_p95", "rocksdb_bytes_per_read_p95: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_db_multiget_micros_p95".to_owned(),
            register_gauge_vec!(
                opts!("rocksdb_db_multiget_micros_p95", "rocksdb_db_multiget_micros_p95: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_error_handler_autoresume_retry_count_p50".to_owned(),
            register_gauge_vec!(
                opts!(
                    "rocksdb_error_handler_autoresume_retry_count_p50",
                    "rocksdb_error_handler_autoresume_retry_count_p50: description."
                ),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_compression_times_nanos_p100".to_owned(),
            register_gauge_vec!(
                opts!(
                    "rocksdb_compression_times_nanos_p100",
                    "rocksdb_compression_times_nanos_p100: description."
                ),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_db_get_micros_p99".to_owned(),
            register_gauge_vec!(
                opts!("rocksdb_db_get_micros_p99", "rocksdb_db_get_micros_p99: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_blobdb_key_size_p99".to_owned(),
            register_gauge_vec!(
                opts!("rocksdb_blobdb_key_size_p99", "rocksdb_blobdb_key_size_p99: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_compaction_times_micros_p95".to_owned(),
            register_gauge_vec!(
                opts!("rocksdb_compaction_times_micros_p95", "rocksdb_compaction_times_micros_p95: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_wal_file_sync_micros_p50".to_owned(),
            register_gauge_vec!(
                opts!("rocksdb_wal_file_sync_micros_p50", "rocksdb_wal_file_sync_micros_p50: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_blobdb_multiget_micros_p50".to_owned(),
            register_gauge_vec!(
                opts!("rocksdb_blobdb_multiget_micros_p50", "rocksdb_blobdb_multiget_micros_p50: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_blobdb_compression_micros_p99".to_owned(),
            register_gauge_vec!(
                opts!(
                    "rocksdb_blobdb_compression_micros_p99",
                    "rocksdb_blobdb_compression_micros_p99: description."
                ),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_numfiles_in_singlecompaction_p99".to_owned(),
            register_gauge_vec!(
                opts!(
                    "rocksdb_numfiles_in_singlecompaction_p99",
                    "rocksdb_numfiles_in_singlecompaction_p99: description."
                ),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_blobdb_blob_file_read_micros_p50".to_owned(),
            register_gauge_vec!(
                opts!(
                    "rocksdb_blobdb_blob_file_read_micros_p50",
                    "rocksdb_blobdb_blob_file_read_micros_p50: description."
                ),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_db_flush_micros_p95".to_owned(),
            register_gauge_vec!(
                opts!("rocksdb_db_flush_micros_p95", "rocksdb_db_flush_micros_p95: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_manifest_file_sync_micros_p100".to_owned(),
            register_gauge_vec!(
                opts!(
                    "rocksdb_manifest_file_sync_micros_p100",
                    "rocksdb_manifest_file_sync_micros_p100: description."
                ),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_manifest_file_sync_micros_p95".to_owned(),
            register_gauge_vec!(
                opts!(
                    "rocksdb_manifest_file_sync_micros_p95",
                    "rocksdb_manifest_file_sync_micros_p95: description."
                ),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_blobdb_compression_micros_p100".to_owned(),
            register_gauge_vec!(
                opts!(
                    "rocksdb_blobdb_compression_micros_p100",
                    "rocksdb_blobdb_compression_micros_p100: description."
                ),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_num_index_and_filter_blocks_read_per_level_p95".to_owned(),
            register_gauge_vec!(
                opts!(
                    "rocksdb_num_index_and_filter_blocks_read_per_level_p95",
                    "rocksdb_num_index_and_filter_blocks_read_per_level_p95: description."
                ),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_compaction_outfile_sync_micros_p99".to_owned(),
            register_gauge_vec!(
                opts!(
                    "rocksdb_compaction_outfile_sync_micros_p99",
                    "rocksdb_compaction_outfile_sync_micros_p99: description."
                ),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_db_multiget_micros_p99".to_owned(),
            register_gauge_vec!(
                opts!("rocksdb_db_multiget_micros_p99", "rocksdb_db_multiget_micros_p99: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_compression_times_nanos_p99".to_owned(),
            register_gauge_vec!(
                opts!("rocksdb_compression_times_nanos_p99", "rocksdb_compression_times_nanos_p99: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_read_block_compaction_micros_p100".to_owned(),
            register_gauge_vec!(
                opts!(
                    "rocksdb_read_block_compaction_micros_p100",
                    "rocksdb_read_block_compaction_micros_p100: description."
                ),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_num_sst_read_per_level_p99".to_owned(),
            register_gauge_vec!(
                opts!("rocksdb_num_sst_read_per_level_p99", "rocksdb_num_sst_read_per_level_p99: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_blobdb_decompression_micros_p95".to_owned(),
            register_gauge_vec!(
                opts!(
                    "rocksdb_blobdb_decompression_micros_p95",
                    "rocksdb_blobdb_decompression_micros_p95: description."
                ),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_blobdb_key_size_p95".to_owned(),
            register_gauge_vec!(
                opts!("rocksdb_blobdb_key_size_p95", "rocksdb_blobdb_key_size_p95: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_hard_rate_limit_delay_count_p50".to_owned(),
            register_gauge_vec!(
                opts!(
                    "rocksdb_hard_rate_limit_delay_count_p50",
                    "rocksdb_hard_rate_limit_delay_count_p50: description."
                ),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_sst_batch_size_p95".to_owned(),
            register_gauge_vec!(
                opts!("rocksdb_sst_batch_size_p95", "rocksdb_sst_batch_size_p95: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_compaction_outfile_sync_micros_p95".to_owned(),
            register_gauge_vec!(
                opts!(
                    "rocksdb_compaction_outfile_sync_micros_p95",
                    "rocksdb_compaction_outfile_sync_micros_p95: description."
                ),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_bytes_decompressed_p100".to_owned(),
            register_gauge_vec!(
                opts!("rocksdb_bytes_decompressed_p100", "rocksdb_bytes_decompressed_p100: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_blobdb_decompression_micros_p99".to_owned(),
            register_gauge_vec!(
                opts!(
                    "rocksdb_blobdb_decompression_micros_p99",
                    "rocksdb_blobdb_decompression_micros_p99: description."
                ),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_blobdb_prev_micros_p99".to_owned(),
            register_gauge_vec!(
                opts!("rocksdb_blobdb_prev_micros_p99", "rocksdb_blobdb_prev_micros_p99: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_compaction_times_cpu_micros_p99".to_owned(),
            register_gauge_vec!(
                opts!(
                    "rocksdb_compaction_times_cpu_micros_p99",
                    "rocksdb_compaction_times_cpu_micros_p99: description."
                ),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_blobdb_compression_micros_p50".to_owned(),
            register_gauge_vec!(
                opts!(
                    "rocksdb_blobdb_compression_micros_p50",
                    "rocksdb_blobdb_compression_micros_p50: description."
                ),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_num_subcompactions_scheduled_p100".to_owned(),
            register_gauge_vec!(
                opts!(
                    "rocksdb_num_subcompactions_scheduled_p100",
                    "rocksdb_num_subcompactions_scheduled_p100: description."
                ),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_subcompaction_setup_times_micros_p99".to_owned(),
            register_gauge_vec!(
                opts!(
                    "rocksdb_subcompaction_setup_times_micros_p99",
                    "rocksdb_subcompaction_setup_times_micros_p99: description."
                ),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_soft_rate_limit_delay_count_p95".to_owned(),
            register_gauge_vec!(
                opts!(
                    "rocksdb_soft_rate_limit_delay_count_p95",
                    "rocksdb_soft_rate_limit_delay_count_p95: description."
                ),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_table_sync_micros_p100".to_owned(),
            register_gauge_vec!(
                opts!("rocksdb_table_sync_micros_p100", "rocksdb_table_sync_micros_p100: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_num_files_stall_count_p99".to_owned(),
            register_gauge_vec!(
                opts!("rocksdb_num_files_stall_count_p99", "rocksdb_num_files_stall_count_p99: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_numfiles_in_singlecompaction_p95".to_owned(),
            register_gauge_vec!(
                opts!(
                    "rocksdb_numfiles_in_singlecompaction_p95",
                    "rocksdb_numfiles_in_singlecompaction_p95: description."
                ),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_read_block_compaction_micros_p99".to_owned(),
            register_gauge_vec!(
                opts!(
                    "rocksdb_read_block_compaction_micros_p99",
                    "rocksdb_read_block_compaction_micros_p99: description."
                ),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_bytes_compressed_p50".to_owned(),
            register_gauge_vec!(
                opts!("rocksdb_bytes_compressed_p50", "rocksdb_bytes_compressed_p50: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_bytes_compressed_p95".to_owned(),
            register_gauge_vec!(
                opts!("rocksdb_bytes_compressed_p95", "rocksdb_bytes_compressed_p95: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_blobdb_key_size_p100".to_owned(),
            register_gauge_vec!(
                opts!("rocksdb_blobdb_key_size_p100", "rocksdb_blobdb_key_size_p100: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_blobdb_value_size_p100".to_owned(),
            register_gauge_vec!(
                opts!("rocksdb_blobdb_value_size_p100", "rocksdb_blobdb_value_size_p100: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_table_sync_micros_p50".to_owned(),
            register_gauge_vec!(
                opts!("rocksdb_table_sync_micros_p50", "rocksdb_table_sync_micros_p50: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_read_block_get_micros_p99".to_owned(),
            register_gauge_vec!(
                opts!("rocksdb_read_block_get_micros_p99", "rocksdb_read_block_get_micros_p99: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_bytes_per_multiget_p95".to_owned(),
            register_gauge_vec!(
                opts!("rocksdb_bytes_per_multiget_p95", "rocksdb_bytes_per_multiget_p95: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_blobdb_multiget_micros_p100".to_owned(),
            register_gauge_vec!(
                opts!("rocksdb_blobdb_multiget_micros_p100", "rocksdb_blobdb_multiget_micros_p100: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_blobdb_blob_file_sync_micros_p100".to_owned(),
            register_gauge_vec!(
                opts!(
                    "rocksdb_blobdb_blob_file_sync_micros_p100",
                    "rocksdb_blobdb_blob_file_sync_micros_p100: description."
                ),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_blobdb_next_micros_p50".to_owned(),
            register_gauge_vec!(
                opts!("rocksdb_blobdb_next_micros_p50", "rocksdb_blobdb_next_micros_p50: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_compaction_outfile_sync_micros_p50".to_owned(),
            register_gauge_vec!(
                opts!(
                    "rocksdb_compaction_outfile_sync_micros_p50",
                    "rocksdb_compaction_outfile_sync_micros_p50: description."
                ),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_bytes_per_write_p95".to_owned(),
            register_gauge_vec!(
                opts!("rocksdb_bytes_per_write_p95", "rocksdb_bytes_per_write_p95: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_db_write_stall_p100".to_owned(),
            register_gauge_vec!(
                opts!("rocksdb_db_write_stall_p100", "rocksdb_db_write_stall_p100: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_blobdb_decompression_micros_p50".to_owned(),
            register_gauge_vec!(
                opts!(
                    "rocksdb_blobdb_decompression_micros_p50",
                    "rocksdb_blobdb_decompression_micros_p50: description."
                ),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_blobdb_prev_micros_p95".to_owned(),
            register_gauge_vec!(
                opts!("rocksdb_blobdb_prev_micros_p95", "rocksdb_blobdb_prev_micros_p95: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_db_flush_micros_p99".to_owned(),
            register_gauge_vec!(
                opts!("rocksdb_db_flush_micros_p99", "rocksdb_db_flush_micros_p99: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_compaction_times_micros_p50".to_owned(),
            register_gauge_vec!(
                opts!("rocksdb_compaction_times_micros_p50", "rocksdb_compaction_times_micros_p50: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_blobdb_seek_micros_p50".to_owned(),
            register_gauge_vec!(
                opts!("rocksdb_blobdb_seek_micros_p50", "rocksdb_blobdb_seek_micros_p50: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_blobdb_seek_micros_p95".to_owned(),
            register_gauge_vec!(
                opts!("rocksdb_blobdb_seek_micros_p95", "rocksdb_blobdb_seek_micros_p95: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_blobdb_get_micros_p50".to_owned(),
            register_gauge_vec!(
                opts!("rocksdb_blobdb_get_micros_p50", "rocksdb_blobdb_get_micros_p50: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_write_raw_block_micros_p99".to_owned(),
            register_gauge_vec!(
                opts!("rocksdb_write_raw_block_micros_p99", "rocksdb_write_raw_block_micros_p99: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_num_data_blocks_read_per_level_p50".to_owned(),
            register_gauge_vec!(
                opts!(
                    "rocksdb_num_data_blocks_read_per_level_p50",
                    "rocksdb_num_data_blocks_read_per_level_p50: description."
                ),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_wal_file_sync_micros_p100".to_owned(),
            register_gauge_vec!(
                opts!("rocksdb_wal_file_sync_micros_p100", "rocksdb_wal_file_sync_micros_p100: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_num_index_and_filter_blocks_read_per_level_p100".to_owned(),
            register_gauge_vec!(
                opts!(
                    "rocksdb_num_index_and_filter_blocks_read_per_level_p100",
                    "rocksdb_num_index_and_filter_blocks_read_per_level_p100: description."
                ),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_num_data_blocks_read_per_level_p100".to_owned(),
            register_gauge_vec!(
                opts!(
                    "rocksdb_num_data_blocks_read_per_level_p100",
                    "rocksdb_num_data_blocks_read_per_level_p100: description."
                ),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_num_data_blocks_read_per_level_p99".to_owned(),
            register_gauge_vec!(
                opts!(
                    "rocksdb_num_data_blocks_read_per_level_p99",
                    "rocksdb_num_data_blocks_read_per_level_p99: description."
                ),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_blobdb_blob_file_read_micros_p99".to_owned(),
            register_gauge_vec!(
                opts!(
                    "rocksdb_blobdb_blob_file_read_micros_p99",
                    "rocksdb_blobdb_blob_file_read_micros_p99: description."
                ),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_sst_batch_size_p100".to_owned(),
            register_gauge_vec!(
                opts!("rocksdb_sst_batch_size_p100", "rocksdb_sst_batch_size_p100: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_blobdb_value_size_p99".to_owned(),
            register_gauge_vec!(
                opts!("rocksdb_blobdb_value_size_p99", "rocksdb_blobdb_value_size_p99: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_subcompaction_setup_times_micros_p50".to_owned(),
            register_gauge_vec!(
                opts!(
                    "rocksdb_subcompaction_setup_times_micros_p50",
                    "rocksdb_subcompaction_setup_times_micros_p50: description."
                ),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_soft_rate_limit_delay_count_p100".to_owned(),
            register_gauge_vec!(
                opts!(
                    "rocksdb_soft_rate_limit_delay_count_p100",
                    "rocksdb_soft_rate_limit_delay_count_p100: description."
                ),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_blobdb_multiget_micros_p95".to_owned(),
            register_gauge_vec!(
                opts!("rocksdb_blobdb_multiget_micros_p95", "rocksdb_blobdb_multiget_micros_p95: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_decompression_times_nanos_p50".to_owned(),
            register_gauge_vec!(
                opts!(
                    "rocksdb_decompression_times_nanos_p50",
                    "rocksdb_decompression_times_nanos_p50: description."
                ),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_blobdb_prev_micros_p50".to_owned(),
            register_gauge_vec!(
                opts!("rocksdb_blobdb_prev_micros_p50", "rocksdb_blobdb_prev_micros_p50: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_compaction_times_micros_p100".to_owned(),
            register_gauge_vec!(
                opts!(
                    "rocksdb_compaction_times_micros_p100",
                    "rocksdb_compaction_times_micros_p100: description."
                ),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_bytes_per_multiget_p50".to_owned(),
            register_gauge_vec!(
                opts!("rocksdb_bytes_per_multiget_p50", "rocksdb_bytes_per_multiget_p50: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_read_block_get_micros_p50".to_owned(),
            register_gauge_vec!(
                opts!("rocksdb_read_block_get_micros_p50", "rocksdb_read_block_get_micros_p50: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_memtable_compaction_count_p100".to_owned(),
            register_gauge_vec!(
                opts!(
                    "rocksdb_memtable_compaction_count_p100",
                    "rocksdb_memtable_compaction_count_p100: description."
                ),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_bytes_decompressed_p50".to_owned(),
            register_gauge_vec!(
                opts!("rocksdb_bytes_decompressed_p50", "rocksdb_bytes_decompressed_p50: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_decompression_times_nanos_p100".to_owned(),
            register_gauge_vec!(
                opts!(
                    "rocksdb_decompression_times_nanos_p100",
                    "rocksdb_decompression_times_nanos_p100: description."
                ),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_blobdb_get_micros_p99".to_owned(),
            register_gauge_vec!(
                opts!("rocksdb_blobdb_get_micros_p99", "rocksdb_blobdb_get_micros_p99: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_l0_slowdown_count_p50".to_owned(),
            register_gauge_vec!(
                opts!("rocksdb_l0_slowdown_count_p50", "rocksdb_l0_slowdown_count_p50: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_decompression_times_nanos_p95".to_owned(),
            register_gauge_vec!(
                opts!(
                    "rocksdb_decompression_times_nanos_p95",
                    "rocksdb_decompression_times_nanos_p95: description."
                ),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_blobdb_get_micros_p95".to_owned(),
            register_gauge_vec!(
                opts!("rocksdb_blobdb_get_micros_p95", "rocksdb_blobdb_get_micros_p95: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_blobdb_seek_micros_p100".to_owned(),
            register_gauge_vec!(
                opts!("rocksdb_blobdb_seek_micros_p100", "rocksdb_blobdb_seek_micros_p100: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_wal_file_sync_micros_p99".to_owned(),
            register_gauge_vec!(
                opts!("rocksdb_wal_file_sync_micros_p99", "rocksdb_wal_file_sync_micros_p99: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_db_write_micros_p100".to_owned(),
            register_gauge_vec!(
                opts!("rocksdb_db_write_micros_p100", "rocksdb_db_write_micros_p100: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_manifest_file_sync_micros_p50".to_owned(),
            register_gauge_vec!(
                opts!(
                    "rocksdb_manifest_file_sync_micros_p50",
                    "rocksdb_manifest_file_sync_micros_p50: description."
                ),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_blobdb_compression_micros_p95".to_owned(),
            register_gauge_vec!(
                opts!(
                    "rocksdb_blobdb_compression_micros_p95",
                    "rocksdb_blobdb_compression_micros_p95: description."
                ),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_num_files_stall_count_p50".to_owned(),
            register_gauge_vec!(
                opts!("rocksdb_num_files_stall_count_p50", "rocksdb_num_files_stall_count_p50: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_blobdb_write_micros_p95".to_owned(),
            register_gauge_vec!(
                opts!("rocksdb_blobdb_write_micros_p95", "rocksdb_blobdb_write_micros_p95: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_table_open_io_micros_p95".to_owned(),
            register_gauge_vec!(
                opts!("rocksdb_table_open_io_micros_p95", "rocksdb_table_open_io_micros_p95: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_table_sync_micros_p99".to_owned(),
            register_gauge_vec!(
                opts!("rocksdb_table_sync_micros_p99", "rocksdb_table_sync_micros_p99: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_table_open_io_micros_p100".to_owned(),
            register_gauge_vec!(
                opts!("rocksdb_table_open_io_micros_p100", "rocksdb_table_open_io_micros_p100: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_hard_rate_limit_delay_count_p95".to_owned(),
            register_gauge_vec!(
                opts!(
                    "rocksdb_hard_rate_limit_delay_count_p95",
                    "rocksdb_hard_rate_limit_delay_count_p95: description."
                ),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_memtable_compaction_count_p50".to_owned(),
            register_gauge_vec!(
                opts!(
                    "rocksdb_memtable_compaction_count_p50",
                    "rocksdb_memtable_compaction_count_p50: description."
                ),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_blobdb_blob_file_write_micros_p99".to_owned(),
            register_gauge_vec!(
                opts!(
                    "rocksdb_blobdb_blob_file_write_micros_p99",
                    "rocksdb_blobdb_blob_file_write_micros_p99: description."
                ),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_compaction_times_cpu_micros_p50".to_owned(),
            register_gauge_vec!(
                opts!(
                    "rocksdb_compaction_times_cpu_micros_p50",
                    "rocksdb_compaction_times_cpu_micros_p50: description."
                ),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_table_open_io_micros_p99".to_owned(),
            register_gauge_vec!(
                opts!("rocksdb_table_open_io_micros_p99", "rocksdb_table_open_io_micros_p99: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_bytes_per_write_p99".to_owned(),
            register_gauge_vec!(
                opts!("rocksdb_bytes_per_write_p99", "rocksdb_bytes_per_write_p99: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_blobdb_next_micros_p99".to_owned(),
            register_gauge_vec!(
                opts!("rocksdb_blobdb_next_micros_p99", "rocksdb_blobdb_next_micros_p99: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_blobdb_value_size_p95".to_owned(),
            register_gauge_vec!(
                opts!("rocksdb_blobdb_value_size_p95", "rocksdb_blobdb_value_size_p95: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_num_sst_read_per_level_p50".to_owned(),
            register_gauge_vec!(
                opts!("rocksdb_num_sst_read_per_level_p50", "rocksdb_num_sst_read_per_level_p50: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_num_subcompactions_scheduled_p99".to_owned(),
            register_gauge_vec!(
                opts!(
                    "rocksdb_num_subcompactions_scheduled_p99",
                    "rocksdb_num_subcompactions_scheduled_p99: description."
                ),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_decompression_times_nanos_p99".to_owned(),
            register_gauge_vec!(
                opts!(
                    "rocksdb_decompression_times_nanos_p99",
                    "rocksdb_decompression_times_nanos_p99: description."
                ),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_read_block_get_micros_p95".to_owned(),
            register_gauge_vec!(
                opts!("rocksdb_read_block_get_micros_p95", "rocksdb_read_block_get_micros_p95: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_num_subcompactions_scheduled_p95".to_owned(),
            register_gauge_vec!(
                opts!(
                    "rocksdb_num_subcompactions_scheduled_p95",
                    "rocksdb_num_subcompactions_scheduled_p95: description."
                ),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_blobdb_multiget_micros_p99".to_owned(),
            register_gauge_vec!(
                opts!("rocksdb_blobdb_multiget_micros_p99", "rocksdb_blobdb_multiget_micros_p99: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_db_get_micros_p100".to_owned(),
            register_gauge_vec!(
                opts!("rocksdb_db_get_micros_p100", "rocksdb_db_get_micros_p100: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_blobdb_write_micros_p50".to_owned(),
            register_gauge_vec!(
                opts!("rocksdb_blobdb_write_micros_p50", "rocksdb_blobdb_write_micros_p50: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_compression_times_nanos_p95".to_owned(),
            register_gauge_vec!(
                opts!("rocksdb_compression_times_nanos_p95", "rocksdb_compression_times_nanos_p95: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_compaction_times_cpu_micros_p100".to_owned(),
            register_gauge_vec!(
                opts!(
                    "rocksdb_compaction_times_cpu_micros_p100",
                    "rocksdb_compaction_times_cpu_micros_p100: description."
                ),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_num_index_and_filter_blocks_read_per_level_p99".to_owned(),
            register_gauge_vec!(
                opts!(
                    "rocksdb_num_index_and_filter_blocks_read_per_level_p99",
                    "rocksdb_num_index_and_filter_blocks_read_per_level_p99: description."
                ),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_db_write_micros_p95".to_owned(),
            register_gauge_vec!(
                opts!("rocksdb_db_write_micros_p95", "rocksdb_db_write_micros_p95: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_bytes_decompressed_p95".to_owned(),
            register_gauge_vec!(
                opts!("rocksdb_bytes_decompressed_p95", "rocksdb_bytes_decompressed_p95: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_wal_file_sync_micros_p95".to_owned(),
            register_gauge_vec!(
                opts!("rocksdb_wal_file_sync_micros_p95", "rocksdb_wal_file_sync_micros_p95: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_blobdb_seek_micros_p99".to_owned(),
            register_gauge_vec!(
                opts!("rocksdb_blobdb_seek_micros_p99", "rocksdb_blobdb_seek_micros_p99: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_blobdb_blob_file_write_micros_p100".to_owned(),
            register_gauge_vec!(
                opts!(
                    "rocksdb_blobdb_blob_file_write_micros_p100",
                    "rocksdb_blobdb_blob_file_write_micros_p100: description."
                ),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_sst_read_micros_p95".to_owned(),
            register_gauge_vec!(
                opts!("rocksdb_sst_read_micros_p95", "rocksdb_sst_read_micros_p95: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_read_num_merge_operands_p95".to_owned(),
            register_gauge_vec!(
                opts!("rocksdb_read_num_merge_operands_p95", "rocksdb_read_num_merge_operands_p95: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_sst_read_micros_p99".to_owned(),
            register_gauge_vec!(
                opts!("rocksdb_sst_read_micros_p99", "rocksdb_sst_read_micros_p99: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_db_multiget_micros_p50".to_owned(),
            register_gauge_vec!(
                opts!("rocksdb_db_multiget_micros_p50", "rocksdb_db_multiget_micros_p50: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_write_raw_block_micros_p50".to_owned(),
            register_gauge_vec!(
                opts!("rocksdb_write_raw_block_micros_p50", "rocksdb_write_raw_block_micros_p50: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_subcompaction_setup_times_micros_p95".to_owned(),
            register_gauge_vec!(
                opts!(
                    "rocksdb_subcompaction_setup_times_micros_p95",
                    "rocksdb_subcompaction_setup_times_micros_p95: description."
                ),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_read_block_get_micros_p100".to_owned(),
            register_gauge_vec!(
                opts!("rocksdb_read_block_get_micros_p100", "rocksdb_read_block_get_micros_p100: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_db_seek_micros_p95".to_owned(),
            register_gauge_vec!(
                opts!("rocksdb_db_seek_micros_p95", "rocksdb_db_seek_micros_p95: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_bytes_per_read_p99".to_owned(),
            register_gauge_vec!(
                opts!("rocksdb_bytes_per_read_p99", "rocksdb_bytes_per_read_p99: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_bytes_compressed_p100".to_owned(),
            register_gauge_vec!(
                opts!("rocksdb_bytes_compressed_p100", "rocksdb_bytes_compressed_p100: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_blobdb_value_size_p50".to_owned(),
            register_gauge_vec!(
                opts!("rocksdb_blobdb_value_size_p50", "rocksdb_blobdb_value_size_p50: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_blobdb_write_micros_p99".to_owned(),
            register_gauge_vec!(
                opts!("rocksdb_blobdb_write_micros_p99", "rocksdb_blobdb_write_micros_p99: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_hard_rate_limit_delay_count_p100".to_owned(),
            register_gauge_vec!(
                opts!(
                    "rocksdb_hard_rate_limit_delay_count_p100",
                    "rocksdb_hard_rate_limit_delay_count_p100: description."
                ),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_error_handler_autoresume_retry_count_p95".to_owned(),
            register_gauge_vec!(
                opts!(
                    "rocksdb_error_handler_autoresume_retry_count_p95",
                    "rocksdb_error_handler_autoresume_retry_count_p95: description."
                ),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_db_multiget_micros_p100".to_owned(),
            register_gauge_vec!(
                opts!("rocksdb_db_multiget_micros_p100", "rocksdb_db_multiget_micros_p100: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_bytes_per_read_p100".to_owned(),
            register_gauge_vec!(
                opts!("rocksdb_bytes_per_read_p100", "rocksdb_bytes_per_read_p100: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_bytes_per_multiget_p100".to_owned(),
            register_gauge_vec!(
                opts!("rocksdb_bytes_per_multiget_p100", "rocksdb_bytes_per_multiget_p100: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_error_handler_autoresume_retry_count_p100".to_owned(),
            register_gauge_vec!(
                opts!(
                    "rocksdb_error_handler_autoresume_retry_count_p100",
                    "rocksdb_error_handler_autoresume_retry_count_p100: description."
                ),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_numfiles_in_singlecompaction_p100".to_owned(),
            register_gauge_vec!(
                opts!(
                    "rocksdb_numfiles_in_singlecompaction_p100",
                    "rocksdb_numfiles_in_singlecompaction_p100: description."
                ),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_db_write_stall_p50".to_owned(),
            register_gauge_vec!(
                opts!("rocksdb_db_write_stall_p50", "rocksdb_db_write_stall_p50: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_bytes_compressed_p99".to_owned(),
            register_gauge_vec!(
                opts!("rocksdb_bytes_compressed_p99", "rocksdb_bytes_compressed_p99: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_blobdb_key_size_p50".to_owned(),
            register_gauge_vec!(
                opts!("rocksdb_blobdb_key_size_p50", "rocksdb_blobdb_key_size_p50: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_blobdb_gc_micros_p95".to_owned(),
            register_gauge_vec!(
                opts!("rocksdb_blobdb_gc_micros_p95", "rocksdb_blobdb_gc_micros_p95: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_blobdb_blob_file_read_micros_p100".to_owned(),
            register_gauge_vec!(
                opts!(
                    "rocksdb_blobdb_blob_file_read_micros_p100",
                    "rocksdb_blobdb_blob_file_read_micros_p100: description."
                ),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_db_write_stall_p99".to_owned(),
            register_gauge_vec!(
                opts!("rocksdb_db_write_stall_p99", "rocksdb_db_write_stall_p99: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_db_write_micros_p50".to_owned(),
            register_gauge_vec!(
                opts!("rocksdb_db_write_micros_p50", "rocksdb_db_write_micros_p50: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_bytes_per_write_p100".to_owned(),
            register_gauge_vec!(
                opts!("rocksdb_bytes_per_write_p100", "rocksdb_bytes_per_write_p100: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_blobdb_get_micros_p100".to_owned(),
            register_gauge_vec!(
                opts!("rocksdb_blobdb_get_micros_p100", "rocksdb_blobdb_get_micros_p100: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_blobdb_gc_micros_p99".to_owned(),
            register_gauge_vec!(
                opts!("rocksdb_blobdb_gc_micros_p99", "rocksdb_blobdb_gc_micros_p99: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_db_get_micros_p50".to_owned(),
            register_gauge_vec!(
                opts!("rocksdb_db_get_micros_p50", "rocksdb_db_get_micros_p50: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_compaction_times_micros_p99".to_owned(),
            register_gauge_vec!(
                opts!("rocksdb_compaction_times_micros_p99", "rocksdb_compaction_times_micros_p99: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_write_raw_block_micros_p95".to_owned(),
            register_gauge_vec!(
                opts!("rocksdb_write_raw_block_micros_p95", "rocksdb_write_raw_block_micros_p95: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_read_block_compaction_micros_p95".to_owned(),
            register_gauge_vec!(
                opts!(
                    "rocksdb_read_block_compaction_micros_p95",
                    "rocksdb_read_block_compaction_micros_p95: description."
                ),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_db_seek_micros_p50".to_owned(),
            register_gauge_vec!(
                opts!("rocksdb_db_seek_micros_p50", "rocksdb_db_seek_micros_p50: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_num_sst_read_per_level_p100".to_owned(),
            register_gauge_vec!(
                opts!("rocksdb_num_sst_read_per_level_p100", "rocksdb_num_sst_read_per_level_p100: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_blobdb_blob_file_sync_micros_p95".to_owned(),
            register_gauge_vec!(
                opts!(
                    "rocksdb_blobdb_blob_file_sync_micros_p95",
                    "rocksdb_blobdb_blob_file_sync_micros_p95: description."
                ),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_error_handler_autoresume_retry_count_p99".to_owned(),
            register_gauge_vec!(
                opts!(
                    "rocksdb_error_handler_autoresume_retry_count_p99",
                    "rocksdb_error_handler_autoresume_retry_count_p99: description."
                ),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_num_subcompactions_scheduled_p50".to_owned(),
            register_gauge_vec!(
                opts!(
                    "rocksdb_num_subcompactions_scheduled_p50",
                    "rocksdb_num_subcompactions_scheduled_p50: description."
                ),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_db_flush_micros_p50".to_owned(),
            register_gauge_vec!(
                opts!("rocksdb_db_flush_micros_p50", "rocksdb_db_flush_micros_p50: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_manifest_file_sync_micros_p99".to_owned(),
            register_gauge_vec!(
                opts!(
                    "rocksdb_manifest_file_sync_micros_p99",
                    "rocksdb_manifest_file_sync_micros_p99: description."
                ),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_num_files_stall_count_p100".to_owned(),
            register_gauge_vec!(
                opts!("rocksdb_num_files_stall_count_p100", "rocksdb_num_files_stall_count_p100: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_db_seek_micros_p99".to_owned(),
            register_gauge_vec!(
                opts!("rocksdb_db_seek_micros_p99", "rocksdb_db_seek_micros_p99: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_db_get_micros_p95".to_owned(),
            register_gauge_vec!(
                opts!("rocksdb_db_get_micros_p95", "rocksdb_db_get_micros_p95: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_bytes_per_multiget_p99".to_owned(),
            register_gauge_vec!(
                opts!("rocksdb_bytes_per_multiget_p99", "rocksdb_bytes_per_multiget_p99: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_l0_slowdown_count_p99".to_owned(),
            register_gauge_vec!(
                opts!("rocksdb_l0_slowdown_count_p99", "rocksdb_l0_slowdown_count_p99: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_compaction_outfile_sync_micros_p100".to_owned(),
            register_gauge_vec!(
                opts!(
                    "rocksdb_compaction_outfile_sync_micros_p100",
                    "rocksdb_compaction_outfile_sync_micros_p100: description."
                ),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_blobdb_prev_micros_p100".to_owned(),
            register_gauge_vec!(
                opts!("rocksdb_blobdb_prev_micros_p100", "rocksdb_blobdb_prev_micros_p100: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_read_num_merge_operands_p50".to_owned(),
            register_gauge_vec!(
                opts!("rocksdb_read_num_merge_operands_p50", "rocksdb_read_num_merge_operands_p50: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_blobdb_decompression_micros_p100".to_owned(),
            register_gauge_vec!(
                opts!(
                    "rocksdb_blobdb_decompression_micros_p100",
                    "rocksdb_blobdb_decompression_micros_p100: description."
                ),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_sst_batch_size_p99".to_owned(),
            register_gauge_vec!(
                opts!("rocksdb_sst_batch_size_p99", "rocksdb_sst_batch_size_p99: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_subcompaction_setup_times_micros_p100".to_owned(),
            register_gauge_vec!(
                opts!(
                    "rocksdb_subcompaction_setup_times_micros_p100",
                    "rocksdb_subcompaction_setup_times_micros_p100: description."
                ),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_soft_rate_limit_delay_count_p99".to_owned(),
            register_gauge_vec!(
                opts!(
                    "rocksdb_soft_rate_limit_delay_count_p99",
                    "rocksdb_soft_rate_limit_delay_count_p99: description."
                ),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_blobdb_blob_file_write_micros_p95".to_owned(),
            register_gauge_vec!(
                opts!(
                    "rocksdb_blobdb_blob_file_write_micros_p95",
                    "rocksdb_blobdb_blob_file_write_micros_p95: description."
                ),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_blobdb_blob_file_read_micros_p95".to_owned(),
            register_gauge_vec!(
                opts!(
                    "rocksdb_blobdb_blob_file_read_micros_p95",
                    "rocksdb_blobdb_blob_file_read_micros_p95: description."
                ),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_db_flush_micros_p100".to_owned(),
            register_gauge_vec!(
                opts!("rocksdb_db_flush_micros_p100", "rocksdb_db_flush_micros_p100: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_num_data_blocks_read_per_level_p95".to_owned(),
            register_gauge_vec!(
                opts!(
                    "rocksdb_num_data_blocks_read_per_level_p95",
                    "rocksdb_num_data_blocks_read_per_level_p95: description."
                ),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_db_write_stall_p95".to_owned(),
            register_gauge_vec!(
                opts!("rocksdb_db_write_stall_p95", "rocksdb_db_write_stall_p95: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_blobdb_blob_file_sync_micros_p99".to_owned(),
            register_gauge_vec!(
                opts!(
                    "rocksdb_blobdb_blob_file_sync_micros_p99",
                    "rocksdb_blobdb_blob_file_sync_micros_p99: description."
                ),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_numfiles_in_singlecompaction_p50".to_owned(),
            register_gauge_vec!(
                opts!(
                    "rocksdb_numfiles_in_singlecompaction_p50",
                    "rocksdb_numfiles_in_singlecompaction_p50: description."
                ),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_table_open_io_micros_p50".to_owned(),
            register_gauge_vec!(
                opts!("rocksdb_table_open_io_micros_p50", "rocksdb_table_open_io_micros_p50: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_blobdb_gc_micros_p100".to_owned(),
            register_gauge_vec!(
                opts!("rocksdb_blobdb_gc_micros_p100", "rocksdb_blobdb_gc_micros_p100: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_bytes_decompressed_p99".to_owned(),
            register_gauge_vec!(
                opts!("rocksdb_bytes_decompressed_p99", "rocksdb_bytes_decompressed_p99: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_l0_slowdown_count_p100".to_owned(),
            register_gauge_vec!(
                opts!("rocksdb_l0_slowdown_count_p100", "rocksdb_l0_slowdown_count_p100: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_read_num_merge_operands_p100".to_owned(),
            register_gauge_vec!(
                opts!(
                    "rocksdb_read_num_merge_operands_p100",
                    "rocksdb_read_num_merge_operands_p100: description."
                ),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_blobdb_gc_micros_p50".to_owned(),
            register_gauge_vec!(
                opts!("rocksdb_blobdb_gc_micros_p50", "rocksdb_blobdb_gc_micros_p50: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_write_raw_block_micros_p100".to_owned(),
            register_gauge_vec!(
                opts!("rocksdb_write_raw_block_micros_p100", "rocksdb_write_raw_block_micros_p100: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_hard_rate_limit_delay_count_p99".to_owned(),
            register_gauge_vec!(
                opts!(
                    "rocksdb_hard_rate_limit_delay_count_p99",
                    "rocksdb_hard_rate_limit_delay_count_p99: description."
                ),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_blobdb_blob_file_write_micros_p50".to_owned(),
            register_gauge_vec!(
                opts!(
                    "rocksdb_blobdb_blob_file_write_micros_p50",
                    "rocksdb_blobdb_blob_file_write_micros_p50: description."
                ),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_sst_batch_size_p50".to_owned(),
            register_gauge_vec!(
                opts!("rocksdb_sst_batch_size_p50", "rocksdb_sst_batch_size_p50: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_blobdb_write_micros_p100".to_owned(),
            register_gauge_vec!(
                opts!("rocksdb_blobdb_write_micros_p100", "rocksdb_blobdb_write_micros_p100: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_l0_slowdown_count_p95".to_owned(),
            register_gauge_vec!(
                opts!("rocksdb_l0_slowdown_count_p95", "rocksdb_l0_slowdown_count_p95: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_blobdb_blob_file_sync_micros_p50".to_owned(),
            register_gauge_vec!(
                opts!(
                    "rocksdb_blobdb_blob_file_sync_micros_p50",
                    "rocksdb_blobdb_blob_file_sync_micros_p50: description."
                ),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_table_sync_micros_p95".to_owned(),
            register_gauge_vec!(
                opts!("rocksdb_table_sync_micros_p95", "rocksdb_table_sync_micros_p95: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_compaction_times_cpu_micros_p95".to_owned(),
            register_gauge_vec!(
                opts!(
                    "rocksdb_compaction_times_cpu_micros_p95",
                    "rocksdb_compaction_times_cpu_micros_p95: description."
                ),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_soft_rate_limit_delay_count_p50".to_owned(),
            register_gauge_vec!(
                opts!(
                    "rocksdb_soft_rate_limit_delay_count_p50",
                    "rocksdb_soft_rate_limit_delay_count_p50: description."
                ),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_db_write_micros_p99".to_owned(),
            register_gauge_vec!(
                opts!("rocksdb_db_write_micros_p99", "rocksdb_db_write_micros_p99: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_num_files_stall_count_p95".to_owned(),
            register_gauge_vec!(
                opts!("rocksdb_num_files_stall_count_p95", "rocksdb_num_files_stall_count_p95: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_bytes_per_read_p50".to_owned(),
            register_gauge_vec!(
                opts!("rocksdb_bytes_per_read_p50", "rocksdb_bytes_per_read_p50: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_compression_times_nanos_p50".to_owned(),
            register_gauge_vec!(
                opts!("rocksdb_compression_times_nanos_p50", "rocksdb_compression_times_nanos_p50: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_num_index_and_filter_blocks_read_per_level_p50".to_owned(),
            register_gauge_vec!(
                opts!(
                    "rocksdb_num_index_and_filter_blocks_read_per_level_p50",
                    "rocksdb_num_index_and_filter_blocks_read_per_level_p50: description."
                ),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_blobdb_next_micros_p100".to_owned(),
            register_gauge_vec!(
                opts!("rocksdb_blobdb_next_micros_p100", "rocksdb_blobdb_next_micros_p100: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_num_sst_read_per_level_p95".to_owned(),
            register_gauge_vec!(
                opts!("rocksdb_num_sst_read_per_level_p95", "rocksdb_num_sst_read_per_level_p95: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_memtable_compaction_count_p99".to_owned(),
            register_gauge_vec!(
                opts!(
                    "rocksdb_memtable_compaction_count_p99",
                    "rocksdb_memtable_compaction_count_p99: description."
                ),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_read_block_compaction_micros_p50".to_owned(),
            register_gauge_vec!(
                opts!(
                    "rocksdb_read_block_compaction_micros_p50",
                    "rocksdb_read_block_compaction_micros_p50: description."
                ),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_memtable_compaction_count_p95".to_owned(),
            register_gauge_vec!(
                opts!(
                    "rocksdb_memtable_compaction_count_p95",
                    "rocksdb_memtable_compaction_count_p95: description."
                ),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_read_num_merge_operands_p99".to_owned(),
            register_gauge_vec!(
                opts!("rocksdb_read_num_merge_operands_p99", "rocksdb_read_num_merge_operands_p99: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
        (
            "rocksdb_bytes_per_write_p50".to_owned(),
            register_gauge_vec!(
                opts!("rocksdb_bytes_per_write_p50", "rocksdb_bytes_per_write_p50: description."),
                &["instance_id", "index_id", "organization_id"]
            )
            .unwrap()
        ),
    ]);
}
