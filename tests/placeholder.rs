mod partial {
    use partialdebug::placeholder::PartialDebug;

    #[derive(PartialDebug)]
    pub struct UnitStruct;

    #[derive(PartialDebug)]
    pub struct TupleStruct(pub &'static str);

    #[derive(PartialDebug)]
    pub struct NormalStruct {
        pub field: &'static str,
    }

    #[derive(PartialDebug)]
    pub enum Mixed {
        Unit,
        Tuple(Box<u8>),
        Struct { a: u8 },
    }
}

mod normal {
    #[derive(Debug)]
    pub struct UnitStruct;

    #[derive(Debug)]
    pub struct TupleStruct(pub &'static str);

    #[derive(Debug)]
    pub struct NormalStruct {
        pub field: &'static str,
    }

    #[derive(Debug)]
    pub enum Mixed {
        Unit,
        Tuple(Box<u8>),
        Struct { a: u8 },
    }
}

#[test]
fn test_unit() {
    assert_eq!(
        format!("{:?}", normal::UnitStruct),
        format!("{:?}", partial::UnitStruct)
    );
}
#[test]
fn test_tuple() {
    assert_eq!(
        format!("{:?}", normal::TupleStruct("asdf")),
        format!("{:?}", partial::TupleStruct("asdf"))
    );
}
#[test]
fn test_normal() {
    assert_eq!(
        format!("{:?}", normal::NormalStruct { field: "asdf" }),
        format!("{:?}", partial::NormalStruct { field: "asdf" })
    );
}
#[test]
fn test_enum() {
    assert_eq!(
        format!("{:?}", normal::Mixed::Unit),
        format!("{:?}", partial::Mixed::Unit)
    );
    assert_eq!(
        format!("{:?}", normal::Mixed::Tuple(Box::new(42))),
        format!("{:?}", partial::Mixed::Tuple(Box::new(42)))
    );
    assert_eq!(
        format!("{:?}", normal::Mixed::Struct { a: 42 }),
        format!("{:?}", partial::Mixed::Struct { a: 42 })
    );
}
