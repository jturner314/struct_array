//! Provides procedural macros that allow a struct to be easily converted
//! to/from arrays and slices.
//!
//! The `StructArrayDeref` and `StructArrayConvert` procedural macros implement
//! the necessary traits such that the struct can be easily converted to/from
//! arrays and slices. The `StructArray` procedural macro applies both
//! `StructArrayDeref` and `StructArrayConvert`. The macros work for normal
//! structs and tuple structs. The macros check that the struct has the
//! following properties:
//!
//!   * all the fields must be public (because they are exposed in
//!     arrays/slices created by the conversion functions)
//!   * all the fields must have the same type
//!   * the struct must have at least one field
//!   * the struct must have the `#[repr(C)]` attribute
//!
//! # Example
//!
//! ```ignore
//! #[macro_use]
//! extern crate struct_array;
//!
//! /// Example struct array.
//! #[derive(Clone,Debug,PartialEq,StructArray)]
//! #[repr(C)]
//! struct Example {
//!     pub x: u32,
//!     pub y: u32,
//! }
//!
//! fn main() {
//!     // Deref as an array.
//!     {
//!         let example = Example { x: 42, y: 56 };
//!         let array: [u32; 2] = *example;
//!         assert_eq!(array, [42, 56]);
//!     }
//!
//!     // Index (via derefing as an array).
//!     {
//!         let mut example = Example { x: 42, y: 56 };
//!         example[1] = 23;
//!         assert_eq!(example, Example { x: 42, y: 23 });
//!     }
//!
//!     // Convert into an array.
//!     {
//!         let example = Example { x: 42, y: 56 };
//!         let array: [u32; 2] = example.into();
//!         assert_eq!(array, [42, 56]);
//!     }
//!
//!     // Convert from an array.
//!     {
//!         let array = [42, 56];
//!         let example: Example = array.into();
//!         assert_eq!(example, Example { x: 42, y: 56 });
//!     }
//!
//!     // Convert a ref into a slice.
//!     {
//!         let example = &Example { x: 42, y: 56 };
//!         let slice: &[u32] = example.into();
//!         assert_eq!(slice, &[42, 56]);
//!     }
//!
//!     // Convert a slice into a ref.
//!     {
//!         let slice = &[42, 56][..];
//!         let example: &Example = slice.into();
//!         assert_eq!(example, &Example { x: 42, y: 56 });
//!     }
//! }
//!
//! ```
//!
//! # Trait implementations
//!
//! Deriving `StructArray` for a struct causes it to implement all the methods
//! provided by `StructArrayDeref` and `StructArrayConvert`.
//!
//! Deriving `StructArrayDeref` for a struct `Foo` causes it to implement:
//!
//! * `Deref<Target=[T; len]> for Foo`
//! * `DerefMut<Target=[T; len]> for Foo`
//!
//! Deriving `StructArrayConvert` for a struct `Foo` creates implementations
//! for the following:
//!
//! * `From<Foo> for [T; len]`
//! * `From<[T; len]> for Foo`
//! * `From<&Foo> for &[T; len]`
//! * `AsRef<[T; len] for Foo`
//! * `From<&[T; len] for &Foo`
//! * `AsRef<Foo> for [T; len]`
//! * `From<&mut Foo> for &mut [T; len]`
//! * `AsMut<[T; len] for Foo`
//! * `From<&mut [T; len] for &mut Foo`
//! * `AsMut<Foo> for [T; len]`
//! * `From<&Foo> for &[T]`
//! * `AsRef<[T]> for Foo`
//! * `From<&[T]> for &Foo`
//! * `AsRef<Foo> for [T]`
//! * `From<&mut Foo> for &mut [T]`
//! * `AsMut<[T]> for Foo`
//! * `From<&mut [T]> for &mut Foo`
//! * `AsMut<Foo> for [T]`
//!
//! Note that converting from a slice will panic if the `len()` of the slice
//! does not must match the number of fields in the struct.

#![recursion_limit = "500"]

extern crate proc_macro;
use proc_macro::TokenStream;

extern crate syn;

