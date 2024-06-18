use crate::metrics::types::RocksDBMetrics;

use crate::metrics;

use regex::Regex;
use std::collections::HashMap;

use lazy_static::lazy_static;

lazy_static! {
    static ref RE_COUNT: Regex = Regex::new(r"^([\w\.]+)\s+(?:P\d{2,3}\s+:\s+\d+\.\d+\s+)*(?:COUNT\s+:\s+(\d+))").unwrap();
    static ref RE_GAUGE: Regex = Regex::new(r"^([\w\.]+)(?:\s+P50\s+:\s+([+-]?(?:[0-9]*[.])?[0-9]+)\s+P95\s+:\s+([+-]?(?:[0-9]*[.])?[0-9]+)\s+P99\s+:\s+([+-]?(?:[0-9]*[.])?[0-9]+)\s+P100\s+:\s+([+-]?(?:[0-9]*[.])?[0-9]+))").unwrap();
}

pub fn process_rocksdb_metrics(metrics_string: String) {
    // Add RocksDB metrics

    let mut metrics_counter_map: HashMap<String, u64> = HashMap::new();
    let mut metrics_gauge_map: HashMap<String, f64> = HashMap::new();

    // Read from Rocksdb
    for line in metrics_string.lines() {
        let caps = RE_COUNT.captures(line).unwrap();
        let metric_name: String = caps[1].to_owned().replace(".", "_");
        let metric_counter: u64 = caps[2].to_owned().parse::<u64>().unwrap();
        metrics_counter_map.insert(metric_name, metric_counter);

        // If gauges present
        if line.contains(" P50 ") && line.contains(" P95 ") {
            let caps = RE_GAUGE.captures(line).unwrap();

            let metric_p50: String = caps[2].to_owned();
            let metric_name: String = format!("{}_p50", caps[1].to_owned().replace(".", "_"));
            metrics_gauge_map.insert(metric_name.clone(), metric_p50.parse::<f64>().unwrap());

            let metric_p95: String = caps[3].to_owned();
            let metric_name: String = format!("{}_p95", caps[1].to_owned().replace(".", "_"));
            metrics_gauge_map.insert(metric_name.clone(), metric_p95.parse::<f64>().unwrap());

            let metric_p99: String = caps[4].to_owned();
            let metric_name: String = format!("{}_p99", caps[1].to_owned().replace(".", "_"));
            metrics_gauge_map.insert(metric_name.clone(), metric_p99.parse::<f64>().unwrap());

            let metric_p100: String = caps[5].to_owned();
            let metric_name: String = format!("{}_p100", caps[1].to_owned().replace(".", "_"));
            metrics_gauge_map.insert(metric_name.clone(), metric_p100.parse::<f64>().unwrap());
        }
    }

    for counter_key in metrics_counter_map.clone().keys() {
        if let Some(counter_value) = metrics_counter_map.get_mut(counter_key) {
            *counter_value -= metrics::ROCKSDB_COUNTERS[counter_key]
                .with_label_values(&[&metrics::INSTANCE_ID, &metrics::INDEX_ID, &metrics::ORGANIZATION_ID])
                .get();
        }
        metrics::ROCKSDB_COUNTERS[counter_key]
            .with_label_values(&[&metrics::INSTANCE_ID, &metrics::INDEX_ID, &metrics::ORGANIZATION_ID])
            .inc_by(metrics_counter_map[counter_key]);
    }

    for (gauge_key, gauge_value) in metrics_gauge_map {
        metrics::ROCKSDB_GAUGES[&gauge_key]
            .with_label_values(&[&metrics::INSTANCE_ID, &metrics::INDEX_ID, &metrics::ORGANIZATION_ID])
            .set(gauge_value);
    }

    // Substract bytes that are already set
    // rocksdb_metrics.bytes_written -= metrics::ROCKSDB_COUNTERS["rocksdb_bytes_written"]
    //     .with_label_values(&[&metrics::INSTANCE_ID, &metrics::INDEX_ID, &metrics::ORGANIZATION_ID])
    //     .get();

    // rocksdb_metrics.bytes_read -= metrics::ROCKSDB_BYTES_READ
    //     .with_label_values(&[&metrics::INSTANCE_ID, &metrics::INDEX_ID, &metrics::ORGANIZATION_ID])
    //     .get();

    // Increment counter
    // metrics::ROCKSDB_COUNTERS["rocksdb_bytes_written"]
    //     .with_label_values(&[&metrics::INSTANCE_ID, &metrics::INDEX_ID, &metrics::ORGANIZATION_ID])
    //     .inc_by(rocksdb_metrics.bytes_written);
    // metrics::ROCKSDB_BYTES_READ
    //     .with_label_values(&[&metrics::INSTANCE_ID, &metrics::INDEX_ID, &metrics::ORGANIZATION_ID])
    //     .inc_by(rocksdb_metrics.bytes_read);
}
