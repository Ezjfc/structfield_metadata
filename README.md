# Metadata Macro
Rust macros for a very elementary metadata-like system in structs and tuple structs.

[![Crates.io Version](https://img.shields.io/crates/v/metadata_macro)](https://crates.io/crates/metadata_macro)
[![docs.rs](https://img.shields.io/docsrs/metadata_macro)](https://docs.rs/metadata_macro/latest/)

# Examples
This example declares a private main struct with two fields and a private metadata struct to
hold the multilingual description of the fields:
```rs
use metadata_macro::metadata;

metadata!({
        #[derive(Default, PartialEq, Debug)]
        struct YourStruct {
            field_a: bool,
            field_b: usize,
        }
    },
    #[derive(PartialEq, Debug)] struct YourStructDescription: &'static str,
);

impl Default for YourStructDescription {
    fn default() -> Self {
        Self {
            field_a: "Describes field_a",
            field_b: "Describes field_b",
        }
    }
}

impl YourStructDescription {
    fn i18n_some_language() -> Self {
        Self {
            field_a: "Describes field_a in other language",
            field_b: "Describes field_b in other language",
        }
    }
}
```

# Known issues
- Generic type parameters cannot be used in structs wrapped in the macros.
