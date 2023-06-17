use rusqlite::params;
use std::path::Path;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::{Arc, Mutex};
use tracing_forest::tree::Tree;
use tracing_subscriber::layer::SubscriberExt;

pub fn override_thread_subscriber(
    db_path: impl AsRef<Path>,
) -> anyhow::Result<(tracing::subscriber::DefaultGuard, Arc<AtomicU32>)> {
    let current_id = Arc::new(AtomicU32::default());
    let processor = tracing_forest::Printer::new().formatter(StoreTreeToDb {
        con: Arc::new(Mutex::new(rusqlite::Connection::open(&db_path)?)),
        run_id: current_id.clone(),
    });
    let subscriber = tracing_subscriber::Registry::default().with(tracing_forest::ForestLayer::from(processor));
    let guard = tracing::subscriber::set_default(subscriber);
    Ok((guard, current_id))
}

pub struct StoreTreeToDb {
    pub con: Arc<Mutex<rusqlite::Connection>>,
    pub run_id: Arc<AtomicU32>,
}

impl tracing_forest::printer::Formatter for StoreTreeToDb {
    type Error = rusqlite::Error;

    fn fmt(&self, tree: &Tree) -> Result<String, Self::Error> {
        let json = serde_json::to_string_pretty(&tree).expect("serialization to string always works");
        let run_id = self.run_id.load(Ordering::SeqCst);
        self.con
            .lock()
            .unwrap()
            .execute("UPDATE run SET spans_json = ?1 WHERE id = ?2", params![json, run_id])?;
        Ok(String::new())
    }
}