#[macro_use]
extern crate quote;
use quote::ToTokens;

/// Errors in the input to one of the macros.
#[derive(Clone,Debug,Eq,PartialEq)]
enum MacroInputError {
    ZeroFields,
    NonpublicField,
    DifferingFieldTypes,
    NotStruct,
    NotReprC,
}

impl std::fmt::Display for MacroInputError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            MacroInputError::ZeroFields => write!(f, "the struct must have at least one field"),
            MacroInputError::NonpublicField => write!(f, "all fields in the struct must be public"),
            MacroInputError::DifferingFieldTypes => write!(f, "all fields in the struct must have the same type"),
            MacroInputError::NotStruct => write!(f, "the type must be a struct (or tuple struct), not an enum"),
            MacroInputError::NotReprC => write!(f, "the struct must have the #[repr(C)] attribute"),
        }
    }
}

impl std::error::Error for MacroInputError {
    fn description(&self) -> &str {
        match *self {
            MacroInputError::ZeroFields => "struct had no fields",
            MacroInputError::NonpublicField => "struct had at least one nonpublic field",
            MacroInputError::DifferingFieldTypes => "struct had fields of differing types",
            MacroInputError::NotStruct => "input was not a struct",
            MacroInputError::NotReprC => "struct was missing the #[repr(C)] attribute",
        }
    }

    fn cause(&self) -> Option<&std::error::Error> {
        None
    }
}

/// Relevant information about the struct from the macro input.
struct StructInfo<'a> {
    name: &'a syn::Ident,
    generics: &'a syn::Generics,
    field_type: &'a syn::Ty,
    field_count: usize,
}

/// Extracts the relevant information from the macro input and checks that the
/// struct meets the requirements for the macros.
fn parse_input<'a>(ast: &'a syn::MacroInput) -> Result<StructInfo<'a>, MacroInputError> {
    let repr_c =
        syn::MetaItem::List("repr".into(),
                            vec![syn::NestedMetaItem::MetaItem(syn::MetaItem::Word("C".into()))]);
    if !ast.attrs.iter().any(|attr| attr.value == repr_c) {
        Err(MacroInputError::NotReprC)
    } else {
        match ast.body {
            syn::Body::Enum(_) => Err(MacroInputError::NotStruct),
            syn::Body::Struct(ref data) => {
                let field_type = &data.fields().first().ok_or(MacroInputError::ZeroFields)?.ty;
                if data.fields().iter().any(|field| field.vis != syn::Visibility::Public) {
                    Err(MacroInputError::NonpublicField)
                } else if data.fields().iter().any(|field| field.ty != *field_type) {
                    Err(MacroInputError::DifferingFieldTypes)
                } else {
                    Ok(StructInfo {
                        name: &ast.ident,
                        generics: &ast.generics,
                        field_type: field_type,
                        field_count: data.fields().len(),
                    })
                }
            }
        }
    }
}

/// Implements derive of `StructArray`.
///
/// This function is called by the Rust compiler when compiling code that uses
/// `#[derive(StructArray)]`.
#[proc_macro_derive(StructArray)]
pub fn derive_struct_array(input: TokenStream) -> TokenStream {
    let source = input.to_string();

    // Parse the string representation into a syntax tree.
    let ast = syn::parse_macro_input(&source).unwrap();

    // Check the struct and get the necessary info.
    let struct_info = parse_input(&ast).unwrap_or_else(|err| {
        panic!(format!("Error expanding #[derive(StructArray)]: {}", err))
    });

    // Build the output.
    let mut expanded = quote::Tokens::new();
    impl_struct_array_deref(&struct_info).to_tokens(&mut expanded);
    impl_struct_array_convert(&struct_info).to_tokens(&mut expanded);

    // Return the generated impl as a TokenStream.
    expanded.parse().unwrap()
}

