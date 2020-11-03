use partialdebug::placeholder::PartialDebug;

#[derive(PartialDebug)]
struct UnitStruct;

#[derive(PartialDebug)]
struct TupleStruct(&'static str);

#[derive(PartialDebug)]
struct NormalStruct {
    field: &'static str,
}
