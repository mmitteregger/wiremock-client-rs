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


fn create_wire_mock() -> WireMock {
    WireMockBuilder::new()
        .port(8181)
        .build()
}

fn print_json_value<T: serde::Serialize + ?Sized>(value: &T) {
    let json_string = serde_json::to_string_pretty(value).unwrap();
    println!("{}", json_string);
}
