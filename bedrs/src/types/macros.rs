#[macro_export]
macro_rules! bed3 {
    ($chr:expr, $start:expr, $end:expr) => {{
        use $crate::types::meta::MetaBed3;
        use $crate::types::record::Coordinate;
        use $crate::types::record::Record;
        Record::new(Coordinate::new($chr, $start, $end), MetaBed3::default())
    }};
}

#[macro_export]
macro_rules! bed4 {
    ($chr:expr, $start:expr, $end:expr, $name:expr) => {{
        use $crate::types::meta::MetaBed4;
        use $crate::types::record::Coordinate;
        use $crate::types::record::Record;
        Record::new(Coordinate::new($chr, $start, $end), MetaBed4::new($name))
    }};
}
