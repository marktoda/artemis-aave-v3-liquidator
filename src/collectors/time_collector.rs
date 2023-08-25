use anyhow::Result;
use artemis_core::types::{Collector, CollectorStream};
use async_trait::async_trait;
use futures::StreamExt;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::time::Duration;
use tokio_stream::wrappers::IntervalStream;

/// A collector that listens for new blocks, and generates a stream of
/// [events](NewBlock) which contain the block number and hash.
pub struct TimeCollector {
    pub poll_secs: u64,
}

/// A new block event, containing the block number and hash.
#[derive(Debug, Clone)]
pub struct NewTick {
    pub timestamp: u64,
}

impl TimeCollector {
    pub fn new(poll_secs: u64) -> Self {
        Self { poll_secs }
    }
}

/// Implementation of the [Collector](Collector) trait for the [BlockCollector](BlockCollector).
/// This implementation uses the [PubsubClient](PubsubClient) to subscribe to new blocks.
#[async_trait]
impl Collector<NewTick> for TimeCollector {
    async fn get_event_stream(&self) -> Result<CollectorStream<'_, NewTick>> {
        let stream = IntervalStream::new(tokio::time::interval(Duration::from_secs(
            self.poll_secs,
        )))
        .then(|_| async {
            let timestamp: u64 = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Invalid timestamp")
                .as_secs();
            return NewTick { timestamp };
        });

        Ok(Box::pin(stream))
    }
}
