use serde::{Deserialize, Serialize};
use serde_json::json;
use serde_json::Value;

/// This is a common model that is used multiple times in this project.
/// It is used to send a blank request to the server to get a response.

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlankRequest {}

/// Represents the default response from the API.
/// T must be serializable.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DefaultResponse<T>
where
    T: serde::Serialize,
{
    /// Represents the success status of the request.
    pub success: bool,
    /// Represents the message from the API, this can be optional
    pub message: Option<String>,
    /// Represents the data from the API, this can be optional and the type should be Serializable
    pub data: Option<T>,
}

/// Represents the query object for Directus API Request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Query {
    /// The filter of the query
    pub filter: Filter,
    /// The fields to be included in the query
    pub fields: Vec<String>,
}

/// Represents the Directus file for file imports.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DirectusFile {
    /// Folder name, should be configured on the directus as public.
    pub folder: String,
    /// File id, UUID for file generated. This will be included in the hyperlink to get the file.
    pub id: String,
}

/// Represents a filter condition.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilterCondition {
    /// Represents the condition for the filter.
    #[serde(flatten)]
    pub condition: Value,
}

/// Represents filters for query.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Filter {
    /// Use for `AND` condition.
    pub _and: Vec<FilterCondition>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct QueryBuilder {
    filters: Vec<(String, Value)>,
}

impl QueryBuilder {
    pub fn new() -> Self {
        QueryBuilder {
            filters: Vec::new(),
        }
    }

    pub fn add_filter(&mut self, field: &str, value: Value) {
        self.filters.push((field.to_string(), value));
    }

    pub fn build(&self) -> Value {
        let mut filter_json = json!({});
        for (field, value) in &self.filters {
            let field_json = json!({ field: value });
            filter_json
                .as_object_mut()
                .unwrap()
                .extend(field_json.as_object().unwrap().clone());
        }
        filter_json
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Greeks {
    /// The delta.
    pub delta: f64,
    /// The gamma.
    pub gamma: f64,
    /// The theta.
    pub theta: f64,
}
