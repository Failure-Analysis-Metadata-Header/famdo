use clap::ValueEnum;
use serde_json::Value;
use std::fmt;
use std::path::{Path, PathBuf};
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
            SchemaVersion::V2 => "master",
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
}

// Trait for schema type operations
trait SchemaTypeTrait: Copy {
    fn file_name(&self) -> &'static str;
    fn label(&self) -> &'static str;
    fn url_for(&self, version: SchemaVersion) -> String {
        format!(
            "{}/{}/schema/{}/{}",
            SCHEMA_BASE_URL,
            version.branch(),
            version.folder(),
            self.file_name()
        )
    }
}

#[derive(Debug, Clone, Copy)]
pub enum V1SchemaType {
    General,
    Customer,
    Tool,
    Method,
    DataEvaluation,
    History,
}

impl SchemaTypeTrait for V1SchemaType {
    fn file_name(&self) -> &'static str {
        match self {
            V1SchemaType::General => "General Section.json",
            V1SchemaType::Customer => "Customer Section.json",
            V1SchemaType::Tool => "Tool Specific.json",
            V1SchemaType::Method => "Method Specific.json",
            V1SchemaType::DataEvaluation => "Data Evaluation.json",
            V1SchemaType::History => "History.json",
        }
    }

    fn label(&self) -> &'static str {
        match self {
            V1SchemaType::General => "General Section",
            V1SchemaType::Customer => "Customer Section",
            V1SchemaType::Tool => "Tool Specific",
            V1SchemaType::Method => "Method Specific",
            V1SchemaType::DataEvaluation => "Data Evaluation",
            V1SchemaType::History => "History",
        }
    }
}

impl V1SchemaType {
    pub fn all() -> &'static [V1SchemaType] {
        &[
            V1SchemaType::General,
            V1SchemaType::Customer,
            V1SchemaType::Tool,
            V1SchemaType::Method,
            V1SchemaType::DataEvaluation,
            V1SchemaType::History,
        ]
    }
}

#[derive(Debug, Clone, Copy)]
pub enum V2SchemaType {
    General,
    Customer,
    Tool,
    Method,
    DataEvaluation,
    History,
}

impl SchemaTypeTrait for V2SchemaType {
    fn file_name(&self) -> &'static str {
        match self {
            V2SchemaType::General => "generalSection.json",
            V2SchemaType::Customer => "customerSection.json",
            V2SchemaType::Tool => "toolSpecific.json",
            V2SchemaType::Method => "methodSpecific.json",
            V2SchemaType::DataEvaluation => "dataEvaluation.json",
            V2SchemaType::History => "historySection.json",
        }
    }

    fn label(&self) -> &'static str {
        match self {
            V2SchemaType::General => "general",
            V2SchemaType::Customer => "customer",
            V2SchemaType::Tool => "tool",
            V2SchemaType::Method => "method",
            V2SchemaType::DataEvaluation => "data evaluation",
            V2SchemaType::History => "history",
        }
    }
}

impl V2SchemaType {
    pub fn all() -> &'static [V2SchemaType] {
        &[
            V2SchemaType::General,
            V2SchemaType::Customer,
            V2SchemaType::Tool,
            V2SchemaType::Method,
            V2SchemaType::DataEvaluation,
            V2SchemaType::History,
        ]
    }
}

impl fmt::Display for V1SchemaType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.label())
    }
}

impl fmt::Display for V2SchemaType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.label())
    }
}

// V1-specific schema cache
pub struct V1SchemaCache {
    pub general: Value,
    pub customer: Value,
    pub tool: Value,
    pub method: Value,
    pub data_evaluation: Value,
    pub history: Value,
}

