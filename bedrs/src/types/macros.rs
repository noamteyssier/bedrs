#[macro_export]
macro_rules! bed3 {
    ($chr:expr, $start:expr, $end:expr) => {{
        use $crate::types::meta::MetaBed3;
        use $crate::types::record::Features;
        use $crate::types::record::Record;
        Record::new(Features::new($chr, $start, $end), MetaBed3::default())
    }};
    ($chr:expr, $start:expr, $end:expr, $strand:expr) => {{
        use $crate::types::meta::MetaStrandedBed3;
        use $crate::types::record::Features;
        use $crate::types::record::Record;
        Record::new(
            Features::new($chr, $start, $end),
            MetaStrandedBed3::new($strand),
        )
    }};
}

#[macro_export]
macro_rules! bed4 {
    ($chr:expr, $start:expr, $end:expr, $name:expr) => {{
        use $crate::types::meta::MetaBed4;
        use $crate::types::record::Features;
        use $crate::types::record::Record;
        Record::new(Features::new($chr, $start, $end), MetaBed4::new($name))
    }};
}

#[macro_export]
macro_rules! bed6 {
    ($chr:expr, $start:expr, $end:expr, $name:expr, $score:expr, $strand:expr) => {{
        use $crate::types::meta::MetaBed6;
        use $crate::types::record::Features;
        use $crate::types::record::Record;
        Record::new(
            Features::new($chr, $start, $end),
            MetaBed6::new($name, $score, $strand),
        )
    }};
}

#[macro_export]
macro_rules! bed12 {
    ($chr:expr, $start:expr, $end:expr, $name:expr, $score:expr, $strand:expr, $thick_start:expr, $thick_end:expr, $rgb:expr, $block_count:expr, $block_sizes:expr, $block_starts:expr) => {{
        use $crate::types::meta::MetaBed12;
        use $crate::types::record::Features;
        use $crate::types::record::Record;
        Record::new(
            Features::new($chr, $start, $end),
            MetaBed12::new(
                $name,
                $score,
                $strand,
                $thick_start,
                $thick_end,
                $rgb,
                $block_count,
                $block_sizes,
                $block_starts,
            ),
        )
    }};
}

#[macro_export]
macro_rules! bedgraph {
    ($chr:expr, $start:expr, $end:expr, $score:expr) => {{
        use $crate::types::meta::MetaBedGraph;
        use $crate::types::record::Features;
        use $crate::types::record::Record;
        Record::new(Features::new($chr, $start, $end), MetaBedGraph::new($score))
    }};
}

#[macro_export]
macro_rules! gtf {
    ($chr:expr, $start:expr, $end:expr, $name:expr, $score:expr, $strand:expr, $frame:expr, $attributes:expr) => {{
        use $crate::types::meta::MetaGtf;
        use $crate::types::record::Features;
        use $crate::types::record::Record;
        Record::new(
            Features::new($chr, $start, $end),
            MetaGtf::new($name, $score, $strand, $frame, $attributes),
        )
    }};
}

#[macro_export]
macro_rules! meta_interval {
    ($chr:expr, $start:expr, $end:expr, $name:expr) => {{
        use $crate::types::meta::MetaMetaInterval;
        use $crate::types::record::Features;
        use $crate::types::record::Record;
        Record::new(
            Features::new($chr, $start, $end),
            MetaMetaInterval::new($name),
        )
    }};
}

#[macro_export]
macro_rules! stranded_bed3 {
    ($chr:expr, $start:expr, $end:expr, $strand:expr) => {{
        use $crate::types::meta::MetaStrandedBed3;
        use $crate::types::record::Features;
        use $crate::types::record::Record;
        Record::new(
            Features::new($chr, $start, $end),
            MetaStrandedBed3::new($strand),
        )
    }};
}
