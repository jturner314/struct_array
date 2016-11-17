# struct_array

The `struct_array!` macro generates a `struct` that can be easily converted
to/from arrays and slices. The members of the struct must all be the same type.

## Example

```rust
#[macro_use]
extern crate struct_array;

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

fn main() {
    // Deref as an array.
    {
        let example = Example { x: 42, y: 56 };
        let array: &[u32; 2] = &example;
        assert_eq!(array, &[42, 56]);
    }

    // Deref as a slice (via derefing as an array).
    {
        let example = Example { x: 42, y: 56 };
        let slice: &[u32] = &*example;
        assert_eq!(slice, &[42, 56]);
    }

    // Convert into an array.
    {
        let example = Example { x: 42, y: 56 };
        let array: [u32; 2] = example.into();
        assert_eq!(array, [42, 56]);
    }

    // Convert from an array to a struct.
    {
        let array = [42, 56];
        let example: Example = array.into();
        assert_eq!(example, Example { x: 42, y: 56 });
    }

    // Convert from a slice to a struct reference.
    {
        let slice = &[42, 56][..];
        let example: &Example = slice.into();
        assert_eq!(example, &Example { x: 42, y: 56 });
    }
}
```

## License

`struct_array` is copyright 2016, Jim Turner.

Licensed under the MIT license. See [LICENSE](LICENSE) for details.
