use serde::{Deserialize, Serialize};
//use serde_json::Value;
use std::collections::HashMap;
//use strum_macros::{AsRefStr, Display, EnumString, IntoStaticStr};

//#[derive(Debug, Serialize, Deserialize)]
pub type RowMap = HashMap<String, String>;

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct FromValue {
    from: RowMap,
}

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct FromToValue {
    from: RowMap,
    to: RowMap,
}

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all(deserialize = "UPPERCASE", serialize = "PascalCase"))]
pub enum Operation {
    /// Insert a FromValue  ( really we should do Vec<RowMap>)
    Insert(Vec<FromValue>),
    /// Update a Vactor of items that define a From and To of type RowMap
    Update(Vec<FromToValue>),
    /// Delete a Vector of FromValue (RowMap)
    Delete(Vec<FromValue>),
    ///Truncate the table
    Truncate(TableName),
}

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all(deserialize = "lowercase", serialize = "PascalCase"))]
pub enum TableName {
    Role,
    Site,
    Level,
    Package,
}

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all(deserialize = "lowercase", serialize = "PascalCase"))]
pub enum TableOperation {
    Role(Operation),
    Site(Operation),
    Level(Operation),
    Package(Operation),
}

/// A Vector of table operations
#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Changeset {
    /// Internal changesets have transaction ids.
    /// Externally generated ones do not
    transaction_id: Option<i32>,
    actions: Vec<TableOperation>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn can_deserialize_from() {
        fn typed_eg() -> serde_json::Result<()> {
            let data = r#"{"from" : {"path": "facility.dev01"}}"#;
            let _a: FromValue = serde_json::from_str(data)?;
            Ok(())
        }
        assert!(typed_eg().is_ok());
    }
    #[test]
    fn can_deserialize_from_to() {
        fn typed_eg() -> serde_json::Result<()> {
            let data =
                r#"{"from" : {"path": "facility.dev01"}, "to" : {"path": "facility.dev02"}}"#;
            let _a: FromToValue = serde_json::from_str(data)?;
            Ok(())
        }
        assert!(typed_eg().is_ok());
    }
    #[test]
    fn can_deserialize_operation() {
        fn typed_eg() -> serde_json::Result<()> {
            let data =
                r#"{"UPDATE" : [{"from" : {"path": "facility.dev01"}, "to" : {"path": "facility.dev02"}}]}"#;
            let _a: Operation = serde_json::from_str(data)?;
            Ok(())
        }
        assert!(typed_eg().is_ok());
    }
    #[test]
    fn can_deserialize_table_operation() {
        fn typed_eg() -> serde_json::Result<()> {
            let data = r#"{"level" : {"UPDATE" : [{"from" : {"path": "facility.dev01"}, "to" : {"path": "facility.dev02"}}]}}"#;
            let _a: TableOperation = serde_json::from_str(data)?;
            Ok(())
        }
        assert!(typed_eg().is_ok());
    }
    #[test]
    fn can_deserialize_changeset() {
        fn typed_eg() -> serde_json::Result<()> {
            let data = r#"{"transaction_id" : 9918, "actions" : [{"level" : {"UPDATE" : [{"from" : {"path": "facility.dev01"}, "to" : {"path": "facility.dev02"}}]}}]}"#;
            let _a: Changeset = serde_json::from_str(data)?;
            Ok(())
        }
        assert!(typed_eg().is_ok());
    }
}
