# struct_array

Provides procedural macros that allow a struct to be easily converted to/from
arrays and slices. (See version 0.1.0 for a less-featureful alternative using
`macro_rules!` instead of procedural macros.)

The `StructArrayDeref` and `StructArrayConvert` procedural macros implement
the necessary traits such that the struct can be easily converted to/from
arrays and slices. The `StructArray` procedural macro applies both
`StructArrayDeref` and `StructArrayConvert`. The macros work for normal
structs and tuple structs. The macros check that the struct has the
following properties:

  * all the fields must be public (because they are exposed in
    arrays/slices created by the conversion functions)
  * all the fields must have the same type
  * the struct must have at least one field
  * the struct must have the `#[repr(C)]` attribute

## Example

```rust
#[macro_use]
extern crate struct_array;

/// Example struct array.
#[derive(Clone,Debug,PartialEq,StructArray)]
#[repr(C)]
struct Example {
    pub x: u32,
    pub y: u32,
}

fn main() {
    // Deref as an array.
    {
        let example = Example { x: 42, y: 56 };
        let array: [u32; 2] = *example;
        assert_eq!(array, [42, 56]);
    }

    // Index (via derefing as an array).
    {
        let mut example = Example { x: 42, y: 56 };
        example[1] = 23;
        assert_eq!(example, Example { x: 42, y: 23 });
    }

    // Convert into an array.
    {
        let example = Example { x: 42, y: 56 };
        let array: [u32; 2] = example.into();
        assert_eq!(array, [42, 56]);
    }

    // Convert from an array.
    {
        let array = [42, 56];
        let example: Example = array.into();
        assert_eq!(example, Example { x: 42, y: 56 });
    }

    // Convert a ref into a slice.
    {
        let example = &Example { x: 42, y: 56 };
        let slice: &[u32] = example.into();
        assert_eq!(slice, &[42, 56]);
    }

    // Convert a slice into a ref.
    {
        let slice = &[42, 56][..];
        let example: &Example = slice.into();
        assert_eq!(example, &Example { x: 42, y: 56 });
    }
}

```

## Trait implementations

Deriving `StructArray` for a struct causes it to implement all the methods
provided by `StructArrayDeref` and `StructArrayConvert`.

Deriving `StructArrayDeref` for a struct `Foo` causes it to implement:

* `Deref<Target=[T; len]> for Foo`
* `DerefMut<Target=[T; len]> for Foo`

Deriving `StructArrayConvert` for a struct `Foo` creates implementations
for the following:

* `From<Foo> for [T; len]`
* `From<[T; len]> for Foo`
* `From<&Foo> for &[T; len]`
* `AsRef<[T; len] for Foo`
* `From<&[T; len] for &Foo`
* `AsRef<Foo> for [T; len]`
* `From<&mut Foo> for &mut [T; len]`
* `AsMut<[T; len] for Foo`
* `From<&mut [T; len] for &mut Foo`
* `AsMut<Foo> for [T; len]`
* `From<&Foo> for &[T]`
* `AsRef<[T]> for Foo`
* `From<&[T]> for &Foo`
* `AsRef<Foo> for [T]`
* `From<&mut Foo> for &mut [T]`
* `AsMut<[T]> for Foo`
* `From<&mut [T]> for &mut Foo`
* `AsMut<Foo> for [T]`

Note that converting from a slice will panic if the `len()` of the slice
does not must match the number of fields in the struct.

## License

`struct_array` is copyright 2016, Jim Turner.

Licensed under the MIT license. See [LICENSE](LICENSE) for details.
