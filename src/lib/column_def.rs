#[doc(hidden)]
use serde::Deserialize;

#[doc(hidden)]
use serde_json::Value;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
/// Specifies intended type and the desired type when converted to json
pub enum ColType {
    /// will be converted to a json string
    String,

    /// will be converted to a json number with no decimal points
    Integer,

    /// will be converted to a regular json number
    Float,

    /// will be parsed as date but converted to a unix timestamp in seconds since epoch
    Date,

    /// will be converted to json true or false
    Bool,
}

impl Default for ColType {
    fn default() -> Self {
        ColType::String
    }
}

#[derive(Debug, Default)]
pub struct ColumnDef {
    pub col_type: ColType,
    pub col_name: String,
    pub col_format: String,
    pub col_true_string: String,
    pub col_false_string: String,
    pub col_default: Value,
}

#[derive(Default, Deserialize, Debug)]
/// User input for column options
pub struct ColumnDefOptions {
    #[serde(rename = "type")]
    /// specifies the type of column
    pub col_type: Option<ColType>,

    #[serde(rename = "name")]
    /// the name to be used for the json key
    pub col_name: Option<String>,

    #[serde(rename = "format")]
    /// a format specifier for the date type
    pub col_format: Option<String>,

    #[serde(rename = "trueString")]
    /// the string to parse as "true" when column type is bool
    pub col_true_string: Option<String>,

    #[serde(rename = "falseString")]
    /// the string to parse as "false" when column type is bool
    pub col_false_string: Option<String>,

    #[serde(rename = "default")]
    /// default json value to give to this column when unable to parse or empty
    pub col_default: Option<Value>,
}