impl V1SchemaCache {
    async fn download_all(
        version: SchemaVersion,
        use_cache: bool,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if use_cache && let Ok(cache) = Self::load_from_cache(version).await {
            return Ok(cache);
        }

        let (general, customer, tool, method, data_evaluation, history) = tokio::join!(
            download_and_parse_schema(V1SchemaType::General, version),
            download_and_parse_schema(V1SchemaType::Customer, version),
            download_and_parse_schema(V1SchemaType::Tool, version),
            download_and_parse_schema(V1SchemaType::Method, version),
            download_and_parse_schema(V1SchemaType::DataEvaluation, version),
            download_and_parse_schema(V1SchemaType::History, version),
        );

        let cache = V1SchemaCache {
            general: general?,
            customer: customer?,
            tool: tool?,
            method: method?,
            data_evaluation: data_evaluation?,
            history: history?,
        };

        let _ = cache.save_to_cache(version).await;
        Ok(cache)
    }

    pub fn get(&self, schema_type: V1SchemaType) -> &Value {
        match schema_type {
            V1SchemaType::General => &self.general,
            V1SchemaType::Customer => &self.customer,
            V1SchemaType::Tool => &self.tool,
            V1SchemaType::Method => &self.method,
            V1SchemaType::DataEvaluation => &self.data_evaluation,
            V1SchemaType::History => &self.history,
        }
    }

    async fn load_from_cache(version: SchemaVersion) -> Result<Self, Box<dyn std::error::Error>> {
        let cache_dir = get_cache_dir(version)?;

        let general = load_schema_from_file(&cache_dir, V1SchemaType::General).await?;
        let customer = load_schema_from_file(&cache_dir, V1SchemaType::Customer).await?;
        let tool = load_schema_from_file(&cache_dir, V1SchemaType::Tool).await?;
        let method = load_schema_from_file(&cache_dir, V1SchemaType::Method).await?;
        let data_evaluation =
            load_schema_from_file(&cache_dir, V1SchemaType::DataEvaluation).await?;
        let history = load_schema_from_file(&cache_dir, V1SchemaType::History).await?;

        Ok(V1SchemaCache {
            general,
            customer,
            tool,
            method,
            data_evaluation,
            history,
        })
    }

    async fn save_to_cache(
        &self,
        version: SchemaVersion,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let cache_dir = get_cache_dir(version)?;
        fs::create_dir_all(&cache_dir).await.ok();

        save_schema_to_file(&cache_dir, V1SchemaType::General, &self.general).await?;
        save_schema_to_file(&cache_dir, V1SchemaType::Customer, &self.customer).await?;
        save_schema_to_file(&cache_dir, V1SchemaType::Tool, &self.tool).await?;
        save_schema_to_file(&cache_dir, V1SchemaType::Method, &self.method).await?;
        save_schema_to_file(
            &cache_dir,
            V1SchemaType::DataEvaluation,
            &self.data_evaluation,
        )
        .await?;
        save_schema_to_file(&cache_dir, V1SchemaType::History, &self.history).await?;

        Ok(())
    }
}

// V2-specific schema cache
pub struct V2SchemaCache {
    pub general: Value,
    pub customer: Value,
    pub tool: Value,
    pub method: Value,
    pub data_evaluation: Value,
    pub history: Value,
}

impl V2SchemaCache {
    async fn download_all(
        version: SchemaVersion,
        use_cache: bool,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if use_cache && let Ok(cache) = Self::load_from_cache(version).await {
            return Ok(cache);
        }

        let (general, customer, tool, method, data_evaluation, history) = tokio::join!(
            download_and_parse_schema(V2SchemaType::General, version),
            download_and_parse_schema(V2SchemaType::Customer, version),
            download_and_parse_schema(V2SchemaType::Tool, version),
            download_and_parse_schema(V2SchemaType::Method, version),
            download_and_parse_schema(V2SchemaType::DataEvaluation, version),
            download_and_parse_schema(V2SchemaType::History, version),
        );

        let cache = V2SchemaCache {
            general: general?,
            customer: customer?,
            tool: tool?,
            method: method?,
            data_evaluation: data_evaluation?,
            history: history?,
        };

        let _ = cache.save_to_cache(version).await;
        Ok(cache)
    }

