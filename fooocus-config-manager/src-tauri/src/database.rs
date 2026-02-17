use rusqlite::{Connection, Result as SqliteResult};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use std::path::PathBuf;

pub struct Database(pub Mutex<Connection>);

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoRA {
    pub name: String,
    pub model_name: String,
    pub weight: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModelConfig {
    pub base_model: String,
    pub refiner_model: String,
    pub refiner_switch: f64,
    pub loras: Vec<LoRA>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SamplingConfig {
    pub cfg_scale: f64,
    pub sample_sharpness: f64,
    pub sampler: String,
    pub scheduler: String,
    pub performance: String,
    pub steps: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PromptConfig {
    pub positive: String,
    pub negative: String,
    pub styles: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImageConfig {
    pub aspect_ratio: String,
    pub image_count: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResourceDownloads {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checkpoint_downloads: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lora_downloads: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embedding_downloads: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PresetConfig {
    pub id: String,
    pub name: String,
    pub description: String,
    pub tags: Vec<String>,
    pub is_favorite: bool,
    pub use_count: i32,
    pub created_at: String,
    pub updated_at: String,
    pub model: ModelConfig,
    pub sampling: SamplingConfig,
    pub prompt: PromptConfig,
    pub image: ImageConfig,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resources: Option<ResourceDownloads>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tag {
    pub id: String,
    pub name: String,
    pub color: String,
    pub count: i32,
}

impl Database {
    pub fn new(app_data_dir: PathBuf) -> SqliteResult<Self> {
        std::fs::create_dir_all(&app_data_dir).ok();
        let db_path = app_data_dir.join("fooocus_config.db");
        let conn = Connection::open(db_path)?;
        Self::init_tables(&conn)?;
        Ok(Database(Mutex::new(conn)))
    }

    fn init_tables(conn: &Connection) -> SqliteResult<()> {
        conn.execute_batch(
            r#"
            CREATE TABLE IF NOT EXISTS presets (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                description TEXT,
                tags TEXT,
                is_favorite INTEGER DEFAULT 0,
                use_count INTEGER DEFAULT 0,
                created_at TEXT,
                updated_at TEXT,
                model_config TEXT,
                sampling_config TEXT,
                prompt_config TEXT,
                image_config TEXT,
                resources TEXT
            );

            CREATE TABLE IF NOT EXISTS tags (
                id TEXT PRIMARY KEY,
                name TEXT UNIQUE NOT NULL,
                color TEXT DEFAULT '#6366f1'
            );

            CREATE INDEX IF NOT EXISTS idx_presets_name ON presets(name);
            CREATE INDEX IF NOT EXISTS idx_presets_created_at ON presets(created_at);
            CREATE INDEX IF NOT EXISTS idx_presets_is_favorite ON presets(is_favorite);
            "#,
        )?;
        Ok(())
    }
}
