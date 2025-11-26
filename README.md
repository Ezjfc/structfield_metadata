# Structfield Metadata
Rust macros for a very elementary metadata-like system in structs and tuple structs.

# Examples
This example declares a private main struct with two fields and a private metadata struct to
hold the multilingual description of the fields:
```rs
use structfield_metadata::metadata;

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
- Generic type parameters cannot be used in structs wrapped in `metadata!` or `metadata_only!` macro.
