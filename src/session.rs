// SPDX-License-Identifier: MIT

use crate::backend::Backend;
use std::sync::Arc;

pub struct Session {
    pub backend: Option<Arc<dyn Backend + Send + Sync>>,
    pub model: Option<String>,
    pub document: Option<String>,
}

impl Session {
    pub fn new() -> Self {
        Self {
            backend: None,
            model: None,
            document: None,
        }
    }
}