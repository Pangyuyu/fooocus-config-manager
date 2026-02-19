use crate::database::{Database, PresetConfig, Tag, ModelConfig, SamplingConfig, PromptConfig, ImageConfig, ModelInfo, ModelUsageInfo};
use tauri::State;
use rusqlite::params;
use serde_json;
use uuid::Uuid;
use chrono::Utc;

#[tauri::command]
pub fn get_all_presets(db: State<'_, Database>) -> Result<Vec<PresetConfig>, String> {
    let conn = db.0.lock().map_err(|e| e.to_string())?;
    let mut stmt = conn.prepare(
        "SELECT id, name, description, tags, is_favorite, use_count, created_at, updated_at, 
                model_config, sampling_config, prompt_config, image_config, resources 
         FROM presets ORDER BY updated_at DESC"
    ).map_err(|e| e.to_string())?;

    let presets = stmt.query_map([], |row| {
        Ok(PresetConfig {
            id: row.get(0)?,
            name: row.get(1)?,
            description: row.get(2)?,
            tags: serde_json::from_str(&row.get::<_, String>(3)?).unwrap_or_default(),
            is_favorite: row.get::<_, i32>(4)? != 0,
            use_count: row.get(5)?,
            created_at: row.get(6)?,
            updated_at: row.get(7)?,
            model: serde_json::from_str(&row.get::<_, String>(8)?).unwrap_or_else(|_| ModelConfig {
                base_model: String::new(),
                base_model_id: None,
                refiner_model: String::new(),
                refiner_model_id: None,
                refiner_switch: 0.5,
                loras: vec![],
            }),
            sampling: serde_json::from_str(&row.get::<_, String>(9)?).unwrap_or_else(|_| SamplingConfig {
                cfg_scale: 7.0,
                sample_sharpness: 2.0,
                sampler: String::from("dpmpp_2m_sde_gpu"),
                scheduler: String::from("karras"),
                performance: String::from("Speed"),
                steps: 30,
            }),
            prompt: serde_json::from_str(&row.get::<_, String>(10)?).unwrap_or_else(|_| PromptConfig {
                positive: String::new(),
                negative: String::new(),
                styles: vec![],
            }),
            image: serde_json::from_str(&row.get::<_, String>(11)?).unwrap_or_else(|_| ImageConfig {
                aspect_ratio: String::from("1152*896"),
                image_count: 4,
            }),
            resources: row.get::<_, Option<String>>(12)?.and_then(|s| serde_json::from_str(&s).ok()),
        })
    }).map_err(|e| e.to_string())?;

    presets.collect::<Result<Vec<_>, _>>().map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_preset_by_id(db: State<'_, Database>, id: String) -> Result<Option<PresetConfig>, String> {
    let conn = db.0.lock().map_err(|e| e.to_string())?;
    let mut stmt = conn.prepare(
        "SELECT id, name, description, tags, is_favorite, use_count, created_at, updated_at, 
                model_config, sampling_config, prompt_config, image_config, resources 
         FROM presets WHERE id = ?1"
    ).map_err(|e| e.to_string())?;

    let result = stmt.query_row(params![id], |row| {
        Ok(PresetConfig {
            id: row.get(0)?,
            name: row.get(1)?,
            description: row.get(2)?,
            tags: serde_json::from_str(&row.get::<_, String>(3)?).unwrap_or_default(),
            is_favorite: row.get::<_, i32>(4)? != 0,
            use_count: row.get(5)?,
            created_at: row.get(6)?,
            updated_at: row.get(7)?,
            model: serde_json::from_str(&row.get::<_, String>(8)?).unwrap_or_else(|_| ModelConfig {
                base_model: String::new(),
                base_model_id: None,
                refiner_model: String::new(),
                refiner_model_id: None,
                refiner_switch: 0.5,
                loras: vec![],
            }),
            sampling: serde_json::from_str(&row.get::<_, String>(9)?).unwrap_or_else(|_| SamplingConfig {
                cfg_scale: 7.0,
                sample_sharpness: 2.0,
                sampler: String::from("dpmpp_2m_sde_gpu"),
                scheduler: String::from("karras"),
                performance: String::from("Speed"),
                steps: 30,
            }),
            prompt: serde_json::from_str(&row.get::<_, String>(10)?).unwrap_or_else(|_| PromptConfig {
                positive: String::new(),
                negative: String::new(),
                styles: vec![],
            }),
            image: serde_json::from_str(&row.get::<_, String>(11)?).unwrap_or_else(|_| ImageConfig {
                aspect_ratio: String::from("1152*896"),
                image_count: 4,
            }),
            resources: row.get::<_, Option<String>>(12)?.and_then(|s| serde_json::from_str(&s).ok()),
        })
    });

    match result {
        Ok(preset) => Ok(Some(preset)),
        Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
pub fn create_preset(db: State<'_, Database>, preset: PresetConfig) -> Result<PresetConfig, String> {
    let conn = db.0.lock().map_err(|e| e.to_string())?;
    let id = Uuid::new_v4().to_string();
    let now = Utc::now().to_rfc3339();
    
    let model_json = serde_json::to_string(&preset.model).map_err(|e| e.to_string())?;
    let sampling_json = serde_json::to_string(&preset.sampling).map_err(|e| e.to_string())?;
    let prompt_json = serde_json::to_string(&preset.prompt).map_err(|e| e.to_string())?;
    let image_json = serde_json::to_string(&preset.image).map_err(|e| e.to_string())?;
    let tags_json = serde_json::to_string(&preset.tags).map_err(|e| e.to_string())?;
    let resources_json = preset.resources.as_ref()
        .map(|r| serde_json::to_string(r).unwrap_or_default());

    conn.execute(
        "INSERT INTO presets (id, name, description, tags, is_favorite, use_count, created_at, updated_at, 
         model_config, sampling_config, prompt_config, image_config, resources)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13)",
        params![
            id,
            preset.name,
            preset.description,
            tags_json,
            preset.is_favorite as i32,
            0,
            now,
            now,
            model_json,
            sampling_json,
            prompt_json,
            image_json,
            resources_json,
        ],
    ).map_err(|e| e.to_string())?;

    Ok(PresetConfig {
        id,
        name: preset.name,
        description: preset.description,
        tags: preset.tags,
        is_favorite: preset.is_favorite,
        use_count: 0,
        created_at: now.clone(),
        updated_at: now,
        model: preset.model,
        sampling: preset.sampling,
        prompt: preset.prompt,
        image: preset.image,
        resources: preset.resources,
    })
}

#[tauri::command]
pub fn update_preset(db: State<'_, Database>, preset: PresetConfig) -> Result<PresetConfig, String> {
    let conn = db.0.lock().map_err(|e| e.to_string())?;
    let now = Utc::now().to_rfc3339();
    
    let model_json = serde_json::to_string(&preset.model).map_err(|e| e.to_string())?;
    let sampling_json = serde_json::to_string(&preset.sampling).map_err(|e| e.to_string())?;
    let prompt_json = serde_json::to_string(&preset.prompt).map_err(|e| e.to_string())?;
    let image_json = serde_json::to_string(&preset.image).map_err(|e| e.to_string())?;
    let tags_json = serde_json::to_string(&preset.tags).map_err(|e| e.to_string())?;
    let resources_json = preset.resources.as_ref()
        .map(|r| serde_json::to_string(r).unwrap_or_default());

    conn.execute(
        "UPDATE presets SET name = ?1, description = ?2, tags = ?3, is_favorite = ?4, 
         updated_at = ?5, model_config = ?6, sampling_config = ?7, prompt_config = ?8, 
         image_config = ?9, resources = ?10 WHERE id = ?11",
        params![
            preset.name,
            preset.description,
            tags_json,
            preset.is_favorite as i32,
            now,
            model_json,
            sampling_json,
            prompt_json,
            image_json,
            resources_json,
            preset.id,
        ],
    ).map_err(|e| e.to_string())?;

    Ok(PresetConfig {
        updated_at: now,
        ..preset
    })
}

#[tauri::command]
pub fn delete_preset(db: State<'_, Database>, id: String) -> Result<(), String> {
    let conn = db.0.lock().map_err(|e| e.to_string())?;
    conn.execute("DELETE FROM presets WHERE id = ?1", params![id]).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub fn search_presets(db: State<'_, Database>, query: String) -> Result<Vec<PresetConfig>, String> {
    let conn = db.0.lock().map_err(|e| e.to_string())?;
    let search_pattern = format!("%{}%", query);
    
    let mut stmt = conn.prepare(
        "SELECT id, name, description, tags, is_favorite, use_count, created_at, updated_at, 
                model_config, sampling_config, prompt_config, image_config, resources 
         FROM presets WHERE name LIKE ?1 OR description LIKE ?1 OR tags LIKE ?1
         ORDER BY updated_at DESC"
    ).map_err(|e| e.to_string())?;

    let presets = stmt.query_map(params![search_pattern], |row| {
        Ok(PresetConfig {
            id: row.get(0)?,
            name: row.get(1)?,
            description: row.get(2)?,
            tags: serde_json::from_str(&row.get::<_, String>(3)?).unwrap_or_default(),
            is_favorite: row.get::<_, i32>(4)? != 0,
            use_count: row.get(5)?,
            created_at: row.get(6)?,
            updated_at: row.get(7)?,
            model: serde_json::from_str(&row.get::<_, String>(8)?).unwrap_or_else(|_| ModelConfig {
                base_model: String::new(),
                base_model_id: None,
                refiner_model: String::new(),
                refiner_model_id: None,
                refiner_switch: 0.5,
                loras: vec![],
            }),
            sampling: serde_json::from_str(&row.get::<_, String>(9)?).unwrap_or_else(|_| SamplingConfig {
                cfg_scale: 7.0,
                sample_sharpness: 2.0,
                sampler: String::from("dpmpp_2m_sde_gpu"),
                scheduler: String::from("karras"),
                performance: String::from("Speed"),
                steps: 30,
            }),
            prompt: serde_json::from_str(&row.get::<_, String>(10)?).unwrap_or_else(|_| PromptConfig {
                positive: String::new(),
                negative: String::new(),
                styles: vec![],
            }),
            image: serde_json::from_str(&row.get::<_, String>(11)?).unwrap_or_else(|_| ImageConfig {
                aspect_ratio: String::from("1152*896"),
                image_count: 4,
            }),
            resources: row.get::<_, Option<String>>(12)?.and_then(|s| serde_json::from_str(&s).ok()),
        })
    }).map_err(|e| e.to_string())?;

    presets.collect::<Result<Vec<_>, _>>().map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_all_tags(db: State<'_, Database>) -> Result<Vec<Tag>, String> {
    let conn = db.0.lock().map_err(|e| e.to_string())?;
    let mut stmt = conn.prepare(
        "SELECT t.id, t.name, t.color, COUNT(p.id) as count 
         FROM tags t LEFT JOIN presets p ON ',' || p.tags || ',' LIKE '%,' || t.name || ',%' 
         GROUP BY t.id ORDER BY t.name"
    ).map_err(|e| e.to_string())?;

    let tags = stmt.query_map([], |row| {
        Ok(Tag {
            id: row.get(0)?,
            name: row.get(1)?,
            color: row.get(2)?,
            count: row.get(3)?,
        })
    }).map_err(|e| e.to_string())?;

    tags.collect::<Result<Vec<_>, _>>().map_err(|e| e.to_string())
}

#[tauri::command]
pub fn create_tag(db: State<'_, Database>, name: String, color: String) -> Result<Tag, String> {
    let conn = db.0.lock().map_err(|e| e.to_string())?;
    let id = Uuid::new_v4().to_string();
    
    conn.execute(
        "INSERT INTO tags (id, name, color) VALUES (?1, ?2, ?3)",
        params![id, name, color],
    ).map_err(|e| e.to_string())?;

    Ok(Tag {
        id,
        name,
        color,
        count: 0,
    })
}

#[tauri::command]
pub fn delete_tag(db: State<'_, Database>, id: String) -> Result<(), String> {
    let conn = db.0.lock().map_err(|e| e.to_string())?;
    conn.execute("DELETE FROM tags WHERE id = ?1", params![id]).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub fn toggle_favorite(db: State<'_, Database>, id: String) -> Result<(), String> {
    let conn = db.0.lock().map_err(|e| e.to_string())?;
    conn.execute(
        "UPDATE presets SET is_favorite = NOT is_favorite, updated_at = ?1 WHERE id = ?2",
        params![Utc::now().to_rfc3339(), id],
    ).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub fn increment_use_count(db: State<'_, Database>, id: String) -> Result<(), String> {
    let conn = db.0.lock().map_err(|e| e.to_string())?;
    conn.execute(
        "UPDATE presets SET use_count = use_count + 1, updated_at = ?1 WHERE id = ?2",
        params![Utc::now().to_rfc3339(), id],
    ).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub fn get_all_models(db: State<'_, Database>) -> Result<Vec<ModelInfo>, String> {
    let conn = db.0.lock().map_err(|e| e.to_string())?;
    let mut stmt = conn.prepare(
        "SELECT id, name, file_name, model_type, description, scope, path, tags, created_at, updated_at 
         FROM models ORDER BY updated_at DESC"
    ).map_err(|e| e.to_string())?;

    let models = stmt.query_map([], |row| {
        Ok(ModelInfo {
            id: row.get(0)?,
            name: row.get(1)?,
            file_name: row.get(2)?,
            model_type: row.get(3)?,
            description: row.get(4)?,
            scope: serde_json::from_str(&row.get::<_, String>(5)?).unwrap_or_default(),
            path: row.get(6)?,
            tags: serde_json::from_str(&row.get::<_, String>(7)?).unwrap_or_default(),
            created_at: row.get(8)?,
            updated_at: row.get(9)?,
        })
    }).map_err(|e| e.to_string())?;

    models.collect::<Result<Vec<_>, _>>().map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_presets_by_model_id(db: State<'_, Database>, model_id: String) -> Result<Vec<PresetConfig>, String> {
    let conn = db.0.lock().map_err(|e| e.to_string())?;
    
    let mut stmt = conn.prepare(
        "SELECT id, name, description, tags, is_favorite, use_count, created_at, updated_at, 
                model_config, sampling_config, prompt_config, image_config, resources 
         FROM presets ORDER BY updated_at DESC"
    ).map_err(|e| e.to_string())?;

    let presets = stmt.query_map([], |row| {
        Ok(PresetConfig {
            id: row.get(0)?,
            name: row.get(1)?,
            description: row.get(2)?,
            tags: serde_json::from_str(&row.get::<_, String>(3)?).unwrap_or_default(),
            is_favorite: row.get::<_, i32>(4)? != 0,
            use_count: row.get(5)?,
            created_at: row.get(6)?,
            updated_at: row.get(7)?,
            model: serde_json::from_str(&row.get::<_, String>(8)?).unwrap_or_else(|_| ModelConfig {
                base_model: String::new(),
                base_model_id: None,
                refiner_model: String::new(),
                refiner_model_id: None,
                refiner_switch: 0.5,
                loras: vec![],
            }),
            sampling: serde_json::from_str(&row.get::<_, String>(9)?).unwrap_or_else(|_| SamplingConfig {
                cfg_scale: 7.0,
                sample_sharpness: 2.0,
                sampler: String::from("dpmpp_2m_sde_gpu"),
                scheduler: String::from("karras"),
                performance: String::from("Speed"),
                steps: 30,
            }),
            prompt: serde_json::from_str(&row.get::<_, String>(10)?).unwrap_or_else(|_| PromptConfig {
                positive: String::new(),
                negative: String::new(),
                styles: vec![],
            }),
            image: serde_json::from_str(&row.get::<_, String>(11)?).unwrap_or_else(|_| ImageConfig {
                aspect_ratio: String::from("1152*896"),
                image_count: 4,
            }),
            resources: row.get::<_, Option<String>>(12)?.and_then(|s| serde_json::from_str(&s).ok()),
        })
    }).map_err(|e| e.to_string())?;

    let all_presets: Vec<PresetConfig> = presets.collect::<Result<Vec<_>, _>>().map_err(|e| e.to_string())?;
    
    let filtered: Vec<PresetConfig> = all_presets.into_iter().filter(|p| {
        if p.model.base_model_id.as_deref() == Some(model_id.as_str()) {
            return true;
        }
        if p.model.refiner_model_id.as_deref() == Some(model_id.as_str()) {
            return true;
        }
        for lora in &p.model.loras {
            if lora.model_id.as_deref() == Some(model_id.as_str()) {
                return true;
            }
        }
        false
    }).collect();

    Ok(filtered)
}

#[tauri::command]
pub fn check_model_usage(db: State<'_, Database>, model_id: String) -> Result<ModelUsageInfo, String> {
    let presets = get_presets_by_model_id(db, model_id)?;
    
    let preset_names: Vec<String> = presets.iter().map(|p| p.name.clone()).collect();
    let usage_count = preset_names.len() as i32;
    
    Ok(ModelUsageInfo {
        is_used: usage_count > 0,
        usage_count,
        preset_names,
    })
}

#[tauri::command]
pub fn get_models_by_type(db: State<'_, Database>, model_type: String) -> Result<Vec<ModelInfo>, String> {
    let conn = db.0.lock().map_err(|e| e.to_string())?;
    let mut stmt = conn.prepare(
        "SELECT id, name, file_name, model_type, description, scope, path, tags, created_at, updated_at 
         FROM models WHERE model_type = ?1 ORDER BY updated_at DESC"
    ).map_err(|e| e.to_string())?;

    let models = stmt.query_map(params![model_type], |row| {
        Ok(ModelInfo {
            id: row.get(0)?,
            name: row.get(1)?,
            file_name: row.get(2)?,
            model_type: row.get(3)?,
            description: row.get(4)?,
            scope: serde_json::from_str(&row.get::<_, String>(5)?).unwrap_or_default(),
            path: row.get(6)?,
            tags: serde_json::from_str(&row.get::<_, String>(7)?).unwrap_or_default(),
            created_at: row.get(8)?,
            updated_at: row.get(9)?,
        })
    }).map_err(|e| e.to_string())?;

    models.collect::<Result<Vec<_>, _>>().map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_model_by_id(db: State<'_, Database>, id: String) -> Result<Option<ModelInfo>, String> {
    let conn = db.0.lock().map_err(|e| e.to_string())?;
    let mut stmt = conn.prepare(
        "SELECT id, name, file_name, model_type, description, scope, path, tags, created_at, updated_at 
         FROM models WHERE id = ?1"
    ).map_err(|e| e.to_string())?;

    let result = stmt.query_row(params![id], |row| {
        Ok(ModelInfo {
            id: row.get(0)?,
            name: row.get(1)?,
            file_name: row.get(2)?,
            model_type: row.get(3)?,
            description: row.get(4)?,
            scope: serde_json::from_str(&row.get::<_, String>(5)?).unwrap_or_default(),
            path: row.get(6)?,
            tags: serde_json::from_str(&row.get::<_, String>(7)?).unwrap_or_default(),
            created_at: row.get(8)?,
            updated_at: row.get(9)?,
        })
    });

    match result {
        Ok(model) => Ok(Some(model)),
        Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
pub fn create_model(db: State<'_, Database>, model: ModelInfo) -> Result<ModelInfo, String> {
    let conn = db.0.lock().map_err(|e| e.to_string())?;
    let id = Uuid::new_v4().to_string();
    let now = Utc::now().to_rfc3339();
    
    let scope_json = serde_json::to_string(&model.scope).map_err(|e| e.to_string())?;
    let tags_json = serde_json::to_string(&model.tags).map_err(|e| e.to_string())?;

    conn.execute(
        "INSERT INTO models (id, name, file_name, model_type, description, scope, path, tags, created_at, updated_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
        params![
            id,
            model.name,
            model.file_name,
            model.model_type,
            model.description,
            scope_json,
            model.path,
            tags_json,
            now,
            now,
        ],
    ).map_err(|e| e.to_string())?;

    Ok(ModelInfo {
        id,
        name: model.name,
        file_name: model.file_name,
        model_type: model.model_type,
        description: model.description,
        scope: model.scope,
        path: model.path,
        tags: model.tags,
        created_at: now.clone(),
        updated_at: now,
    })
}

#[tauri::command]
pub fn update_model(db: State<'_, Database>, model: ModelInfo) -> Result<ModelInfo, String> {
    let conn = db.0.lock().map_err(|e| e.to_string())?;
    let now = Utc::now().to_rfc3339();
    
    let scope_json = serde_json::to_string(&model.scope).map_err(|e| e.to_string())?;
    let tags_json = serde_json::to_string(&model.tags).map_err(|e| e.to_string())?;

    conn.execute(
        "UPDATE models SET name = ?1, file_name = ?2, model_type = ?3, description = ?4, 
         scope = ?5, path = ?6, tags = ?7, updated_at = ?8 WHERE id = ?9",
        params![
            model.name,
            model.file_name,
            model.model_type,
            model.description,
            scope_json,
            model.path,
            tags_json,
            now,
            model.id,
        ],
    ).map_err(|e| e.to_string())?;

    Ok(ModelInfo {
        updated_at: now,
        ..model
    })
}

#[tauri::command]
pub fn delete_model(db: State<'_, Database>, id: String) -> Result<(), String> {
    let conn = db.0.lock().map_err(|e| e.to_string())?;
    conn.execute("DELETE FROM models WHERE id = ?1", params![id]).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub fn search_models(db: State<'_, Database>, query: String) -> Result<Vec<ModelInfo>, String> {
    let conn = db.0.lock().map_err(|e| e.to_string())?;
    let search_pattern = format!("%{}%", query);
    
    let mut stmt = conn.prepare(
        "SELECT id, name, file_name, model_type, description, scope, path, tags, created_at, updated_at 
         FROM models WHERE name LIKE ?1 OR description LIKE ?1 OR scope LIKE ?1 OR tags LIKE ?1
         ORDER BY updated_at DESC"
    ).map_err(|e| e.to_string())?;

    let models = stmt.query_map(params![search_pattern], |row| {
        Ok(ModelInfo {
            id: row.get(0)?,
            name: row.get(1)?,
            file_name: row.get(2)?,
            model_type: row.get(3)?,
            description: row.get(4)?,
            scope: serde_json::from_str(&row.get::<_, String>(5)?).unwrap_or_default(),
            path: row.get(6)?,
            tags: serde_json::from_str(&row.get::<_, String>(7)?).unwrap_or_default(),
            created_at: row.get(8)?,
            updated_at: row.get(9)?,
        })
    }).map_err(|e| e.to_string())?;

    models.collect::<Result<Vec<_>, _>>().map_err(|e| e.to_string())
}