/// Implements derive of `StructArrayDeref`.
///
/// This function is called by the Rust compiler when compiling code that uses
/// `#[derive(StructArrayDeref)]`.
#[proc_macro_derive(StructArrayDeref)]
pub fn derive_struct_array_deref(input: TokenStream) -> TokenStream {
    let source = input.to_string();

    // Parse the string representation into a syntax tree.
    let ast = syn::parse_macro_input(&source).unwrap();

    // Check the struct and get the necessary info.
    let struct_info = parse_input(&ast).unwrap_or_else(|err| {
        panic!(format!("Error expanding #[derive(StructArrayDeref)]: {}", err))
    });

    // Build the output.
    let expanded = impl_struct_array_deref(&struct_info);

    // Return the generated impl as a TokenStream.
    expanded.parse().unwrap()
}

fn impl_struct_array_deref(struct_info: &StructInfo) -> quote::Tokens {
    let StructInfo { name, generics, field_type, field_count } = *struct_info;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    quote! {
        impl #impl_generics ::std::ops::Deref for #name #ty_generics #where_clause {
            type Target = [#field_type; #field_count];

            fn deref(&self) -> &[#field_type; #field_count] {
                unsafe {
                    &*(self as *const #name as *const [#field_type; #field_count])
                }
            }
        }

        impl #impl_generics ::std::ops::DerefMut for #name #ty_generics #where_clause {
            fn deref_mut(&mut self) -> &mut [#field_type; #field_count] {
                unsafe {
                    &mut *(self as *mut #name as *mut [#field_type; #field_count])
                }
            }
        }
    }
}

/// Implements derive of `StructArrayConvert`.
///
/// This function is called by the Rust compiler when compiling code that uses
/// `#[derive(StructArrayConvert)]`.
#[proc_macro_derive(StructArrayConvert)]
pub fn derive_struct_array_convert(input: TokenStream) -> TokenStream {
    let source = input.to_string();

    // Parse the string representation into a syntax tree.
    let ast = syn::parse_macro_input(&source).unwrap();

    // Check the struct and get the necessary info.
    let struct_info = parse_input(&ast).unwrap_or_else(|err| {
        panic!(format!("Error expanding #[derive(StructArrayConvert)]: {}", err))
    });

    // Build the output.
    let expanded = impl_struct_array_convert(&struct_info);

    // Return the generated impl as a TokenStream.
    expanded.parse().unwrap()
}

fn impl_struct_array_convert(struct_info: &StructInfo) -> quote::Tokens {
    let StructInfo { name, generics, field_type, field_count } = *struct_info;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    let from_slice_doc = format!("
Performs the conversion.

# Panics

Panics if the `len()` of the slice is not {}.
", field_count);
    quote! {
        impl #impl_generics From<#name> for [#field_type; #field_count] #ty_generics #where_clause {
            fn from(s: #name) -> [#field_type; #field_count] {
                unsafe {
                    ::std::mem::transmute(s)
                }
            }
        }

        impl #impl_generics From<[#field_type; #field_count]> for #name #ty_generics #where_clause {
            fn from(array: [#field_type; #field_count]) -> #name {
                unsafe {
                    ::std::mem::transmute(array)
                }
            }
        }

        impl<'a> #impl_generics From<&'a #name> for &'a [#field_type; #field_count] #ty_generics #where_clause {
            fn from(s: &'a #name) -> &'a [#field_type; #field_count] {
                unsafe {
                    &*(s as *const #name as *const [#field_type; #field_count])
                }
            }
        }

        impl #impl_generics ::std::convert::AsRef<[#field_type; #field_count]> for #name #ty_generics #where_clause {
            fn as_ref(&self) -> &[#field_type; #field_count] {
                unsafe {
                    &*(self as *const #name as *const [#field_type; #field_count])
                }
            }
        }

        impl<'a> #impl_generics From<&'a [#field_type; #field_count]> for &'a #name #ty_generics #where_clause {
            fn from(array: &[#field_type; #field_count]) -> &#name {
                unsafe {
                    &*(array as *const [#field_type; #field_count] as *const #name)
                }
            }
        }

        impl #impl_generics ::std::convert::AsRef<#name> for [#field_type; #field_count] #ty_generics #where_clause {
            fn as_ref(&self) -> &#name {
                unsafe {
                    &*(self as *const [#field_type; #field_count] as *const #name)
                }
            }
        }

        impl<'a> #impl_generics From<&'a mut #name> for &'a mut [#field_type; #field_count] #ty_generics #where_clause {
            fn from(s: &'a mut #name) -> &'a mut [#field_type; #field_count] {
                unsafe {
                    &mut *(s as *mut #name as *mut [#field_type; #field_count])
                }
            }
        }

        impl #impl_generics ::std::convert::AsMut<[#field_type; #field_count]> for #name #ty_generics #where_clause {
            fn as_mut(&mut self) -> &mut [#field_type; #field_count] {
                unsafe {
                    &mut *(self as *mut #name as *mut [#field_type; #field_count])
                }
            }
        }

        impl<'a> #impl_generics From<&'a mut [#field_type; #field_count]> for &'a mut #name #ty_generics #where_clause {
            fn from(array: &mut [#field_type; #field_count]) -> &mut #name {
                unsafe {
                    &mut *(array as *mut [#field_type; #field_count] as *mut #name)
                }
            }
        }

        impl #impl_generics ::std::convert::AsMut<#name> for [#field_type; #field_count] #ty_generics #where_clause {
            fn as_mut(&mut self) -> &mut #name {
                unsafe {
                    &mut *(self as *mut [#field_type; #field_count] as *mut #name)
                }
            }
        }

        impl<'a> #impl_generics From<&'a #name> for &'a [#field_type] #ty_generics #where_clause {
            fn from(s: &'a #name) -> &'a [#field_type] {
                unsafe {
                    ::std::slice::from_raw_parts(s as *const #name as *const #field_type, #field_count)
                }
            }
        }

        impl #impl_generics ::std::convert::AsRef<[#field_type]> for #name #ty_generics #where_clause {
            fn as_ref(&self) -> &[#field_type] {
                unsafe {
                    ::std::slice::from_raw_parts(self as *const #name as *const #field_type, #field_count)
                }
            }
        }

        impl<'a> #impl_generics From<&'a [#field_type]> for &'a #name #ty_generics #where_clause {
            #[doc=#from_slice_doc]
            fn from(slice: &'a [#field_type]) -> &'a #name {
                assert_eq!(slice.len(), #field_count);
                unsafe {
                    &*(slice.as_ptr() as *const #name)
                }
            }
        }

        impl #impl_generics ::std::convert::AsRef<#name> for [#field_type] #ty_generics #where_clause {
            #[doc=#from_slice_doc]
            fn as_ref(&self) -> &#name {
                assert_eq!(self.len(), #field_count);
                unsafe {
                    &*(self.as_ptr() as *const #name)
                }
            }
        }

        impl<'a> #impl_generics From<&'a mut #name> for &'a mut [#field_type] #ty_generics #where_clause {
            fn from(s: &'a mut #name) -> &'a mut [#field_type] {
                unsafe {
                    ::std::slice::from_raw_parts_mut(s as *mut #name as *mut #field_type, #field_count)
                }
            }
        }

        impl #impl_generics ::std::convert::AsMut<[#field_type]> for #name #ty_generics #where_clause {
            fn as_mut(&mut self) -> &mut [#field_type] {
                unsafe {
                    ::std::slice::from_raw_parts_mut(self as *mut #name as *mut #field_type, #field_count)
                }
            }
        }

        impl<'a> #impl_generics From<&'a mut [#field_type]> for &'a mut #name #ty_generics #where_clause {
            #[doc=#from_slice_doc]
            fn from(slice: &'a mut [#field_type]) -> &'a mut #name {
                assert_eq!(slice.len(), #field_count);
                unsafe {
                    &mut *(slice.as_mut_ptr() as *mut #name)
                }
            }
        }

        impl #impl_generics ::std::convert::AsMut<#name> for [#field_type] #ty_generics #where_clause {
            #[doc=#from_slice_doc]
            fn as_mut(&mut self) -> &mut #name {
                assert_eq!(self.len(), #field_count);
                unsafe {
                    &mut *(self.as_mut_ptr() as *mut #name)
                }
            }
        }
    }
}
