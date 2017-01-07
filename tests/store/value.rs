use bifrost::raft::*;
use bifrost::raft::client::RaftClient;
use bifrost::store::value::string;
use bifrost::store::value::string::commands::{set, get};

#[test]
fn string(){
    let addr = String::from("127.0.0.1:2010");
    let original_string = String::from("The stored text");
    let altered_string = String::from("The altered text");
    let string_sm = string::Value::new_by_name(
        String::from("test"),
        original_string.clone()
    );
    let server = RaftServer::new(Options{
        storage: Storage::Default(),
        address: addr.clone()
    });
    let server = server.unwrap();
    let sm_id = string_sm.id;
    server.register_state_machine(Box::new(string_sm));
    server.bootstrap();

    let client = RaftClient::new(vec!(addr)).unwrap();
    assert_eq!(
        client.execute(sm_id, &get{}).unwrap().unwrap(),
        original_string.clone()
    );
    client.execute(sm_id, &set{v: altered_string.clone()}).unwrap().unwrap();
    assert_eq!(
        client.execute(sm_id, &get{}).unwrap().unwrap(),
        altered_string.clone()
    );
}