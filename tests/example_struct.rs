#[macro_use]
extern crate struct_array;

/// Example struct array.
#[derive(Clone,Debug,PartialEq,StructArray)]
#[repr(C)]
struct Example {
    /// x member
    pub x: u32,
    /// y member
    pub y: u32,
}

#[test]
fn test_deref() {
    let example = Example { x: 0, y: 1 };
    assert_eq!(*example, [0, 1]);
}

#[test]
fn test_deref_mut() {
    use std::ops::DerefMut;

    let mut example = Example { x: 0, y: 1 };
    {
        let array: &mut [u32; 2] = &mut example.deref_mut();
        array[1] = 2;
        assert_eq!(array, &[0, 2]);
    }
    assert_eq!(example, Example { x: 0, y: 2 });
}

#[test]
fn test_into_array() {
    let example = Example { x: 0, y: 1 };
    let array: [u32; 2] = example.into();
    assert_eq!(array, [0, 1]);
}

#[test]
fn test_from_array() {
    let array = [0, 1];
    let mut example: Example = array.into();
    example.y = 2;
    assert_eq!(example, Example { x: 0, y: 2 });
}

#[test]
fn test_into_array_ref() {
    let example = &Example { x: 0, y: 1 };
    let array: &[u32; 2] = example.into();
    assert_eq!(array, &[0, 1]);
}

#[test]
fn test_struct_ref_as_array_ref() {
    let example = &Example { x: 0, y: 1 };
    let array: &[u32; 2] = example.as_ref();
    assert_eq!(array, &[0, 1]);
}

#[test]
fn test_from_array_ref() {
    let array = &[0, 1];
    let example: &Example = array.into();
    assert_eq!(example, &Example { x: 0, y: 1 });
}

#[test]
fn test_array_ref_as_struct_ref() {
    let array = &[0, 1];
    let example: &Example = array.as_ref();
    assert_eq!(example, &Example { x: 0, y: 1 });
}

#[test]
fn test_into_array_ref_mut() {
    let mut example = &mut Example { x: 0, y: 1 };
    {
        let mut array: &mut [u32; 2] = example.into();
        array[1] = 2;
        assert_eq!(array, &mut [0, 2]);
    }
    assert_eq!(example, &mut Example { x: 0, y: 2 });
}

#[test]
fn test_struct_ref_mut_as_array_ref_mut() {
    let mut example = &mut Example { x: 0, y: 1 };
    {
        let mut array: &mut [u32; 2] = example.as_mut();
        array[1] = 2;
        assert_eq!(array, &mut [0, 2]);
    }
    assert_eq!(example, &mut Example { x: 0, y: 2 });
}

#[test]
fn test_from_array_ref_mut() {
    let array = &mut [0, 1];
    {
        let mut example: &mut Example = array.into();
        example.y = 2;
        assert_eq!(example, &mut Example { x: 0, y: 2 });
    }
    assert_eq!(array, &mut [0, 2]);
}

#[test]
fn test_array_ref_mut_as_struct_ref_mut() {
    let array = &mut [0, 1];
    {
        let mut example: &mut Example = array.as_mut();
        example.y = 2;
        assert_eq!(example, &mut Example { x: 0, y: 2 });
    }
    assert_eq!(array, &mut [0, 2]);
}

#[test]
fn test_into_slice_ref() {
    let example = Example { x: 0, y: 1 };
    let slice: &[u32] = (&example).into();
    assert_eq!(slice, [0, 1]);
}

#[test]
fn test_array_ref_as_slice_ref() {
    let example = Example { x: 0, y: 1 };
    let slice: &[u32] = (&example).as_ref();
    assert_eq!(slice, [0, 1]);
}

#[test]
fn test_from_slice_ref() {
    let array = [0, 1];
    let slice: &[u32] = &array;
    let example: &Example = slice.into();
    assert_eq!(example, &Example { x: 0, y: 1 });
}

#[test]
fn test_slice_ref_as_array_ref() {
    let array = [0, 1];
    let slice: &[u32] = &array;
    let example: &Example = slice.as_ref();
    assert_eq!(example, &Example { x: 0, y: 1 });
}

#[test]
fn test_into_slice_ref_mut() {
    let mut example = Example { x: 0, y: 1 };
    {
        let slice: &mut [u32] = (&mut example).into();
        slice[1] = 2;
        assert_eq!(slice, &mut [0, 2]);
    }
    assert_eq!(example, Example { x: 0, y: 2 });
}

#[test]
fn test_struct_ref_mut_as_slice_ref_mut() {
    let mut example = Example { x: 0, y: 1 };
    {
        let slice: &mut [u32] = (&mut example).as_mut();
        slice[1] = 2;
        assert_eq!(slice, &mut [0, 2]);
    }
    assert_eq!(example, Example { x: 0, y: 2 });
}

#[test]
fn test_from_slice_ref_mut() {
    let mut array = [0, 1];
    {
        let mut slice: &mut [u32] = &mut array;
        let mut example: &mut Example = slice.into();
        example.y = 2;
        assert_eq!(example, &Example { x: 0, y: 2 });
    }
    assert_eq!(array, [0, 2]);
}

#[test]
fn test_ref_mut_slice_as_struct_ref_mut() {
    let mut array = [0, 1];
    {
        let mut slice: &mut [u32] = &mut array;
        let mut example: &mut Example = slice.as_mut();
        example.y = 2;
        assert_eq!(example, &Example { x: 0, y: 2 });
    }
    assert_eq!(array, [0, 2]);
}
