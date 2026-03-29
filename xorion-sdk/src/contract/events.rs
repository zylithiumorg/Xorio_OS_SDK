//! Contract event logs handling

use serde::{Deserialize, Serialize};
use crate::error::Result;

/// Event log from contract execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventLog {
    /// Transaction hash
    pub transaction_hash: String,
    /// Block number
    pub block_number: u64,
    /// Log index in the block
    pub log_index: u32,
    /// Contract address that emitted the event
    pub address: String,
    /// Event name
    pub event_name: String,
    /// Event topics (indexed parameters)
    pub topics: Vec<String>,
    /// Event data (non-indexed parameters)
    pub data: String,
    /// Parsed event arguments
    pub args: Vec<EventArgument>,
}

/// Event argument
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventArgument {
    pub name: String,
    #[serde(rename = "type")]
    pub arg_type: String,
    pub value: String,
    pub indexed: bool,
}

impl EventLog {
    /// Create a new event log
    pub fn new(
        transaction_hash: String,
        block_number: u64,
        address: String,
        event_name: String,
    ) -> Self {
        EventLog {
            transaction_hash,
            block_number,
            log_index: 0,
            address,
            event_name,
            topics: Vec::new(),
            data: String::new(),
            args: Vec::new(),
        }
    }

    /// Set log index
    pub fn with_log_index(mut self, index: u32) -> Self {
        self.log_index = index;
        self
    }

    /// Set topics
    pub fn with_topics(mut self, topics: Vec<String>) -> Self {
        self.topics = topics;
        self
    }

    /// Set data
    pub fn with_data(mut self, data: String) -> Self {
        self.data = data;
        self
    }

    /// Add an argument
    pub fn add_argument(mut self, arg: EventArgument) -> Self {
        self.args.push(arg);
        self
    }

    /// Get indexed arguments
    pub fn indexed_args(&self) -> Vec<&EventArgument> {
        self.args.iter().filter(|a| a.indexed).collect()
    }

    /// Get non-indexed arguments
    pub fn non_indexed_args(&self) -> Vec<&EventArgument> {
        self.args.iter().filter(|a| !a.indexed).collect()
    }

    /// Get argument by name
    pub fn get_argument(&self, name: &str) -> Option<&EventArgument> {
        self.args.iter().find(|a| a.name == name)
    }
}

/// Event filter for querying logs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventFilter {
    /// Contract address
    pub address: Option<String>,
    /// Event topics to filter by
    pub topics: Vec<Option<Vec<String>>>,
    /// Starting block
    pub from_block: Option<u64>,
    /// Ending block
    pub to_block: Option<u64>,
}

impl EventFilter {
    /// Create a new filter for a specific contract
    pub fn new(address: String) -> Self {
        EventFilter {
            address: Some(address),
            topics: Vec::new(),
            from_block: None,
            to_block: None,
        }
    }

    /// Set from block
    pub fn from_block(mut self, block: u64) -> Self {
        self.from_block = Some(block);
        self
    }

    /// Set to block
    pub fn to_block(mut self, block: u64) -> Self {
        self.to_block = Some(block);
        self
    }

    /// Add topic filter
    pub fn add_topic(mut self, topic: Vec<String>) -> Self {
        self.topics.push(Some(topic));
        self
    }

    /// Convert to JSON-RPC filter object
    pub fn to_json(&self) -> serde_json::Value {
        let mut filter = serde_json::Map::new();

        if let Some(ref addr) = self.address {
            filter.insert("address".to_string(), serde_json::json!([addr]));
        }

        if let Some(from) = self.from_block {
            filter.insert(
                "fromBlock".to_string(),
                serde_json::json!(format!("0x{:x}", from)),
            );
        }

        if let Some(to) = self.to_block {
            filter.insert(
                "toBlock".to_string(),
                serde_json::json!(format!("0x{:x}", to)),
            );
        }

        if !self.topics.is_empty() {
            filter.insert("topics".to_string(), serde_json::json!(self.topics));
        }

        serde_json::Value::Object(filter)
    }
}

/// Topic builder for event filtering
pub struct TopicBuilder;

impl TopicBuilder {
    /// Build topic from address
    pub fn address_topic(address: &str) -> String {
        let stripped = address.strip_prefix("0x").unwrap_or(address);
        format!("0x{:0>64}", stripped)
    }

    /// Build topic from uint256 value
    pub fn uint_topic(value: u64) -> String {
        format!("0x{:0>64x}", value)
    }

    /// Build topic from string/hash
    pub fn hash_topic(hash: &str) -> String {
        if hash.starts_with("0x") {
            hash.to_string()
        } else {
            format!("0x{}", hash)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_event_log_creation() {
        let log = EventLog::new(
            "0xtxhash".to_string(),
            12345,
            "0xcontract".to_string(),
            "Transfer".to_string(),
        );

        assert_eq!(log.transaction_hash, "0xtxhash");
        assert_eq!(log.block_number, 12345);
        assert_eq!(log.event_name, "Transfer");
        assert!(log.args.is_empty());
    }

    #[test]
    fn test_event_filter() {
        let filter = EventFilter::new("0xcontract".to_string())
            .from_block(1000)
            .to_block(2000);

        assert_eq!(filter.address, Some("0xcontract".to_string()));
        assert_eq!(filter.from_block, Some(1000));
        assert_eq!(filter.to_block, Some(2000));
    }

    #[test]
    fn test_topic_builder() {
        let topic = TopicBuilder::address_topic("0x1234567890123456789012345678901234567890");
        assert_eq!(topic.len(), 66); // 0x + 64 chars
        
        let uint_topic = TopicBuilder::uint_topic(1000);
        assert!(uint_topic.starts_with("0x"));
    }
}
