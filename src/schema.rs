use clap::ValueEnum;
use serde_json::Value;
use std::fmt;
use std::path::PathBuf;
use tokio::fs;

const SCHEMA_BASE_URL: &str = "https://raw.githubusercontent.com/Failure-Analysis-Metadata-Header/fa-metadata-schema/refs/heads";

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum SchemaVersion {
    #[value(name = "v1")]
    V1,
    #[value(name = "v2")]
    V2,
}

impl SchemaVersion {
    fn branch(&self) -> &'static str {
        match self {
            SchemaVersion::V1 => "master",
            SchemaVersion::V2 => "schema-v2",
        }
    }

    fn folder(&self) -> &'static str {
        match self {
            SchemaVersion::V1 => "v1",
            SchemaVersion::V2 => "v2",
        }
    }

    fn cache_dir_name(&self) -> &'static str {
        match self {
            SchemaVersion::V1 => "v1",
            SchemaVersion::V2 => "v2",
        }
    }

    pub fn url_for(&self, schema_type: SchemaType) -> String {
        format!(
            "{}/{}/schema/{}/{}",
            SCHEMA_BASE_URL,
            self.branch(),
            self.folder(),
            schema_type.file_name()
        )
    }
}

#[derive(Debug, Clone, Copy)]
pub enum SchemaType {
    General,
    Customer,
    Tool,
    Method,
    DataEvaluation,
    History,
}

impl SchemaType {
    fn file_name(&self) -> &'static str {
        match self {
            SchemaType::General => "generalSection.json",
            SchemaType::Customer => "customerSection.json",
            SchemaType::Tool => "toolSpecific.json",
            SchemaType::Method => "methodSpecific.json",
            SchemaType::DataEvaluation => "dataEvaluation.json",
            SchemaType::History => "historySection.json",
        }
    }

    fn label(&self) -> &'static str {
        match self {
            SchemaType::General => "general",
            SchemaType::Customer => "customer",
            SchemaType::Tool => "tool",
            SchemaType::Method => "method",
            SchemaType::DataEvaluation => "data evaluation",
            SchemaType::History => "history",
        }
    }
}

impl fmt::Display for SchemaType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.label())
    }
}

pub struct SchemaCache {
    pub general: Value,
    pub customer: Value,
    pub tool: Value,
    pub method: Value,
    pub data_evaluation: Value,
    pub history: Value,
}

impl SchemaCache {
    // Download all schemas from GitHub
    pub async fn download_all(
        version: SchemaVersion,
        use_cache: bool,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        // Try to load from cache first if caching is enabled
        if use_cache {
            if let Ok(cache) = Self::load_from_cache(version).await {
                return Ok(cache);
            }
        }

        let (general, customer, tool, method, data_evaluation, history) = tokio::join!(
            download_and_parse_schema(SchemaType::General, version),
            download_and_parse_schema(SchemaType::Customer, version),
            download_and_parse_schema(SchemaType::Tool, version),
            download_and_parse_schema(SchemaType::Method, version),
            download_and_parse_schema(SchemaType::DataEvaluation, version),
            download_and_parse_schema(SchemaType::History, version),
        );

        let cache = SchemaCache {
            general: general?,
            customer: customer?,
            tool: tool?,
            method: method?,
            data_evaluation: data_evaluation?,
            history: history?,
        };

        // Save to cache (don't fail if cache write fails)
        let _ = cache.save_to_cache(version).await;

        Ok(cache)
    }

    // Get a single schema
    pub fn get(&self, schema_type: SchemaType) -> &Value {
        match schema_type {
            SchemaType::General => &self.general,
            SchemaType::Customer => &self.customer,
            SchemaType::Tool => &self.tool,
            SchemaType::Method => &self.method,
            SchemaType::DataEvaluation => &self.data_evaluation,
            SchemaType::History => &self.history,
        }
    }