    pub fn get(&self, schema_type: V2SchemaType) -> &Value {
        match schema_type {
            V2SchemaType::General => &self.general,
            V2SchemaType::Customer => &self.customer,
            V2SchemaType::Tool => &self.tool,
            V2SchemaType::Method => &self.method,
            V2SchemaType::DataEvaluation => &self.data_evaluation,
            V2SchemaType::History => &self.history,
        }
    }

    async fn load_from_cache(version: SchemaVersion) -> Result<Self, Box<dyn std::error::Error>> {
        let cache_dir = get_cache_dir(version)?;

        let general = load_schema_from_file(&cache_dir, V2SchemaType::General).await?;
        let customer = load_schema_from_file(&cache_dir, V2SchemaType::Customer).await?;
        let tool = load_schema_from_file(&cache_dir, V2SchemaType::Tool).await?;
        let method = load_schema_from_file(&cache_dir, V2SchemaType::Method).await?;
        let data_evaluation =
            load_schema_from_file(&cache_dir, V2SchemaType::DataEvaluation).await?;
        let history = load_schema_from_file(&cache_dir, V2SchemaType::History).await?;

        Ok(V2SchemaCache {
            general,
            customer,
            tool,
            method,
            data_evaluation,
            history,
        })
    }

    async fn save_to_cache(
        &self,
        version: SchemaVersion,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let cache_dir = get_cache_dir(version)?;
        fs::create_dir_all(&cache_dir).await.ok();

        save_schema_to_file(&cache_dir, V2SchemaType::General, &self.general).await?;
        save_schema_to_file(&cache_dir, V2SchemaType::Customer, &self.customer).await?;
        save_schema_to_file(&cache_dir, V2SchemaType::Tool, &self.tool).await?;
        save_schema_to_file(&cache_dir, V2SchemaType::Method, &self.method).await?;
        save_schema_to_file(
            &cache_dir,
            V2SchemaType::DataEvaluation,
            &self.data_evaluation,
        )
        .await?;
        save_schema_to_file(&cache_dir, V2SchemaType::History, &self.history).await?;

        Ok(())
    }
}

// Unified schema cache enum
pub enum SchemaCache {
    V1(V1SchemaCache),
    V2(V2SchemaCache),
}

impl SchemaCache {
    // Download all schemas from GitHub
    pub async fn download_all(
        version: SchemaVersion,
        use_cache: bool,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        match version {
            SchemaVersion::V1 => {
                let cache = V1SchemaCache::download_all(version, use_cache).await?;
                Ok(SchemaCache::V1(cache))
            }
            SchemaVersion::V2 => {
                let cache = V2SchemaCache::download_all(version, use_cache).await?;
                Ok(SchemaCache::V2(cache))
            }
        }
    }

    // Access schemas by field name (works across versions)
    pub fn general(&self) -> &Value {
        match self {
            SchemaCache::V1(cache) => &cache.general,
            SchemaCache::V2(cache) => &cache.general,
        }
    }

    pub fn customer(&self) -> &Value {
        match self {
            SchemaCache::V1(cache) => &cache.customer,
            SchemaCache::V2(cache) => &cache.customer,
        }
    }

    pub fn tool(&self) -> &Value {
        match self {
            SchemaCache::V1(cache) => &cache.tool,
            SchemaCache::V2(cache) => &cache.tool,
        }
    }

    pub fn method(&self) -> &Value {
        match self {
            SchemaCache::V1(cache) => &cache.method,
            SchemaCache::V2(cache) => &cache.method,
        }
    }

    pub fn data_evaluation(&self) -> &Value {
        match self {
            SchemaCache::V1(cache) => &cache.data_evaluation,
            SchemaCache::V2(cache) => &cache.data_evaluation,
        }
    }

    pub fn history(&self) -> &Value {
        match self {
            SchemaCache::V1(cache) => &cache.history,
            SchemaCache::V2(cache) => &cache.history,
        }
    }

