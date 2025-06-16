use std::collections::HashMap;
use std::fmt;

#[derive(Debug, Clone)]
pub struct MemoryLocator {
    pub index: String,
    pub model: String,
    pub version: Option<String>,
}

impl fmt::Display for MemoryLocator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Handle the Option<String> for 'version'
        let version_str = match &self.version {
            Some(v) => v.as_str(), // Get a string slice from the Option<String>
            None => "latest",     // This should be in a global spot of constant strings...?
        };

        // Write the formatted string to the formatter
        write!(f, "{}:{}:{}", self.index, self.model, version_str)
    }
}

#[derive(Debug, Clone)]
pub struct MemoryEntry {
    pub id: String,
    pub embedding: Vec<f32>,
    pub text: String,
    pub metadata: HashMap<String, String>,
}

pub struct AddEntryParams {
    pub locator: MemoryLocator,
    pub entry: MemoryEntry,
}

pub struct QueryParams {
    pub locator: MemoryLocator,
    pub embedding: Vec<f32>,
    pub top_k: usize,
    pub filters: Option<HashMap<String, String>>,
}

pub struct IndexParams {
    pub name: String,
    pub model: String,
    pub dimensions: i32
}

pub fn universal_index_name(locator: &MemoryLocator, dimensions: usize) -> String {
    format!(
        "{}:{}",
        locator, dimensions
    )
}
