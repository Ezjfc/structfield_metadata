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
/// #[macro_use] extern crate metadata_macro;
///
/// fn main() {
///     metadata!(
///         {
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
            $crate::metadata_only!(
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
/// This macro receives a main struct and a metadata struct. The main struct will be discarded,
/// its fields will change the type and be taken to the metadata struct.
/// # Examples
/// This example only declares a private metadata struct to hold the multilingual description of
/// two fields:
/// ```
/// #[macro_use] extern crate metadata_macro;
///
/// fn main() {
///     metadata_only!(
///         {
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
        // The magic pattern next to `$name` matches generic type parameters and struct lifetimes:
        // https://stackoverflow.com/a/61189128/13787084
            struct $name:ident $(< $( $lt:tt $( : $clt:tt $(+ $dlt:tt )* )? ),+ >)? {
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
        struct $metadata_struct $(<$($generics)*>)? {
            $(
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
            // The magic pattern next to `$name` matches generic type parameters and struct lifetimes:
            // https://stackoverflow.com/a/61189128/13787084
            struct $name:ident $(< $( $lt:tt $( : $clt:tt $(+ $dlt:tt )* )? ),+ >)? (
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
        $crate::put_tuple_discard_type!{
            $(#[$metadata_attrs])*
            $metadata_vis
            struct $metadata_struct (
                $({
                    discard: $type,
                    keep: $metadata_type,
                },)*
            );
        };
    };
}

/// <section class="warning">
///     <b><code>put_struct!</code> is for internal use only and is not part of the API.</b>
/// </section>
///
/// This macro is almost no-op except for maintaining the macro hygiene of Rust.
/// See: <https://stackoverflow.com/a/75530574/13787084>
/// It receives the main struct and pastes it:
#[macro_export]
// #[deprecated = "For internal use only."]
macro_rules! put_struct {
    // This rule handles structs:
    ({
        $(#[$attrs:meta])*
        $vis:vis
        // The magic pattern next to `$name` matches generic type parameters and struct lifetimes:
        // https://stackoverflow.com/a/61189128/13787084
        struct $name:ident $(< $( $lt:tt $( : $clt:tt $(+ $dlt:tt )* )? ),+ >)? {
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
        struct $name $(< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)? {
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
        // The magic pattern next to `$name` matches generic type parameters and struct lifetimes:
        // https://stackoverflow.com/a/61189128/13787084
        struct $name:ident $(< $( $lt:tt $( : $clt:tt $(+ $dlt:tt )* )? ),+ >)? (
            $(
                $(#[$field_attrs:meta])*
                $type:ty
            ),*
            $(,)?
        );
    }) => {
        $(#[$attrs])*
        $vis
        struct $name $(< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)? (
            $(
                $(#[$field_attrs])*
                $type,
            )*
        );
    };
}

/// <section class="warning">
///     <b><code>put_tuple_discard_type!</code> is for internal use only and is not part of the API.</b>
/// </section>
///
/// This macro a is workaround for the macro expansion error "attempted to repeat an expression
/// containing no syntax variables matched as repeating at this depth" for tuple structs. Since
/// there is no way for declarative macros to know the number of repetitions of metavariables,
/// putting a repetition to use must refer to at least one of its containing metavariables.
///
/// It receives specifications of a tuple struct but two different types for each field. The
/// `discard` type will be abandoned while the `keep` type will become the eventual type of that
/// field.
/// Note: in [`metadata_only!`], all `keep` types are equal when calling this macro.
///
#[macro_export]
macro_rules! put_tuple_discard_type {
    {
        $(#[$attrs:meta])*
        $vis:vis
        // The magic pattern next to `$name` matches generic type parameters and struct lifetimes:
        // https://stackoverflow.com/a/61189128/13787084
        struct $name:ident $(< $( $lt:tt $( : $clt:tt $(+ $dlt:tt )* )? ),+ >)? (
            $({
                discard: $discard:ty,
                keep: $keep:ty,
            }),*
            $(,)?
        );
    } => {
        $(#[$attrs])*
        $vis
        struct $name $(< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)? (
            $($keep,)*
        );
    };
}
