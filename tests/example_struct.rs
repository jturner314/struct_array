#![feature(proc_macro)]

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
    use std::ops::Deref;

    let foo = Example { x: 0, y: 1 };
    let bar: &[u32; 2] = &foo.deref();
    assert_eq!(bar, &[0, 1]);
}

#[test]
fn test_deref_mut() {
    use std::ops::DerefMut;

    let mut foo = Example { x: 0, y: 1 };
    {
        let bar: &mut [u32; 2] = &mut foo.deref_mut();
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
fn test_struct_ref_as_array_ref() {
    let foo = &Example { x: 0, y: 1 };
    let bar: &[u32; 2] = foo.as_ref();
    assert_eq!(bar, &[0, 1]);
}

#[test]
fn test_from_array_ref() {
    let foo = &[0, 1];
    let bar: &Example = foo.into();
    assert_eq!(bar, &Example { x: 0, y: 1 });
}

#[test]
fn test_array_ref_as_struct_ref() {
    let foo = &[0, 1];
    let bar: &Example = foo.as_ref();
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
fn test_struct_ref_mut_as_array_ref_mut() {
    let mut foo = &mut Example { x: 0, y: 1 };
    {
        let mut bar: &mut [u32; 2] = foo.as_mut();
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
fn test_array_ref_mut_as_struct_ref_mut() {
    let foo = &mut [0, 1];
    {
        let mut bar: &mut Example = foo.as_mut();
        bar.y = 2;
        assert_eq!(bar, &mut Example { x: 0, y: 2 });
    }
    assert_eq!(foo, &mut [0, 2]);
}

#[test]
fn test_into_slice_ref() {
    let foo = Example { x: 0, y: 1 };
    let bar: &[u32] = (&foo).into();
    assert_eq!(bar, [0, 1]);
}

#[test]
fn test_array_ref_as_slice_ref() {
    let foo = Example { x: 0, y: 1 };
    let bar: &[u32] = (&foo).as_ref();
    assert_eq!(bar, [0, 1]);
}

#[test]
fn test_from_slice_ref() {
    let foo = [0, 1];
    let bar: &[u32] = &foo;
    let baz: &Example = bar.into();
    assert_eq!(baz, &Example { x: 0, y: 1 });
}

#[test]
fn test_slice_ref_as_array_ref() {
    let foo = [0, 1];
    let bar: &[u32] = &foo;
    let baz: &Example = bar.as_ref();
    assert_eq!(baz, &Example { x: 0, y: 1 });
}

#[test]
fn test_into_slice_ref_mut() {
    let mut foo = Example { x: 0, y: 1 };
    {
        let bar: &mut [u32] = (&mut foo).into();
        bar[1] = 2;
        assert_eq!(bar, &mut [0, 2]);
    }
    assert_eq!(foo, Example { x: 0, y: 2 });
}

#[test]
fn test_struct_ref_mut_as_slice_ref_mut() {
    let mut foo = Example { x: 0, y: 1 };
    {
        let bar: &mut [u32] = (&mut foo).as_mut();
        bar[1] = 2;
        assert_eq!(bar, &mut [0, 2]);
    }
    assert_eq!(foo, Example { x: 0, y: 2 });
}

#[test]
fn test_from_slice_ref_mut() {
    let mut foo = [0, 1];
    {
        let mut bar: &mut [u32] = &mut foo;
        let mut baz: &mut Example = bar.into();
        baz.y = 2;
        assert_eq!(baz, &Example { x: 0, y: 2 });
    }
    assert_eq!(foo, [0, 2]);
}

#[test]
fn test_ref_mut_slice_as_struct_ref_mut() {
    let mut foo = [0, 1];
    {
        let mut bar: &mut [u32] = &mut foo;
        let mut baz: &mut Example = bar.as_mut();
        baz.y = 2;
        assert_eq!(baz, &Example { x: 0, y: 2 });
    }
    assert_eq!(foo, [0, 2]);
}
