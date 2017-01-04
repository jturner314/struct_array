//! Provides a procedural macro that allows a struct to be easily converted
//! to/from arrays and slices.
//!
//! The `StructArray` derive macro implements the necessary traits such that
//! the struct can be easily converted to/from arrays and slices. The macro
//! works for normal structs and tuple structs. The struct must have the
//! following properties:
//!
//!   * all the fields must be public (indicated with the `pub` keyword)
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
//!     /// x member
//!     pub x: u32,
//!     /// y member
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
//!     // Deref as a slice (via derefing as an array).
//!     {
//!         let example = Example { x: 42, y: 56 };
//!         let slice: &[u32] = &*example;
//!         assert_eq!(slice, &[42, 56]);
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
//!     // Convert from an array to a struct.
//!     {
//!         let array = [42, 56];
//!         let example: Example = array.into();
//!         assert_eq!(example, Example { x: 42, y: 56 });
//!     }
//!
//!     // Convert from a slice to a struct reference.
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
//! Deriving `StructArray` for a struct `Foo` causes it to implement:
//!
//! * `Deref<Target=[T; len]> for Foo`
//! * `DerefMut<Target=[T; len]> for Foo`
//! * `Into<[T; len] for Foo`
//! * `From<[T; len]> for Foo`
//! * `Into<&[T; len] for &Foo`
//! * `AsRef<[T; len] for Foo`
//! * `From<&[T; len] for &Foo`
//! * `AsRef<Foo> for [T; len]`
//! * `Into<&mut [T; len] for &mut Foo`
//! * `AsMut<[T; len] for Foo`
//! * `From<&mut [T; len] for &mut Foo`
//! * `AsMut<Foo> for [T; len]`
//! * `Into<&[T]> for &Foo`
//! * `AsRef<[T]> for Foo`
//! * `From<&[T]> for &Foo`
//! * `AsRef<Foo> for [T]`
//! * `Into<&mut [T]> for &mut Foo`
//! * `AsMut<[T]> for Foo`
//! * `From<&mut [T]> for &mut Foo`
//! * `AsMut<Foo> for [T]`
//!
//! Note that converting from a slice will panic if the `len()` of the slice
//! does not must match the `len` specified for the struct.

#![feature(proc_macro, proc_macro_lib)]

#![recursion_limit = "500"]

extern crate proc_macro;
use proc_macro::TokenStream;

extern crate syn;

#[macro_use]
extern crate quote;

/// Implements derive of `StructArray`.
///
/// This function is called by the Rust compiler when compiling code that uses
/// `#[derive(StructArray)]`.
#[proc_macro_derive(StructArray)]
pub fn derive_struct_array(input: TokenStream) -> TokenStream {
    let source = input.to_string();

    // Parse the string representation into a syntax tree
    let ast = syn::parse_macro_input(&source).unwrap();

    // Build the output
    let expanded = impl_struct_array(&ast);

    // Return the generated impl as a TokenStream
    expanded.parse().unwrap()
}

fn impl_struct_array(ast: &syn::MacroInput) -> quote::Tokens {
    let name = &ast.ident;
    let (field_type, field_count) = match ast.body {
        syn::Body::Struct(ref data) => {
            let field_type = &data.fields().first().expect(
                "#[derive(StructArray)] can only be used with structs that have at least one field"
            ).ty;
            for field in data.fields() {
                if field.vis != syn::Visibility::Public {
                    panic!("#[derive(StructArray)] can only used if all the fields in the struct are public")
                }
                if field.ty != *field_type {
                    panic!("#[derive(StructArray)] can only used with structs that have identical field types")
                }
            }
            (field_type, data.fields().len())
        },
        syn::Body::Enum(_) => panic!("#[derive(StructArray)] can only be used with structs"),
    };
    let repr_c = syn::MetaItem::List("repr".into(), vec![syn::NestedMetaItem::MetaItem(syn::MetaItem::Word("C".into()))]);
    if !ast.attrs.iter().any(|attr| attr.value == repr_c) {
        panic!("#[derive(StructArray)] can only be used with structs that are #[repr(C)]");
    }

    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();
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

        impl #impl_generics Into<[#field_type; #field_count]> for #name #ty_generics #where_clause {
            fn into(self) -> [#field_type; #field_count] {
                unsafe {
                    ::std::mem::transmute(self)
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

        impl<'a> #impl_generics Into<&'a [#field_type; #field_count]> for &'a #name #ty_generics #where_clause {
            fn into(self) -> &'a [#field_type; #field_count] {
                unsafe {
                    &*(self as *const #name as *const [#field_type; #field_count])
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

        impl<'a> #impl_generics Into<&'a mut [#field_type; #field_count]> for &'a mut #name #ty_generics #where_clause {
            fn into(self) -> &'a mut [#field_type; #field_count] {
                unsafe {
                    &mut *(self as *mut #name as *mut [#field_type; #field_count])
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

        impl<'a> #impl_generics Into<&'a [#field_type]> for &'a #name #ty_generics #where_clause {
            fn into(self) -> &'a [#field_type] {
                unsafe {
                    ::std::slice::from_raw_parts(self as *const #name as *const #field_type, #field_count)
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
            fn from(slice: &'a [#field_type]) -> &'a #name {
                assert!(slice.len() == #field_count);
                unsafe {
                    &*(slice.as_ptr() as *const #name)
                }
            }
        }

        impl #impl_generics ::std::convert::AsRef<#name> for [#field_type] #ty_generics #where_clause {
            fn as_ref(&self) -> &#name {
                assert!(self.len() == #field_count);
                unsafe {
                    &*(self.as_ptr() as *const #name)
                }
            }
        }

        impl<'a> #impl_generics Into<&'a mut [#field_type]> for &'a mut #name #ty_generics #where_clause {
            fn into(self) -> &'a mut [#field_type] {
                unsafe {
                    ::std::slice::from_raw_parts_mut(self as *mut #name as *mut #field_type, #field_count)
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
            fn from(slice: &'a mut [#field_type]) -> &'a mut #name {
                assert!(slice.len() == #field_count);
                unsafe {
                    &mut *(slice.as_mut_ptr() as *mut #name)
                }
            }
        }

        impl #impl_generics ::std::convert::AsMut<#name> for [#field_type] #ty_generics #where_clause {
            fn as_mut(&mut self) -> &mut #name {
                assert!(self.len() == #field_count);
                unsafe {
                    &mut *(self.as_mut_ptr() as *mut #name)
                }
            }
        }
    }
}
