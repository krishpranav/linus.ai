/**
 * @filename: types.rs
 * @author: Krisna Pranav
*/


use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReviewMode {
    Calm,
    Informative,
    Hardcore,
}

impl ReviewMode {
    pub fn as_str(&self) -> &'static str {
        match self {
            ReviewMode::Calm => "calm",
            ReviewMode::Informative => "informative",
            ReviewMode::Hardcore => "hardcore",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            ReviewMode::Calm => "Teaching tone, encouraging, suggestive",
            ReviewMode::Informative => "Structured, technical depth, prioritized",
            ReviewMode::Hardcore => "Zero tolerance, aggressive critique, flags anti-patterns",
        }
    }
}

impl std::fmt::Display for ReviewMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl std::str::FromStr for ReviewMode {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "calm" => Ok(ReviewMode::Calm),
            "informative" => Ok(ReviewMode::Informative),
            "hardcore" => Ok(ReviewMode::Hardcore),
            other => Err(anyhow::anyhow!("Unknown review mode: '{}'. Choose calm, informative, or hardcore.", other)),
        }
    }
}

/// A file entry from the scanner
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileEntry {
    pub path: String,
    pub lines: usize,
    pub language: String,
}

/// Dependency relationship between files
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DependencyEdge {
    pub from: String,
    pub to: String,
}

/// Full context passed to the LLM prompt builder
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReviewContext {
    pub total_files: usize,
    pub total_lines: usize,
    pub top_files: Vec<FileEntry>,
    pub large_files: Vec<FileEntry>,
    pub languages: Vec<(String, usize)>,
    pub circular_deps: Vec<Vec<String>>,
    pub dependency_edges: Vec<DependencyEdge>,
    pub mode: ReviewMode,
    pub model: String,
}

/// Sections of the final review output
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ReviewOutput {
    pub architecture: String,
    pub performance: String,
    pub security: String,
    pub code_smells: String,
    pub structural_improvements: String,
    pub raw: String,
}

/// Application state used by the TUI
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AppStatus {
    Idle,
    Scanning,
    BuildingGraph,
    QueryingOllama,
    Done,
    Error(String),
}

impl std::fmt::Display for AppStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppStatus::Idle => write!(f, "Ready"),
            AppStatus::Scanning => write!(f, "Scanning repository..."),
            AppStatus::BuildingGraph => write!(f, "Building dependency graph..."),
            AppStatus::QueryingOllama => write!(f, "Querying Ollama..."),
            AppStatus::Done => write!(f, "Review complete"),
            AppStatus::Error(e) => write!(f, "Error: {}", e),
        }
    }
}

/// The full application state
pub struct AppState {
    pub status: AppStatus,
    pub context: Option<ReviewContext>,
    pub output: Option<ReviewOutput>,
    pub scroll: u16,
    pub model: String,
    pub mode: ReviewMode,
    pub available_models: Vec<String>,
    pub selected_model_idx: usize,
}

impl AppState {
    pub fn new(model: String, mode: ReviewMode) -> Self {
        Self {
            status: AppStatus::Idle,
            context: None,
            output: None,
            scroll: 0,
            model,
            mode,
            available_models: Vec::new(),
            selected_model_idx: 0,
        }
    }
}