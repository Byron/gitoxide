use crate::progress::{MessageLevel, Progress};
use std::time::Duration;

pub struct LogProgress {
    name: String,
    max: Option<u32>,
    unit: Option<&'static str>,
    last_set: Option<std::time::SystemTime>,
}

const EMIT_LOG_EVERY_S: f32 = 0.5;

impl LogProgress {
    pub fn new(name: impl Into<String>) -> Self {
        LogProgress {
            name: name.into(),
            max: None,
            unit: None,
            last_set: None,
        }
    }
}

impl Progress for LogProgress {
    type SubProgress = LogProgress;

    fn add_child(&mut self, name: impl Into<String>) -> Self::SubProgress {
        LogProgress::new(format!("{}::{}", self.name, Into::<String>::into(name)))
    }

    fn init(&mut self, max: Option<u32>, unit: Option<&'static str>) {
        self.max = max;
        self.unit = unit;
    }

    fn set(&mut self, step: u32) {
        let now = std::time::SystemTime::now();
        let last = self.last_set.unwrap_or(now);
        if now
            .duration_since(last)
            .unwrap_or_else(|_| Duration::default())
            .as_secs_f32()
            > EMIT_LOG_EVERY_S
        {
            match (self.max, self.unit) {
                (Some(max), Some(unit)) => {
                    log::info!("{} → {} / {} {}", self.name, step, max, unit)
                }
                (None, Some(unit)) => log::info!("{} → {} {}", self.name, step, unit),
                (Some(max), None) => log::info!("{} → {} / {}", self.name, step, max),
                (None, None) => log::info!("{} → {}", self.name, step),
            }
        }
        self.last_set = Some(now);
    }

    fn message(&mut self, level: MessageLevel, message: impl Into<String>) {
        let message: String = message.into();
        match level {
            MessageLevel::Info => log::info!("ℹ{} → {}", self.name, message),
            MessageLevel::Failure => log::error!("𐄂{} → {}", self.name, message),
            MessageLevel::Success => log::info!("✓{} → {}", self.name, message),
        }
    }
}
