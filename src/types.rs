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
      ReviewMode::Calm => "A calm review mode that provides gentle feedback and encouragement.",
      ReviewMode::Informative => "An informative review mode that offers detailed feedback and suggestions for improvement.",
      ReviewMode::Hardcore => "A hardcore review mode that delivers direct and critical feedback for maximum improvement.",
    }
  }
}
