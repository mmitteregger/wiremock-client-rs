use wiremock_client::*;
use wiremock_client::common::metadata;
use uuid::Uuid;


#[test]
pub fn create_and_retrieve_stub_metadata() {
    let wire_mock = create_wire_mock();

    let stub = wire_mock.stub_for(get("/with-metadata")
        .with_id(Uuid::new_v4())
        .with_metadata(metadata()
            .attr("one", 1)
            .attr("two", "2")
            .attr("three", true)
            .attr("four", metadata()
                .attr("five", "55555")
            )
            .list("six", vec![1, 2, 3])
        ))
        .unwrap();

    let retrieved_stub = wire_mock.get_stub_mapping(stub.id())
        .unwrap()
        .unwrap();
    assert_eq!(wire_mock.remove_stub_mapping(stub.id()).unwrap(), true);

    let metadata = retrieved_stub.metadata();
    print_json_value(&metadata);

    assert_eq!(metadata.get_u64("one"), Some(1));
    assert_eq!(metadata.get_str("two"), Some("2"));
    assert_eq!(metadata.get_bool("three"), Some(true));

    let four = metadata.get_metadata("four").unwrap();

    assert_eq!(four.get_str("five"), Some("55555"));

    let six = metadata.get_mapped_array("six", |value| value.as_u64()).unwrap();
    assert_eq!(six[0], Some(1));
}

#[test]
pub fn can_find_stubs_by_metadata() {
    let wire_mock = create_wire_mock();

    let stub1 = wire_mock.stub_for(get("/with-metadata")
        .with_id(Uuid::new_v4())
        .with_metadata(metadata()
            .attr("can_find_stubs_by_metadata-four", metadata()
                .attr("can_find_stubs_by_metadata-five", "55555")
            )
            .list("can_find_stubs_by_metadata-six", vec![1, 2, 3])
    )).unwrap();
    let stub2 = wire_mock.stub_for(get("/without-metadata")).unwrap();

    let json_path = "$..can_find_stubs_by_metadata-four.can_find_stubs_by_metadata-five";
    let stubs = wire_mock.find_stubs_by_metadata(matching_json_path(json_path)).unwrap();
    print_json_value(&stubs);
    assert_eq!(wire_mock.remove_stub_mapping(stub1.id()).unwrap(), true);
    assert_eq!(wire_mock.remove_stub_mapping(stub2.id()).unwrap(), true);

    assert_eq!(stubs.len(), 1);
    let retrieved_stub = &stubs[0];
    assert_eq!(retrieved_stub.id(), stub1.id());
}


fn create_wire_mock() -> WireMock {
    WireMockBuilder::new()
        .port(8181)
        .build()
}

fn print_json_value<T: serde::Serialize + ?Sized>(value: &T) {
    let json_string = serde_json::to_string_pretty(value).unwrap();
    println!("{}", json_string);
}
