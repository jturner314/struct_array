//! A generator for structs that can easily be converted to/from arrays and
//! slices.

/// The `struct_array!` macro generates a `struct` that can be easily converted
/// to/from arrays and slices.
///
/// The format of the macro is similar to a normal struct definition, as shown
/// in the example below. The only differences are an annotation after the
/// struct name for the member type and number of members, and the lack of type
/// annotations on individual members.
///
/// The members of the struct must all be the same type, and the **number of
/// members must be specified correctly**. As long as the number of members is
/// specified correctly, I think everything is safe.
///
/// # Example
///
/// ```
/// #[macro_use]
/// extern crate struct_array;
///
/// struct_array! {
///     /// Example struct array.
///     #[derive(Clone,Debug,PartialEq)]
///     struct Example[u32; 2] {
///         /// x member
///         pub x,
///         /// y member
///         pub y,
///     }
/// }
///
/// fn main() {
///     // Deref as an array.
///     {
///         let example = Example { x: 42, y: 56 };
///         let array: &[u32; 2] = &example;
///         assert_eq!(array, &[42, 56]);
///     }
///
///     // Deref as a slice (via derefing as an array).
///     {
///         let example = Example { x: 42, y: 56 };
///         let slice: &[u32] = &*example;
///         assert_eq!(slice, &[42, 56]);
///     }
///
///     // Convert into an array.
///     {
///         let example = Example { x: 42, y: 56 };
///         let array: [u32; 2] = example.into();
///         assert_eq!(array, [42, 56]);
///     }
///
///     // Convert from an array to a struct.
///     {
///         let array = [42, 56];
///         let example: Example = array.into();
///         assert_eq!(example, Example { x: 42, y: 56 });
///     }
///
///     // Convert from a slice to a struct reference.
///     {
///         let slice = &[42, 56][..];
///         let example: &Example = slice.into();
///         assert_eq!(example, &Example { x: 42, y: 56 });
///     }
/// }
///
/// ```
///
/// # Attributes
///
/// The generated struct is `repr(C)`. Additional attributes can be attached to
/// the generated `struct` as shown in the example.
///
/// # Visibility
///
/// Members of the generated struct are always `pub`, but the struct itself is
/// not `pub` by default. The struct definition can be made `pub` by adding
/// `pub` before `struct`:
///
/// ```ignore
/// #[macro_use]
/// extern crate struct_array;
///
/// mod example {
///     struct_array! {
///         pub struct Example1[u32; 2] {
///             pub x,
///             pub y,
///         }
///     }
///
///     struct_array! {
///         struct Example2[u32; 2] {
///             pub x,
///             pub y,
///         }
///     }
/// }
///
/// fn main() {
///     let example1 = example::Example1 { x: 42, y: 56 };
///     let example2 = example::Example2 { x: 42, y: 56 }; // error: struct `Example2` is private.
/// }
/// ```
///
/// # Trait implementations
///
/// The struct generated by `struct_array! { struct Foo[T; len] { .. } }`
/// implements:
///
/// * `Deref<Target=[T; len]> for Foo`
/// * `DerefMut<Target=[T; len]> for Foo`
/// * `Into<[T; len] for Foo`
/// * `From<[T; len]> for Foo`
/// * `Into<&[T; len] for &Foo`
/// * `From<&[T; len] for &Foo`
/// * `Into<&mut [T; len] for &mut Foo`
/// * `From<&mut [T; len] for &mut Foo`
/// * `Into<&[T]> for &Foo`
/// * `From<&[T]> for &Foo`
/// * `Into<&mut [T]> for &mut Foo`
/// * `From<&mut [T]> for &mut Foo`
///
/// Note that converting from a slice will panic if the `len()` of the slice
/// does not must match the `len` specified for the struct. You can derive or
/// implement additional traits as for normal structs.
#[macro_export]
macro_rules! struct_array {
    (
        $( #[$attr:meta] )*
        pub struct $name:ident[$member_type:ident; $member_count:expr] {
            $(
                $( #[$member_attr:meta] )*
                pub $member:ident
            ),+
        }
    ) => {
        $( #[$attr] )*
        #[repr(C)]
        pub struct $name {
            $(
                $( #[$member_attr] )*
                pub $member: $member_type,
            )+
        }

        struct_array! {
            @_impl struct
                $name[$member_type; $member_count] {
                    $( $member ),+
                }
        }
    };
    (
        $( #[$attr:meta] )*
        struct $name:ident[$member_type:ident; $member_count:expr] {
            $(
                $( #[$member_attr:meta] )*
                pub $member:ident
            ),+
        }
    ) => {
        $( #[$attr] )*
        #[repr(C)]
        struct $name {
            $(
                $( #[$member_attr] )*
                pub $member: $member_type,
            )+
        }

        struct_array! {
            @_impl struct
                $name[$member_type; $member_count] {
                    $( $member ),+
                }
        }
    };
    (
        $( #[$attr:meta] )*
        pub struct $name:ident[$member_type:ident; $member_count:expr] {
            $(
                $( #[$member_attr:meta] )*
                pub $member:ident
            ),+,
        }
    ) => {
        $( #[$attr] )*
        #[repr(C)]
        pub struct $name {
            $(
                $( #[$member_attr] )*
                pub $member: $member_type,
            )+
        }

        struct_array! {
            @_impl struct
                $name[$member_type; $member_count] {
                    $( $member ),+
                }
        }
    };
    (
        $( #[$attr:meta] )*
        struct $name:ident[$member_type:ident; $member_count:expr] {
            $(
                $( #[$member_attr:meta] )*
                pub $member:ident
            ),+,
        }
    ) => {
        $( #[$attr] )*
        #[repr(C)]
        struct $name {
            $(
                $( #[$member_attr] )*
                pub $member: $member_type,
            )+
        }

        struct_array! {
            @_impl struct
                $name[$member_type; $member_count] {
                    $( $member ),+
                }
        }
    };
    (
        @_impl struct $name:ident[$member_type:ident; $member_count:expr] {
            $( $member:ident ),+
        }
    ) => {
        impl ::std::ops::Deref for $name {
            type Target = [$member_type; $member_count];

            fn deref(&self) -> &[$member_type; $member_count] {
                unsafe {
                    &*(self as *const $name as *const [$member_type; $member_count])
                }
            }
        }

        impl ::std::ops::DerefMut for $name {
            fn deref_mut(&mut self) -> &mut [$member_type; $member_count] {
                unsafe {
                    &mut *(self as *mut $name as *mut [$member_type; $member_count])
                }
            }
        }

        impl Into<[$member_type; $member_count]> for $name {
            fn into(self) -> [$member_type; $member_count] {
                unsafe {
                    ::std::mem::transmute(self)
                }
            }
        }

        impl From<[$member_type; $member_count]> for $name {
            fn from(array: [$member_type; $member_count]) -> $name {
                unsafe {
                    ::std::mem::transmute(array)
                }
            }
        }

        impl<'a> Into<&'a [$member_type; $member_count]> for &'a $name {
            fn into(self) -> &'a [$member_type; $member_count] {
                unsafe {
                    &*(self as *const $name as *const [$member_type; $member_count])
                }
            }
        }

        impl<'a> From<&'a [$member_type; $member_count]> for &'a $name {
            fn from(array: &[$member_type; $member_count]) -> &$name {
                unsafe {
                    &*(array as *const [$member_type; $member_count] as *const $name)
                }
            }
        }

        impl<'a> Into<&'a mut [$member_type; $member_count]> for &'a mut $name {
            fn into(self) -> &'a mut [$member_type; $member_count] {
                unsafe {
                    &mut *(self as *mut $name as *mut [$member_type; $member_count])
                }
            }
        }

        impl<'a> From<&'a mut [$member_type; $member_count]> for &'a mut $name {
            fn from(array: &mut [$member_type; $member_count]) -> &mut $name {
                unsafe {
                    &mut *(array as *mut [$member_type; $member_count] as *mut $name)
                }
            }
        }

        impl<'a> Into<&'a [$member_type]> for &'a $name {
            fn into(self) -> &'a [$member_type] {
                unsafe {
                    ::std::slice::from_raw_parts(self as *const $name as *const $member_type, $member_count)
                }
            }
        }

        impl<'a> From<&'a [$member_type]> for &'a $name {
            fn from(slice: &'a [$member_type]) -> &'a $name {
                assert!(slice.len() == $member_count);
                unsafe {
                    &*(slice.as_ptr() as *const $name)
                }
            }
        }

        impl<'a> Into<&'a mut [$member_type]> for &'a mut $name {
            fn into(self) -> &'a mut [$member_type] {
                unsafe {
                    ::std::slice::from_raw_parts_mut(self as *mut $name as *mut $member_type, $member_count)
                }
            }
        }

        impl<'a> From<&'a mut [$member_type]> for &'a mut $name {
            fn from(slice: &'a mut [$member_type]) -> &'a mut $name {
                assert!(slice.len() == $member_count);
                unsafe {
                    &mut *(slice.as_mut_ptr() as *mut $name)
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    struct_array! {
        /// Example struct array.
        #[derive(Clone,Debug,PartialEq)]
        struct Example[u32; 2] {
            /// x member
            pub x,
            /// y member
            pub y,
        }
    }

    #[test]
    fn test_deref() {
        let foo = Example {
            x: 0,
            y: 1,
        };
        let bar: &[u32; 2] = &foo;
        assert_eq!(bar, &[0, 1]);
    }

    #[test]
    fn test_deref_mut() {
        let mut foo = Example {
            x: 0,
            y: 1,
        };
        {
            let bar: &mut [u32; 2] = &mut foo;
            bar[1] = 2;
            assert_eq!(bar, &[0, 2]);
        }
        assert_eq!(foo, Example { x: 0, y: 2 });
    }

    #[test]
    fn test_into_array() {
        let foo = Example { x: 0, y: 1 };
        let bar: [u32; 2] = foo.into();
        assert_eq!(bar, [0, 1]);
    }

    #[test]
    fn test_from_array() {
        let foo = [0, 1];
        let mut bar: Example = foo.into();
        bar.y = 2;
        assert_eq!(bar, Example { x: 0, y: 2 });
    }

    #[test]
    fn test_into_array_ref() {
        let foo = &Example { x: 0, y: 1 };
        let bar: &[u32; 2] = foo.into();
        assert_eq!(bar, &[0, 1]);
    }

    #[test]
    fn test_from_array_ref() {
        let foo = &[0, 1];
        let bar: &Example = foo.into();
        assert_eq!(bar, &Example { x: 0, y: 1 });
    }

    #[test]
    fn test_into_array_ref_mut() {
        let mut foo = &mut Example { x: 0, y: 1 };
        {
            let mut bar: &mut [u32; 2] = foo.into();
            bar[1] = 2;
            assert_eq!(bar, &mut [0, 2]);
        }
        assert_eq!(foo, &mut Example { x: 0, y: 2 });
    }

    #[test]
    fn test_from_array_ref_mut() {
        let foo = &mut [0, 1];
        {
            let mut bar: &mut Example = foo.into();
            bar.y = 2;
            assert_eq!(bar, &mut Example { x: 0, y: 2 });
        }
        assert_eq!(foo, &mut [0, 2]);
    }

    #[test]
    fn test_into_slice() {
        let foo = Example { x: 0, y: 1 };
        let bar: &[u32] = (&foo).into();
        assert_eq!(bar, [0, 1]);
    }

    #[test]
    fn test_from_slice() {
        let foo = [0, 1];
        let bar: &[u32] = &foo;
        let baz: &Example = bar.into();
        assert_eq!(baz, &Example { x: 0, y: 1 });
    }

    #[test]
    fn test_into_mut_slice() {
        let mut foo = Example { x: 0, y: 1 };
        {
            let bar: &mut [u32] = (&mut foo).into();
            bar[1] = 2;
            assert_eq!(bar, &mut [0, 2]);
        }
        assert_eq!(foo, Example { x: 0, y: 2 });
    }

    #[test]
    fn test_from_mut_slice() {
        let mut foo = [0, 1];
        {
            let mut bar: &mut [u32] = &mut foo;
            let mut baz: &mut Example = bar.into();
            baz.y = 2;
            assert_eq!(baz, &Example { x: 0, y: 2 });
        }
        assert_eq!(foo, [0, 2]);
    }
}