    pub fn all_sections(&self) -> Vec<(&'static str, &Value)> {
        match self {
            SchemaCache::V1(_) => vec![
                ("General Section", self.general()),
                ("Customer Section", self.customer()),
                ("Tool Specific", self.tool()),
                ("Method Specific", self.method()),
                ("Data Evaluation", self.data_evaluation()),
                ("History", self.history()),
            ],
            SchemaCache::V2(_) => vec![
                ("generalSection", self.general()),
                ("customerSpecific", self.customer()),
                ("toolSpecific", self.tool()),
                ("methodSpecific", self.method()),
                ("dataEvaluation", self.data_evaluation()),
                ("history", self.history()),
            ],
        }
    }

    pub fn required_sections(&self) -> &'static [&'static str] {
        match self {
            SchemaCache::V1(_) => &["General Section", "Method Specific"],
            SchemaCache::V2(_) => &["generalSection", "methodSpecific"],
        }
    }
}

async fn download_schema<T: SchemaTypeTrait>(
    schema_type: T,
    version: SchemaVersion,
) -> Result<String, Box<dyn std::error::Error>> {
    let url = schema_type.url_for(version);
    let client = reqwest::Client::new();
    let response = client.get(&url).send().await.map_err(|e| {
        format!(
            "Failed to download {} schema from {}: {}",
            schema_type.label(),
            url,
            e
        )
    })?;
    let response = response.error_for_status().map_err(|e| {
        format!(
            "{} schema endpoint {} returned an error response: {}",
            schema_type.label(),
            url,
            e
        )
    })?;
    response.text().await.map_err(|e| {
        format!(
            "Failed to read {} schema body from {}: {}",
            schema_type.label(),
            url,
            e
        )
        .into()
    })
}

fn parse_schema<T: SchemaTypeTrait>(
    schema_type: T,
    schema_text: &str,
    version: SchemaVersion,
) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let url = schema_type.url_for(version);
    serde_json::from_str(schema_text).map_err(|e| {
        format!(
            "Failed to parse {} schema downloaded from {}: {}",
            schema_type.label(),
            url,
            e
        )
        .into()
    })
}

