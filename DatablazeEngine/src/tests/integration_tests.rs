
#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use axum::{http::StatusCode};
    use serde_json::Value;
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

        let parsed = parse_body::<DatabaseResponseModel>(response).await;
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

        let parsed = parse_body::<TableResponseModel>(response).await;
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
            data: vec!(HashMap::from([
                (column_name_one.to_string(), Value::String(column_value_one.to_string())),
                (column_name_two.to_string(), Value::Number(column_value_two.into())),
            ]))
        };
        let response = request_create_row(db_name.to_string(), table_name.to_string(), body, &mut app).await;
        assert_eq!(response.status(), StatusCode::CREATED);

        let response = request_get_row(db_name.to_string(), table_name, 0, &mut app).await;
        assert_eq!(response.status(), StatusCode::OK, "Failed to get row");

        let parsed = parse_body::<HashMap<String, Value>>(response).await;
        assert_eq!(parsed.get(column_name_one).unwrap().as_str().unwrap(), column_value_one);
        assert_eq!(parsed.get(column_name_two).unwrap().as_u64().unwrap(), column_value_two);
    }

    #[tokio::test]
    async fn update_row() {

        let db_name = "testDB";
        let table_name = "testTable";
        let column_name_one = "testColumnOne";
        let column_type_one = "string";
        let column_value_one = "test_one";
        let column_value_two = "test_two";
        let column_value_two_updated = "test_two_updated";
        let column_value_three = "test_three";

        let body = DatabaseCreateRequestModel {
            name: db_name.to_string(),
            datastore: "in_memory".to_string()
        };

        let mut app = create_router().into_service();
        let response = request_create_database(body, &mut app).await;
        assert_eq!(response.status(), StatusCode::CREATED);

        let body = TableCreateRequestModel {
            name: table_name.to_string(),
            columns: vec![
                ColumnCreateRequestModel {
                    name: column_name_one.to_string(),
                    column_type: column_type_one.to_string(),
                },
            ],
        };
        let response = request_create_table(db_name.to_string(), body, &mut app).await;
        assert_eq!(response.status(), StatusCode::CREATED);

        let body = RowCreateRequestModel {
            data: vec!(
                HashMap::from([
                    (column_name_one.to_string(), Value::String(column_value_one.to_string())),
                ]),
                HashMap::from([
                    (column_name_one.to_string(), Value::String(column_value_two.to_string())),
                ]),
                HashMap::from([
                    (column_name_one.to_string(), Value::String(column_value_three.to_string())),
                ])
            )
        };
        let response = request_create_row(db_name.to_string(), table_name.to_string(), body, &mut app).await;
        assert_eq!(response.status(), StatusCode::CREATED);

        let response = request_get_row(db_name.to_string(), table_name, 1, &mut app).await;
        assert_eq!(response.status(), StatusCode::OK, "Failed to get row");

        let parsed = parse_body::<HashMap<String, Value>>(response).await;
        assert_eq!(parsed.get(column_name_one).unwrap().as_str().unwrap(), column_value_two);
        
        let body = RowUpdateRequestModel {
            data: HashMap::from([
                    (column_name_one.to_string(), Value::String(column_value_two_updated.to_string()))])
        };

        let response = request_update_row(db_name.to_string(), table_name.to_string(), 1, body, &mut app).await;
        assert_eq!(response.status(), StatusCode::ACCEPTED, "Failed to update row");

        let response = request_get_row(db_name.to_string(), table_name, 1, &mut app).await;
        assert_eq!(response.status(), StatusCode::OK, "Failed to get row");
        
        let parsed = parse_body::<HashMap<String, Value>>(response).await;
        assert_eq!(parsed.get(column_name_one).unwrap().as_str().unwrap(), column_value_two_updated);
    }

    #[tokio::test]
    async fn delete_row() {

        let db_name = "testDB";
        let table_name = "testTable";
        let column_name_one = "testColumnOne";
        let column_type_one = "string";
        let column_value_one = "test_one";
        let column_value_two = "test_two";
        let column_value_three = "test_three";

        let body = DatabaseCreateRequestModel {
            name: db_name.to_string(),
            datastore: "in_memory".to_string()
        };

        let mut app = create_router().into_service();
        let response = request_create_database(body, &mut app).await;
        assert_eq!(response.status(), StatusCode::CREATED);

        let body = TableCreateRequestModel {
            name: table_name.to_string(),
            columns: vec![
                ColumnCreateRequestModel {
                    name: column_name_one.to_string(),
                    column_type: column_type_one.to_string(),
                },
            ],
        };
        let response = request_create_table(db_name.to_string(), body, &mut app).await;
        assert_eq!(response.status(), StatusCode::CREATED);

        let body = RowCreateRequestModel {
            data: vec!(
                HashMap::from([
                    (column_name_one.to_string(), Value::String(column_value_one.to_string())),
                ]),
                HashMap::from([
                    (column_name_one.to_string(), Value::String(column_value_two.to_string())),
                ]),
                HashMap::from([
                    (column_name_one.to_string(), Value::String(column_value_three.to_string())),
                ])
            )
        };
        let response = request_create_row(db_name.to_string(), table_name.to_string(), body, &mut app).await;
        assert_eq!(response.status(), StatusCode::CREATED);

        let response = request_get_row(db_name.to_string(), table_name, 1, &mut app).await;
        assert_eq!(response.status(), StatusCode::OK, "Failed to get row");

        let parsed = parse_body::<HashMap<String, Value>>(response).await;
        assert_eq!(parsed.get(column_name_one).unwrap().as_str().unwrap(), column_value_two);

        let response = request_delete_row(db_name.to_string(), table_name.to_string(), 1, &mut app).await;
        assert_eq!(response.status(), StatusCode::ACCEPTED, "Failed to update row");

        let response = request_get_row(db_name.to_string(), table_name, 1, &mut app).await;
        assert_eq!(response.status(), StatusCode::OK, "Failed to get row");

        let parsed = parse_body::<HashMap<String, Value>>(response).await;
        assert_eq!(parsed.get(column_name_one).unwrap().as_str().unwrap(), column_value_three);
    }
}