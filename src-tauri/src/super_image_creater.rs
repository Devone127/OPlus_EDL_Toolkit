use serde::Deserialize;
use std::fs::File;
use std::path::Path;
use thiserror::Error;

// ======================== 1. Custom Error Type (Optional, improves error handling) ========================
#[derive(Error, Debug)]
pub enum JsonParseError {
    #[error("File operation error: {0}")]
    FileError(#[from] std::io::Error),
    #[error("JSON parsing error: {0}")]
    JsonError(#[from] serde_json::Error),
}

// ======================== 2. Define Structs Matching JSON Structure ========================
/// Top-level struct corresponding to the entire JSON configuration
#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")] // Ensure field names match JSON's snake_case convention
pub struct PartitionConfig {
    pub super_meta: SuperMeta,
    pub nv_text: String,
    pub block_devices: Vec<BlockDevice>,
    pub groups: Vec<Group>,
    pub nv_id: String,
    pub partitions: Vec<Partition>,
}

/// Struct for the "super_meta" sub-object in JSON
#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SuperMeta {
    pub path: String,
    pub size: String, // Size stored as string (JSON uses string-encoded numbers; convert later if needed)
}

/// Struct for elements in the "block_devices" array
#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct BlockDevice {
    pub block_size: String,
    pub name: String,
    pub alignment: String,
    pub size: String,
}

/// Struct for elements in the "groups" array (maximum_size is optional)
#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Group {
    pub name: String,
    #[serde(default)] // Assign empty string if field is missing in JSON
    pub maximum_size: String,
}

/// Struct for elements in the "partitions" array (path/size are optional)
#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Partition {
    pub is_dynamic: bool,
    pub name: String,
    pub group_name: String,
    #[serde(default)] // Optional field: empty string if missing
    pub path: String,
    #[serde(default)] // Optional field: empty string if missing
    pub size: String,
}

// ======================== 3. Core Function: Read JSON and Parse to Struct ========================
/// Reads a JSON file from the specified path and parses it into a PartitionConfig struct
/// 
/// # Arguments
/// * `path` - Path to the JSON configuration file (e.g., "partition_config.json")
/// 
/// # Returns
/// * `Ok(PartitionConfig)` - Successfully parsed configuration
/// * `Err(JsonParseError)` - Failed to open file or parse JSON
pub fn read_partition_config<P: AsRef<Path>>(path: P) -> Result<PartitionConfig, JsonParseError> {
    // 1. Open the JSON file
    let file = File::open(path)?;

    // 2. Parse JSON from file stream into PartitionConfig struct (serde_json auto-maps fields)
    let config = serde_json::from_reader(file)?;

    // 3. Return parsed configuration
    Ok(config)
}
