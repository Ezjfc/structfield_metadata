use structfield_metadata::metadata;

#[test]
fn test_no_fields() {
    metadata!(
        {
            #[derive(PartialEq, Debug)]
            struct NoFields {
            }
        },
        #[derive(PartialEq, Debug)] struct NoFieldsMetadata: (),
    );
    metadata!(
        {
            #[derive(PartialEq, Debug)]
            struct NoFieldsTuple();
        },
        #[derive(PartialEq, Debug)] struct NoFieldsTupleMetadata: (),
    );
    assert_eq!(NoFields {}, NoFields {});
    assert_eq!(NoFieldsMetadata {}, NoFieldsMetadata {});
    assert_eq!(NoFieldsTuple(), NoFieldsTuple());
    assert_eq!(NoFieldsTupleMetadata(), NoFieldsTupleMetadata());
}
