//! Rust macros for a very elementary metadata-like system in structs and tuple structs.

/// The main struct can be defined in whatever way desired.
/// For non-tuple structs: fields in the metadata structs will inherit visibility from the main
/// struct.
///
/// This macro receives a main struct and multiple metadata structs. The main struct will remain in
/// its original form while its fields will change the type and be taken to each metadata struct.
///
/// # Examples
/// This example declares a private main struct with two fields and a private metadata struct to
/// hold the multilingual description of the fields:
/// ```
/// #[macro_use] extern crate structfield_metadata;
///
/// fn main() {
///     metadata!({
///             #[derive(Default, PartialEq, Debug)]
///             struct YourStruct {
///                 field_a: bool,
///                 field_b: usize,
///             }
///         },
///         #[derive(PartialEq, Debug)] struct YourStructDescription: &'static str,
///     );
///
///     impl Default for YourStructDescription {
///         fn default() -> Self {
///             Self {
///                 field_a: "Describes field_a",
///                 field_b: "Describes field_b",
///             }
///         }
///     }
///
///     impl YourStructDescription {
///         fn i18n_some_language() -> Self {
///             Self {
///                 field_a: "Describes field_a in other language",
///                 field_b: "Describes field_b in other language",
///             }
///         }
///     }
///
///     assert_eq!(YourStruct::default(), YourStruct { field_a: false, field_b: 0 });
///     assert_eq!(YourStructDescription::default(), YourStructDescription {
///         field_a: "Describes field_a",
///         field_b: "Describes field_b",
///     });
///     assert_eq!(YourStructDescription::i18n_some_language(), YourStructDescription {
///         field_a: "Describes field_a in other language",
///         field_b: "Describes field_b in other language",
///     });
/// }
/// ```
///
/// Feel free to share your use cases by pull requests!
#[macro_export]
macro_rules! metadata {
    (
        $main_struct:tt,
        $(
            $(#[$metadata_attrs:meta])*
            $metadata_vis:vis
            struct $metadata_struct:ident: $metadata_type:ty
        ),+
        $(,)*
    ) => {
        $crate::put_struct!($main_struct);
        $(
            structfield_metadata::metadata_only!(
                $main_struct,
                $(#[$metadata_attrs])* $metadata_vis struct $metadata_struct: $metadata_type
            );
        )+
    };
}

/// The main struct can be defined in whatever way desired.
/// For non-tuple structs: fields in the metadata structs will inherit visibility from the main
/// struct.
///
/// This macro receives a main struct and a metadata struct. The main struct will be discarded
/// its fields will change the type and be taken to the metadata struct.
/// # Examples
/// This example only declares a private metadata struct to hold the multilingual description of
/// two fields:
/// ```
/// #[macro_use] extern crate structfield_metadata;
///
/// fn main() {
///     metadata_only!({
///            #[derive(Default, PartialEq, Debug)]
///            struct YourStruct {
///                field_a: bool,
///                field_b: usize,
///            }
///         },
///         #[derive(PartialEq, Debug)] struct YourStructDescription: &'static str
///     );
///
///     impl Default for YourStructDescription {
///         fn default() -> Self {
///             Self {
///                 field_a: "Describes field_a",
///                 field_b: "Describes field_b",
///             }
///         }
///     }
///
///     impl YourStructDescription {
///         fn i18n_some_language() -> Self {
///             Self {
///                 field_a: "Describes field_a in other language",
///                 field_b: "Describes field_b in other language",
///             }
///         }
///     }
///
///     assert_eq!(YourStructDescription::default(), YourStructDescription {
///         field_a: "Describes field_a",
///         field_b: "Describes field_b",
///     });
///     assert_eq!(YourStructDescription::i18n_some_language(), YourStructDescription {
///         field_a: "Describes field_a in other language",
///         field_b: "Describes field_b in other language",
///     });
/// }
/// ```
/// Feel free to share your use cases by pull requests!
#[macro_export]
macro_rules! metadata_only {
    // This rule handles structs:
    (
        {
            $(#[$attrs:meta])*
            $vis:vis
            struct $name:ident {
                $(
                    $(#[$field_attrs:meta])*
                    $field_vis:vis
                    $field:ident: $type:ty
                ),*
                $(,)?
            }
        },
        $(#[$metadata_attrs:meta])*
        $metadata_vis:vis
        struct $metadata_struct:ident: $metadata_type:ty
    ) => {
        $(#[$metadata_attrs])*
        $metadata_vis
        struct $metadata_struct {
            $(
                $(#[$field_attrs])*
                $field_vis
                $field: $metadata_type,
            )*
        }
    };
    // This rule handles tuple structs:
    (
        {
            $(#[$attrs:meta])*
            $vis:vis
            struct $name:ident (
                $(
                    $(#[$field_attrs:meta])*
                    $type:ty
                ),*
                $(,)?
            );
        },
        $(#[$metadata_attrs:meta])*
        $metadata_vis:vis
        struct $metadata_struct:ident: $metadata_type:ty
    ) => {
        $(#[$metadata_attrs])*
        $metadata_vis
        struct $metadata_struct (
            $(
                $(#[$field_attrs])*
                $metadata_type,
            )*
        );
        // TODO: force implement Metadata trait? (Extra work since can't use this macro in proc
        // macro anymore or otherwise duplicated impl)
    };
}

/// <section class="warning">
///     <b>This macro is for internal use only and is not part of the API.</b>
/// </section>
///
/// This rule is almost no-op except maintaining the macro hygigene of Rust.
/// See: <https://stackoverflow.com/a/75530574/13787084>
/// It receives the main struct and pastes it:
#[macro_export]
// #[deprecated = "For internal use only."]
macro_rules! put_struct {
    // This rule handles structs:
    ({
        $(#[$attrs:meta])*
        $vis:vis
        struct $name:ident {
            $(
                $(#[$field_attrs:meta])*
                $field_vis:vis
                $field:ident: $type:ty
            ),*
            $(,)?
        }
    }) => {
        $(#[$attrs])*
        $vis
        struct $name {
            $(
                $(#[$field_attrs])*
                $field_vis
                $field: $type,
            )*
        }
    };
    // This rule handles tuple structs:
    ({
        $(#[$attrs:meta])*
        $vis:vis
        struct $name:ident(
            $(
                $(#[$field_attrs:meta])*
                $type:ty
            ),*
            $(,)?
        );
    }) => {
        $(#[$attrs])*
        $vis
        struct $name(
            $(
                $(#[$field_attrs])*
                $type,
            )*
        );
    };
}
