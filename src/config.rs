use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct ProjectionConfig {
    pub top: f32,
    pub bottom: f32,
    pub left: f32,
    pub right: f32,
}

impl Default for ProjectionConfig {
    fn default() -> Self {
        ProjectionConfig {
            top: 0.0,
            bottom: 100.0,
            left: 0.0,
            right: 100.0,
        }
    }
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Config {
    pub projection: ProjectionConfig
}
