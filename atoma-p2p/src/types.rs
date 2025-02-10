use serde::{Deserialize, Serialize};

use crate::{metrics::NodeMetrics, service::AtomaP2pNodeError};

type Result<T> = std::result::Result<T, AtomaP2pNodeError>;

pub enum AtomaP2pEvent {
    /// An event emitted when a node joins the Atoma network and registers its public URL,
    /// among its peers.
    NodeMetricsRegistrationEvent {
        /// The public URL of the node.
        public_url: String,

        /// The small ID of the node.
        node_small_id: u64,

        /// The timestamp of the event.
        timestamp: u64,

        /// The country of the node.
        country: String,

        /// The metrics of the node.
        node_metrics: NodeMetrics,
    },

    /// An event emitted when a node's small ID ownership needs to be verified.
    VerifyNodeSmallIdOwnership {
        /// The small ID of the node.
        node_small_id: u64,

        /// The Sui address of the node.
        sui_address: String,
    },
}

/// Enum representing different types of messages that can be gossiped across the Atoma network.
#[derive(Debug, Serialize, Deserialize)]
pub enum GossipMessage {
    /// Message containing a signed node message
    SignedNodeMessage(SignedNodeMessage),
}

/// A message containing usage metrics for a node.
///
/// This struct represents a signed message that includes the node's small ID,
/// public URL, and a cryptographic signature to ensure the authenticity of the metrics.
#[derive(Debug, Serialize, Deserialize)]
pub struct UsageMetrics {
    /// The node small ID of the node
    pub node_small_id: u64,

    /// The public URL of the node
    pub node_public_url: String,

    /// The country where the node is located
    pub country: String,

    /// A cryptographic signature of the usage metrics
    pub signature: Vec<u8>,

    /// The timestamp of the usage metrics
    pub timestamp: u64,
}

/// Represents usage metrics collected from a node in the network
///
/// This struct contains essential information about a node's status and location
/// that is periodically published to the network for monitoring and analytics purposes.
#[derive(Debug, Serialize, Deserialize)]
pub struct NodeP2pMetadata {
    /// The public URL where the node can be accessed
    /// This is typically the endpoint that other nodes and clients use to connect
    pub node_public_url: String,

    /// A unique identifier for the node (smaller representation of the full ID)
    /// Used for quick identification and reference in the network
    pub node_small_id: u64,

    /// The country where the node is located (ISO country code)
    /// Used for geographical distribution analysis and latency optimization
    pub country: String,

    /// Unix timestamp indicating when the metrics were collected
    /// Helps track the freshness of the metrics and synchronize data across nodes
    pub timestamp: u64,
}

/// A struct containing a serialized message and its hash
pub struct SerializedMessage {
    /// The serialized message
    pub message: Vec<u8>,

    /// The hash of the serialized message
    pub hash: blake3::Hash,
}

/// A trait for serializing a message (with ciborium) and returning the hash of the serialized message
pub trait SerializeWithHash {
    /// Serialize the message and return the hash of the serialized message
    ///
    /// # Errors
    ///
    /// Returns an error if the message cannot be serialized
    /// or if the hash cannot be computed
    fn serialize_with_hash(&self) -> Result<SerializedMessage>;
}

/// A message containing usage metrics for a node.
///
/// This struct represents a signed message that includes the node's small ID,
/// public URL, and a cryptographic signature to ensure the authenticity of the metrics.
#[derive(Debug, Serialize, Deserialize)]
pub struct NodeMessage {
    /// The node metadata
    pub node_metadata: NodeP2pMetadata,

    /// The node metrics
    pub node_metrics: NodeMetrics,
}

impl SerializeWithHash for NodeMessage {
    fn serialize_with_hash(&self) -> Result<SerializedMessage> {
        let mut bytes = Vec::new();
        ciborium::into_writer(self, &mut bytes)
            .map_err(AtomaP2pNodeError::UsageMetricsSerializeError)?;
        Ok(SerializedMessage {
            hash: blake3::hash(&bytes),
            message: bytes,
        })
    }
}

/// A signed message containing a node message.
///
/// This struct represents a signed message that includes a node message and a cryptographic signature
/// to ensure the authenticity of the message.
#[derive(Debug, Serialize, Deserialize)]
pub struct SignedNodeMessage {
    /// The node message
    pub node_message: NodeMessage,

    /// The signature of the node message
    pub signature: Vec<u8>,
}

/// A trait for serializing a message (with ciborium)
///
/// This trait is used to serialize a message and return the serialized message
/// as a vector of bytes.
pub trait SerializeWithSignature {
    /// Serialize the message
    ///
    /// # Errors
    ///
    /// Returns an error if the message cannot be serialized
    fn serialize_with_signature(&self) -> Result<Vec<u8>>;
}

impl SerializeWithSignature for SignedNodeMessage {
    fn serialize_with_signature(&self) -> Result<Vec<u8>> {
        let mut bytes = Vec::new();
        ciborium::into_writer(self, &mut bytes)
            .map_err(AtomaP2pNodeError::UsageMetricsSerializeError)?;
        Ok(bytes)
    }
}