    // Load all schemas from cache directory
    async fn load_from_cache(version: SchemaVersion) -> Result<Self, Box<dyn std::error::Error>> {
        let cache_dir = get_cache_dir(version)?;

        let general = load_schema_from_file(&cache_dir, SchemaType::General).await?;
        let customer = load_schema_from_file(&cache_dir, SchemaType::Customer).await?;
        let tool = load_schema_from_file(&cache_dir, SchemaType::Tool).await?;
        let method = load_schema_from_file(&cache_dir, SchemaType::Method).await?;
        let data_evaluation = load_schema_from_file(&cache_dir, SchemaType::DataEvaluation).await?;
        let history = load_schema_from_file(&cache_dir, SchemaType::History).await?;

        Ok(SchemaCache {
            general,
            customer,
            tool,
            method,
            data_evaluation,
            history,
        })
    }

    // Save all schemas to cache directory
    async fn save_to_cache(
        &self,
        version: SchemaVersion,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let cache_dir = get_cache_dir(version)?;

        // Create cache directory if it doesn't exist
        fs::create_dir_all(&cache_dir).await.ok();

        save_schema_to_file(&cache_dir, SchemaType::General, &self.general).await?;
        save_schema_to_file(&cache_dir, SchemaType::Customer, &self.customer).await?;
        save_schema_to_file(&cache_dir, SchemaType::Tool, &self.tool).await?;
        save_schema_to_file(&cache_dir, SchemaType::Method, &self.method).await?;
        save_schema_to_file(
            &cache_dir,
            SchemaType::DataEvaluation,
            &self.data_evaluation,
        )
        .await?;
        save_schema_to_file(&cache_dir, SchemaType::History, &self.history).await?;

        Ok(())
    }
}

async fn download_schema(
    schema_type: SchemaType,
    version: SchemaVersion,
) -> Result<String, Box<dyn std::error::Error>> {
    let url = version.url_for(schema_type);
    let client = reqwest::Client::new();
    let response = client.get(&url).send().await.map_err(|e| {
        format!(
            "Failed to download {} schema from {}: {}",
            schema_type, url, e
        )
    })?;
    let response = response.error_for_status().map_err(|e| {
        format!(
            "{} schema endpoint {} returned an error response: {}",
            schema_type, url, e
        )
    })?;
    response.text().await.map_err(|e| {
        format!(
            "Failed to read {} schema body from {}: {}",
            schema_type, url, e
        )
        .into()
    })
}

pub fn parse_schema(
    schema_type: SchemaType,
    schema_text: &str,
    version: SchemaVersion,
) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let url = version.url_for(schema_type);
    serde_json::from_str(schema_text).map_err(|e| {
        format!(
            "Failed to parse {} schema downloaded from {}: {}",
            schema_type, url, e
        )
        .into()
    })
}

async fn download_and_parse_schema(
    schema_type: SchemaType,
    version: SchemaVersion,
) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let schema_text = download_schema(schema_type, version).await?;
    let schema = parse_schema(schema_type, &schema_text, version)?;
    Ok(schema)
}

// Get the cache directory for schemas
fn get_cache_dir(version: SchemaVersion) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let cache_dir = dirs::cache_dir()
        .ok_or("Could not determine cache directory")?
        .join("famdo")
        .join("schemas")
        .join(version.cache_dir_name());
    Ok(cache_dir)
}

// Load a single schema from cache file
async fn load_schema_from_file(
    cache_dir: &PathBuf,
    schema_type: SchemaType,
) -> Result<Value, Box<dyn std::error::Error>> {
    let file_path = cache_dir.join(schema_type.file_name());
    let content = fs::read_to_string(&file_path).await?;
    let schema = serde_json::from_str(&content)?;
    Ok(schema)
}

// Save a single schema to cache file
async fn save_schema_to_file(
    cache_dir: &PathBuf,
    schema_type: SchemaType,
    schema: &Value,
) -> Result<(), Box<dyn std::error::Error>> {
    let file_path = cache_dir.join(schema_type.file_name());
    let content = serde_json::to_string(schema)?;
    fs::write(&file_path, content).await?;
    Ok(())
}
