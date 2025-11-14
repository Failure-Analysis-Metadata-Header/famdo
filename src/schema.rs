use serde_json::Value;
use std::fmt;

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
    pub fn url(&self) -> &'static str {
        match self {
            SchemaType::General => {
                "https://raw.githubusercontent.com/Failure-Analysis-Metadata-Header/fa-metadata-schema/refs/heads/schema-v2/schema/v2/generalSection.json"
            }
            SchemaType::Customer => {
                "https://raw.githubusercontent.com/Failure-Analysis-Metadata-Header/fa-metadata-schema/refs/heads/schema-v2/schema/v2/customerSection.json"
            }
            SchemaType::Tool => {
                "https://raw.githubusercontent.com/Failure-Analysis-Metadata-Header/fa-metadata-schema/refs/heads/schema-v2/schema/v2/toolSpecific.json"
            }
            SchemaType::Method => {
                "https://raw.githubusercontent.com/Failure-Analysis-Metadata-Header/fa-metadata-schema/refs/heads/schema-v2/schema/v2/methodSpecific.json"
            }
            SchemaType::DataEvaluation => {
                "https://raw.githubusercontent.com/Failure-Analysis-Metadata-Header/fa-metadata-schema/refs/heads/schema-v2/schema/v2/dataEvaluation.json"
            }
            SchemaType::History => {
                "https://raw.githubusercontent.com/Failure-Analysis-Metadata-Header/fa-metadata-schema/refs/heads/schema-v2/schema/v2/historySection.json"
            }
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
    pub fn download_all() -> Result<Self, Box<dyn std::error::Error>> {
        Ok(SchemaCache {
            general: download_and_parse_schema(&SchemaType::General)?,
            customer: download_and_parse_schema(&SchemaType::Customer)?,
            tool: download_and_parse_schema(&SchemaType::Tool)?,
            method: download_and_parse_schema(&SchemaType::Method)?,
            data_evaluation: download_and_parse_schema(&SchemaType::DataEvaluation)?,
            history: download_and_parse_schema(&SchemaType::History)?,
        })
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
}

pub fn download_schema(schema_type: &SchemaType) -> Result<String, Box<dyn std::error::Error>> {
    let url = schema_type.url();
    let response = reqwest::blocking::get(url).map_err(|e| {
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
    response.text().map_err(|e| {
        format!(
            "Failed to read {} schema body from {}: {}",
            schema_type, url, e
        )
        .into()
    })
}

pub fn parse_schema(
    schema_type: &SchemaType,
    schema_text: &str,
) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    serde_json::from_str(schema_text).map_err(|e| {
        format!(
            "Failed to parse {} schema downloaded from {}: {}",
            schema_type,
            schema_type.url(),
            e
        )
        .into()
    })
}

pub fn download_and_parse_schema(
    schema_type: &SchemaType,
) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let schema_text = download_schema(schema_type)?;
    let schema = parse_schema(schema_type, &schema_text)?;
    Ok(schema)
}
