pub mod struct_level;
pub mod field_level;

use syn::{punctuated::Punctuated, Token};

type ArgsList<T> = Punctuated::<T, Token![,]>;
