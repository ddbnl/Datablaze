
use std::thread;
use std::thread::JoinHandle;
use datablaze_engine_lib::network::server::Server;
use datablaze_sdk::client::database_client::DatabaseClient;
use datablaze_sdk::commands::server::StopServerCommand;

pub fn get_server_and_client() -> (JoinHandle<()>, DatabaseClient) {
    let addr = "127.0.0.1";
    let port = "3333";
    
    let server_handle = thread::spawn(move || {
        Server::default()
            .use_address("0.0.0.0")
            .use_port(port)
            .run_server();
    });
    let mut client = DatabaseClient::default();
    client
        .use_address(addr)
        .use_port(port);
    (server_handle, client)

}

pub fn stop_server(server_handle: JoinHandle<()>, database_client: DatabaseClient) {
    let stop_server =  StopServerCommand;
    let stop = database_client.send_command(stop_server);
    assert_eq!(stop, Ok(String::new()), "Stop command could not be sent");
    server_handle.join().unwrap();
}
