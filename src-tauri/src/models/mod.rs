use serde::{Serialize, Deserialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Track {
    pub id: String,
    pub title: String,
    pub artist: String,
    pub genre: String,
    pub source_path: PathBuf,
    pub filename: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrganizationResult {
    pub total_files: usize,
    pub organized_files: usize,
    pub failed_files: usize,
    pub duration_ms: u128,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProgressUpdate {
    pub processed: usize,
    pub total: usize,
    pub current_file: String,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MusicBrainzResult {
    pub title: String,
    pub artist: String,
    pub genre: Option<String>,
}
