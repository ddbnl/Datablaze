
#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use axum::{http::StatusCode};
    use serde_json::Value;
    use log::debug;
    use crate::network::model::*;
    use crate::network::route::create_router;
    use crate::tests::support::*;


    #[tokio::test]
    async fn create_database() {
        let mut app = create_router().into_service();

        let db_name = "testDB";
        let db_datastore = "in_memory";

        let body = DatabaseCreateRequestModel {
            name: db_name.to_string(),
            datastore: "in_memory".to_string()
        };
        let response = request_create_database(body, &mut app).await;
        assert_eq!(response.status(), StatusCode::CREATED);

        let response = request_get_database(db_name, &mut app).await;
        assert_eq!(response.status(), StatusCode::OK, "Failed to get database");

        let body_bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .map_err(|e| format!("Failed to read body: {}", e)).unwrap();
        let parsed: DatabaseResponseModel = serde_json::from_slice(&body_bytes).unwrap();
        assert_eq!(parsed.name, db_name);
        assert_eq!(parsed.datastore, db_datastore);

    }

    #[tokio::test]
    async fn create_table() {
        let mut app = create_router().into_service();

        let db_name = "testDB";
        let table_name = "testTable";
        let column_name = "testColumn";
        let column_type = "string";

        let body = DatabaseCreateRequestModel {
            name: db_name.to_string(),
            datastore: "in_memory".to_string()
        };
        let response = request_create_database(body, &mut app).await;
        assert_eq!(response.status(), StatusCode::CREATED);

        let body = TableCreateRequestModel {
            name: table_name.to_string(),
            columns: vec![
                ColumnCreateRequestModel {
                    name: column_name.to_string(),
                    column_type: column_type.to_string(),
                }
            ],
        };
        let response = request_create_table(db_name.to_string(), body, &mut app).await;
        assert_eq!(response.status(), StatusCode::CREATED);

        let response = request_get_table(db_name.to_string(), table_name, &mut app).await;
        assert_eq!(response.status(), StatusCode::OK, "Failed to get table");

        let body_bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .map_err(|e| format!("Failed to read body: {}", e)).unwrap();
        let parsed: TableResponseModel = serde_json::from_slice(&body_bytes).unwrap();
        assert_eq!(parsed.name, table_name);
        assert_eq!(parsed.columns.len(), 1);
        assert_eq!(parsed.columns[0].name, column_name);
        assert_eq!(parsed.columns[0].column_type, column_type);
    }

    #[tokio::test]
    async fn create_row() {
        let mut app = create_router().into_service();

        let db_name = "testDB";
        let table_name = "testTable";
        let column_name_one = "testColumnOne";
        let column_type_one = "string";
        let column_value_one = "test";
        let column_name_two = "testColumnTwo";
        let column_type_two = "int";
        let column_value_two = 1;

        let body = DatabaseCreateRequestModel {
            name: db_name.to_string(),
            datastore: "in_memory".to_string()
        };
        let response = request_create_database(body, &mut app).await;
        assert_eq!(response.status(), StatusCode::CREATED);

        let body = TableCreateRequestModel {
            name: table_name.to_string(),
            columns: vec![
                ColumnCreateRequestModel {
                    name: column_name_one.to_string(),
                    column_type: column_type_one.to_string(),
                },
                ColumnCreateRequestModel {
                    name: column_name_two.to_string(),
                    column_type: column_type_two.to_string(),
                },
            ],
        };
        let response = request_create_table(db_name.to_string(), body, &mut app).await;
        assert_eq!(response.status(), StatusCode::CREATED);

        let body = RowCreateRequestModel {
            data: HashMap::from([
                (column_name_one.to_string(), Value::String(column_value_one.to_string())),
                (column_name_two.to_string(), Value::Number(column_value_two.into())),
            ])
        };
        let response = request_create_row(db_name.to_string(), table_name.to_string(), body, &mut app).await;
        assert_eq!(response.status(), StatusCode::CREATED);

        let response = request_get_row(db_name.to_string(), table_name, 0, &mut app).await;
        assert_eq!(response.status(), StatusCode::OK, "Failed to get row");

        let body_bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .map_err(|e| format!("Failed to read body: {}", e)).unwrap();
        let parsed: HashMap<String, Value> = serde_json::from_slice(&body_bytes).unwrap();
        print!("{:#?}", parsed);
        assert_eq!(parsed.get(column_name_one).unwrap().as_str().unwrap(), column_value_one);
        assert_eq!(parsed.get(column_name_two).unwrap().as_u64().unwrap(), column_value_two);
    }
}