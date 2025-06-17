use std::fmt;

#[derive(Debug, Clone)]
pub struct MemoryLocator {
    pub index: String,
    pub model: String,
    pub version: Option<String>,
}

impl fmt::Display for MemoryLocator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let version_str = self.version.as_deref().unwrap_or("latest");
        write!(f, "{}:{}:{}", self.index, self.model, version_str)
    }
}

pub fn universal_index_name(locator: &MemoryLocator, dimensions: usize) -> String {
    format!("{}:{}", locator, dimensions)
}