async fn download_and_parse_schema<T: SchemaTypeTrait>(
    schema_type: T,
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
async fn load_schema_from_file<T: SchemaTypeTrait>(
    cache_dir: &Path,
    schema_type: T,
) -> Result<Value, Box<dyn std::error::Error>> {
    let file_path = cache_dir.join(schema_type.file_name());
    let content = fs::read_to_string(&file_path).await?;
    let schema = serde_json::from_str(&content)?;
    Ok(schema)
}

// Save a single schema to cache file
async fn save_schema_to_file<T: SchemaTypeTrait>(
    cache_dir: &Path,
    schema_type: T,
    schema: &Value,
) -> Result<(), Box<dyn std::error::Error>> {
    let file_path = cache_dir.join(schema_type.file_name());
    let content = serde_json::to_string(schema)?;
    fs::write(&file_path, content).await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_schema_version_branch() {
        assert_eq!(SchemaVersion::V1.branch(), "master");
        assert_eq!(SchemaVersion::V2.branch(), "master");
    }

    #[test]
    fn test_schema_version_folder() {
        assert_eq!(SchemaVersion::V1.folder(), "v1");
        assert_eq!(SchemaVersion::V2.folder(), "v2");
    }

    #[test]
    fn test_v1_schema_type_file_name() {
        assert_eq!(V1SchemaType::General.file_name(), "General Section.json");
        assert_eq!(V1SchemaType::Customer.file_name(), "Customer Section.json");
        assert_eq!(V1SchemaType::Tool.file_name(), "Tool Specific.json");
        assert_eq!(V1SchemaType::Method.file_name(), "Method Specific.json");
        assert_eq!(
            V1SchemaType::DataEvaluation.file_name(),
            "Data Evaluation.json"
        );
        assert_eq!(V1SchemaType::History.file_name(), "History.json");
    }

    #[test]
    fn test_v2_schema_type_file_name() {
        assert_eq!(V2SchemaType::General.file_name(), "generalSection.json");
        assert_eq!(V2SchemaType::Customer.file_name(), "customerSection.json");
        assert_eq!(V2SchemaType::Tool.file_name(), "toolSpecific.json");
        assert_eq!(V2SchemaType::Method.file_name(), "methodSpecific.json");
        assert_eq!(
            V2SchemaType::DataEvaluation.file_name(),
            "dataEvaluation.json"
        );
        assert_eq!(V2SchemaType::History.file_name(), "historySection.json");
    }

    #[test]
    fn test_v1_schema_type_label() {
        assert_eq!(V1SchemaType::General.label(), "General Section");
        assert_eq!(V1SchemaType::Customer.label(), "Customer Section");
        assert_eq!(V1SchemaType::Tool.label(), "Tool Specific");
        assert_eq!(V1SchemaType::Method.label(), "Method Specific");
        assert_eq!(V1SchemaType::DataEvaluation.label(), "Data Evaluation");
        assert_eq!(V1SchemaType::History.label(), "History");
    }

    #[test]
    fn test_v2_schema_type_label() {
        assert_eq!(V2SchemaType::General.label(), "general");
        assert_eq!(V2SchemaType::Customer.label(), "customer");
        assert_eq!(V2SchemaType::Tool.label(), "tool");
        assert_eq!(V2SchemaType::Method.label(), "method");
        assert_eq!(V2SchemaType::DataEvaluation.label(), "data evaluation");
        assert_eq!(V2SchemaType::History.label(), "history");
    }

    #[test]
    fn test_schema_type_url_for() {
        let url = V2SchemaType::General.url_for(SchemaVersion::V2);
        assert!(url.contains("master"));
        assert!(url.contains("v2"));
        assert!(url.contains("generalSection.json"));
        assert!(url.starts_with(SCHEMA_BASE_URL));
    }

    #[test]
    fn test_parse_schema_valid() {
        let schema_text = r#"{"$schema": "http://json-schema.org/draft-07/schema#"}"#;
        let result = parse_schema(V2SchemaType::General, schema_text, SchemaVersion::V2);
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_schema_invalid() {
        let schema_text = "not valid json";
        let result = parse_schema(V2SchemaType::General, schema_text, SchemaVersion::V2);
        assert!(result.is_err());
    }

    #[test]
    fn test_v1_schema_cache_get() {
        let cache = V1SchemaCache {
            general: serde_json::json!({"type": "general"}),
            customer: serde_json::json!({"type": "customer"}),
            tool: serde_json::json!({"type": "tool"}),
            method: serde_json::json!({"type": "method"}),
            data_evaluation: serde_json::json!({"type": "data_evaluation"}),
            history: serde_json::json!({"type": "history"}),
        };

        assert_eq!(
            cache.get(V1SchemaType::General),
            &serde_json::json!({"type": "general"})
        );
        assert_eq!(
            cache.get(V1SchemaType::Customer),
            &serde_json::json!({"type": "customer"})
        );
    }

    #[test]
    fn test_unified_schema_cache_accessors() {
        let v1_cache = V1SchemaCache {
            general: serde_json::json!({"type": "general"}),
            customer: serde_json::json!({"type": "customer"}),
            tool: serde_json::json!({"type": "tool"}),
            method: serde_json::json!({"type": "method"}),
            data_evaluation: serde_json::json!({"type": "data_evaluation"}),
            history: serde_json::json!({"type": "history"}),
        };

        let cache = SchemaCache::V1(v1_cache);
        assert_eq!(cache.general(), &serde_json::json!({"type": "general"}));
        assert_eq!(cache.customer(), &serde_json::json!({"type": "customer"}));
    }
}
