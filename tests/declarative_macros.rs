use metadata_macro::metadata;

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
    assert_eq!(NoFields {}, NoFields {});
    assert_eq!(NoFieldsMetadata {}, NoFieldsMetadata {});

    metadata!(
        {
            #[derive(PartialEq, Debug)]
            struct NoFieldsTuple();
        },
        #[derive(PartialEq, Debug)] struct NoFieldsTupleMetadata: (),
    );
    assert_eq!(NoFieldsTuple(), NoFieldsTuple());
    assert_eq!(NoFieldsTupleMetadata(), NoFieldsTupleMetadata());
}

#[test]
fn test_multiple_metadata_structs() {
    metadata!(
        {
            #[derive(PartialEq, Debug, Default)]
            struct MultipleMetadataStructs {
                field_a: bool,
                field_b: usize,
            }
        },
        #[derive(PartialEq, Debug, Default)] struct StringLiteralA: &'static str,
        #[derive(PartialEq, Debug, Default)] struct OptionUnit: Option<()>,
        #[derive(PartialEq, Debug, Default)] struct Unit: (),
        #[derive(PartialEq, Debug, Default)] struct StringLiteralB: &'static str,
    );
    assert_eq!(MultipleMetadataStructs::default(), MultipleMetadataStructs { field_a: false, field_b: 0 });
    assert_eq!(StringLiteralA::default(), StringLiteralA { field_a: "", field_b: "" });
    assert_eq!(OptionUnit::default(), OptionUnit { field_a: None, field_b: None });
    assert_eq!(Unit::default(), Unit { field_a: (), field_b: () });
    assert_eq!(StringLiteralB::default(), StringLiteralB { field_a: "", field_b: "" });

    metadata!(
        {
            #[derive(PartialEq, Debug, Default)]
            struct MultipleMetadataStructsTuple(bool, usize);
        },
        #[derive(PartialEq, Debug, Default)] struct StringLiteralTupleA: &'static str,
        #[derive(PartialEq, Debug, Default)] struct OptionUnitTuple: Option<()>,
        #[derive(PartialEq, Debug, Default)] struct UnitTuple: (),
        #[derive(PartialEq, Debug, Default)] struct StringLiteralTupleB: &'static str,
    );
    assert_eq!(MultipleMetadataStructsTuple::default(), MultipleMetadataStructsTuple(false, 0));
    assert_eq!(StringLiteralTupleA::default(), StringLiteralTupleA("", ""));
    assert_eq!(OptionUnitTuple::default(), OptionUnitTuple(None, None));
    assert_eq!(UnitTuple::default(), UnitTuple((), ()));
    assert_eq!(StringLiteralTupleB::default(), StringLiteralTupleB("", ""));
}

#[test]
fn test_keywords() {
    metadata!(
        {
            pub(crate) struct StructVisibility {
                pub public_field: (),
                private_field: (),
                pub (crate) crate_field: (),
            }
        },
        pub struct PublicStruct: (),
        struct PrivateStruct: (),
        pub(crate) struct CrateStruct: (),
    );
    metadata!(
        {
            pub(crate) struct StructVisibilityTuple();
        },
        pub struct PublicStructTuple: (),
        struct PrivateStructTuple: (),
        pub(crate) struct CrateStructTuple: (),
    );
}

#[test]
fn test_unusual_syntax() {
    metadata!(
        {
            struct NoLeadingComma {
                spaced_tokens : ( ) ,
                no_leading_comma: ()
            }
        },
        struct NoLeadingCommaSpacedTokensMetadata : ( )
    );
    metadata!(
        {
            struct HasLeadingCommaTuple((),);
        },
        struct NoLeadingCommaSpacedTokensTupleMetadata : ( )
    );
}

#[test]
fn test_doc_comments() {
    metadata!(
        {
            /// Doc comment.
            struct HasDocComment {
                /// Doc comment.
                has_doc_comment: (),
            }
        },
        /// Doc comment.
        struct HasDocCommentMetadata: (),
    );
    metadata!(
        {
            /// Doc comment.
            struct HasDocCommentTuple(
                /// Doc comment.
                ()
            );
        },
        /// Doc comment.
        struct HasDocCommentTupleMetadata: (),
    );
}

#[test]
fn test_generics_and_lifetimes() {
    #[derive(Default, PartialEq, Debug)]
    struct HasGenerics<T>(T);
    #[derive(Default, PartialEq, Debug)]
    struct HasLifetimes<'a>(Option<&'a ()>);
    #[derive(Default, PartialEq, Debug)]
    struct HasBoth<'a, T>(T, Option<&'a ()>);

    metadata!(
        {
            #[derive(Default, PartialEq, Debug)]
            struct Polymorpher<'a, 'b: 'static, T> {
                normal_field_a: bool,
                has_generics: T,
                has_lifetimes: Option<&'a ()>,
                has_both: Option<&'a T>,

                passes_generics: HasGenerics<T>,
                passes_lifetimes: HasLifetimes<'a>,
                passes_both: HasBoth<'a, T>,

                has_nested_lifetimes: &'b str,
                normal_field_b: usize,
            }
        },
        struct PolymorpherMetadata: (),
    );
    let default: Polymorpher<'_, '_, usize> = Polymorpher::default();
    assert_eq!(default, Polymorpher {
        normal_field_a: false,
        has_generics: 0,
        has_lifetimes: None,
        has_both: None,

        passes_generics: HasGenerics(0),
        passes_lifetimes: HasLifetimes(None),
        passes_both: HasBoth(0, None),

        has_nested_lifetimes: "",
        normal_field_b: 0,
    })
}

#[cfg(feature = "advanced-tests")]
#[test]
fn test_attributes_derive_clap() {
    use clap::Parser;
    metadata!(
        {
            #[derive(Parser)]
            struct ClapParser {
                normal_field_a: bool,
                #[clap(long)]
                clap_parser_field: bool,
                normal_field_b: usize,
            }
        },
        struct ClapParserMetadata: (),
    );
    metadata!(
        {
            struct HasLeadingCommaTuple((),);
        },
        struct NoLeadingCommaMetadataTuple: ()
    );
}

#[test]
fn test_generic_type_parameters() {
    metadata!(
        {
            struct HasGenericTypeParameteres<T> {
                no_leading_comma: T,
            }
        },
        struct NoLeadingCommaMetadata: ()
    );
}
