use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields, parse_macro_input};

/// A procedural macro that implements the `Deref` trait for unit structs.
///
/// This macro will:
/// 1. Parse the input token stream as a derive input
/// 2. Verify that the input is a unit struct
/// 3. Extract the inner type
/// 4. Generate the implementation of `Deref` for the struct
///
/// # Example
///
/// ```rust
/// use std::ops::Deref;
/// use deref_macro::DerefImpl;
///
/// #[derive(DerefImpl)]
/// struct Wrapper(String);
///
/// fn main() {
///     let wrapper = Wrapper(String::from("Hello, world!"));
///     assert_eq!(&*wrapper, "Hello, world!");
/// }
/// ```
#[proc_macro_derive(DerefImpl)]
pub fn derive_deref(input: TokenStream) -> TokenStream {
  // Parse the input tokens into a syntax tree
  let input = parse_macro_input!(input as DeriveInput);

  // Get the name of the struct
  let struct_name = &input.ident;

  // Extract the inner type from the struct
  let inner_type = match &input.data {
    Data::Struct(data_struct) => {
      match &data_struct.fields {
        // For tuple struct with a single field
        Fields::Unnamed(fields) if fields.unnamed.len() == 1 => {
          let first_field = fields.unnamed.first().unwrap();
          &first_field.ty
        }
        // error the rest out. We only want to support tuple structs
        _ => {
          return syn::Error::new_spanned(
            &input.ident,
            "DerefImpl can only be used on tuple structs with exactly one field",
          )
          .to_compile_error()
          .into();
        }
      }
    }
    _ => {
      return syn::Error::new_spanned(&input.ident, "DerefImpl can only be used on structs")
        .to_compile_error()
        .into();
    }
  };

  // Generate the implementation
  let output = quote! {
    impl std::ops::Deref for #struct_name {
      type Target = #inner_type;

      fn deref(&self) -> &Self::Target {
        &self.0
      }
    }
  };

  // Return the generated code
  output.into()
}

/// A procedural macro that implements the `DerefMut` trait for unit structs.
///
/// This macro will:
/// 1. Parse the input token stream as a derive input
/// 2. Verify that the input is a unit struct
/// 3. Extract the inner type
/// 4. Generate the implementation of `DerefMut` for the struct
///
/// # Example
///
/// ```rust
/// use std::ops::DerefMut;
/// use deref_macro::DerefImpl;
/// use deref_macro::DerefMutImpl;
///
/// #[derive(DerefImpl, DerefMutImpl)]
/// struct Wrapper(String);
///
/// fn main() {
///     let mut wrapper = Wrapper(String::from("Hello, world!"));
///     wrapper = Wrapper("Hello Rust!".to_string());
///     assert_eq!(*wrapper, "Hello Rust!");
/// }
/// ```
#[proc_macro_derive(DerefMutImpl)]
pub fn derive_deref_mut(input: TokenStream) -> TokenStream {
  // Parse the input tokens into a syntax tree
  let input = parse_macro_input!(input as DeriveInput);

  // Get the name of the struct
  let struct_name = &input.ident;

  // Generate the implementation
  let output = quote! {
    impl std::ops::DerefMut for #struct_name {
      fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
      }
    }
  };

  // Return the generated code
  output.into()
}
