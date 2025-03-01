mod support;

use support::support_functions::*;
use datablaze_sdk::commands::server::{CreateDatabaseCommand, GetDatabaseCommand};

use serial_test::serial;

#[test]
#[serial]
fn it_stops() {
    let (server_handle, client) = get_server_and_client();
    stop_server(server_handle, client);
}

#[test]
#[serial]
fn it_creates_a_database() {
    let (server_handle, client) = get_server_and_client();
    
    let database_name = "TestDatabase";
    
    let mut command = CreateDatabaseCommand::default();
    command
        .use_in_memory()
        .use_name(database_name);
    let result = client.send_command(command);
    assert_eq!(result, Ok("Created".to_string()), "Could not send database create command");
    
    let mut command = GetDatabaseCommand::default();
    command
        .use_name(database_name);
    let result = client.send_command(command);
    assert_eq!(result, Ok(format!("Found: {}", database_name)), "Could not send database get command");
    
    stop_server(server_handle, client);
}