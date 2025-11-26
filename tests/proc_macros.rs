// use structfield_metadata::metadata;
//
// #[test]
// fn test_no_fields() {
//     #[derive(Metadata, PartialEq, Debug)]
//     struct NoFields {
//     }
//     #[derive(Metadata, PartialEq, Debug)]
//     struct NoFieldsTuple();
//
//     assert_eq!(NoFields {}, NoFields {});
//     assert_eq!(NoFieldsMetadata {}, NoFieldsMetadata {});
//     assert_eq!(NoFieldsTuple(), NoFieldsTuple());
//     assert_eq!(NoFieldsTupleMetadata(), NoFieldsTupleMetadata());
// }
//
// #[test]
// fn test_multiple_metadata_structs() {
//     metadata!(
//         {
//             #[derive(PartialEq, Debug, Default)]
//             struct MultipleMetadataStructs {
//                 field_a: bool,
//                 field_b: usize,
//             }
//         },
//         #[derive(PartialEq, Debug, Default)] struct StringLiteralA: &'static str,
//         #[derive(PartialEq, Debug, Default)] struct OptionUnit: Option<()>,
//         #[derive(PartialEq, Debug, Default)] struct Unit: (),
//         #[derive(PartialEq, Debug, Default)] struct StringLiteralB: &'static str,
//     );
//     assert_eq!(MultipleMetadataStructs::default(), MultipleMetadataStructs { field_a: false, field_b: 0 });
//     assert_eq!(StringLiteralA::default(), StringLiteralA { field_a: "", field_b: "" });
//     assert_eq!(OptionUnit::default(), OptionUnit { field_a: None, field_b: None });
//     assert_eq!(Unit::default(), Unit { field_a: (), field_b: () });
//     assert_eq!(StringLiteralB::default(), StringLiteralB { field_a: "", field_b: "" });
// }
//
// #[test]
// fn test_keywords() {
//     metadata!(
//         {
//             pub(crate) struct StructVisibility {
//                 pub public_field: (),
//                 private_field: (),
//                 pub (crate) crate_field: (),
//             }
//         },
//         pub struct PublicStruct: (),
//         struct PrivateStruct: (),
//         pub(crate) struct CrateStruct: (),
//     );
// }
//
// #[test]
// fn test_no_leading_commas() {
//     metadata!(
//         {
//             struct NoLeadingComma {
//                 no_leading_comma: ()
//             }
//         },
//         struct NoLeadingCommaMetadata: ()
//     );
// }
