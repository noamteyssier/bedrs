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
    ($chr:expr, $start:expr, $end:expr, $source:expr, $feature:expr, $score:expr, $strand:expr, $frame:expr, $attributes:expr) => {{
        use $crate::types::meta::MetaGtf;
        use $crate::types::record::Features;
        use $crate::types::record::Record;
        Record::new(
            Features::new($chr, $start, $end),
            MetaGtf::new($source, $feature, $score, $strand, $frame, $attributes),
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

#[cfg(test)]
mod testing {
    use crate::prelude::*;

    #[test]
    fn bed3_macro() {
        let record = bed3!("chr1", 1, 10);
        assert_eq!(record.chr(), &"chr1");
        assert_eq!(record.start(), 1);
        assert_eq!(record.end(), 10);
    }

    #[test]
    fn bed4_macro() {
        let record = bed4!("chr1", 1, 10, "name");
        assert_eq!(record.chr(), &"chr1");
        assert_eq!(record.start(), 1);
        assert_eq!(record.end(), 10);
        assert_eq!(record.metadata.name(), &"name");
    }

    #[test]
    fn bed6_macro() {
        let record = bed6!("chr1", 1, 10, "name", 10.into(), Strand::Forward);
        assert_eq!(record.chr(), &"chr1");
        assert_eq!(record.start(), 1);
        assert_eq!(record.end(), 10);
        assert_eq!(record.metadata.name(), &"name");
        assert_eq!(record.metadata.score(), &10.into());
        assert_eq!(record.metadata.strand(), &Strand::Forward);
    }

    #[test]
    fn bed12_macro() {
        let record = bed12!(
            "chr1",
            1,
            10,
            "name",
            10.into(),
            Strand::Forward,
            1,
            10,
            "0,0,0",
            1,
            vec![10],
            vec![1]
        );
        assert_eq!(record.chr(), &"chr1");
        assert_eq!(record.start(), 1);
        assert_eq!(record.end(), 10);
        assert_eq!(record.metadata.name(), &"name");
        assert_eq!(record.metadata.score(), &10.into());
        assert_eq!(record.strand().unwrap(), Strand::Forward);
        assert_eq!(record.metadata.thick_start(), &1);
        assert_eq!(record.metadata.thick_end(), &10);
        assert_eq!(record.metadata.item_rgb(), &"0,0,0");
        assert_eq!(record.metadata.block_count(), &1);
        assert_eq!(record.metadata.block_sizes(), &[10]);
        assert_eq!(record.metadata.block_starts(), &[1]);
    }

    #[test]
    fn bedgraph_macro() {
        let record = bedgraph!("chr1", 1, 10, 10.into());
        assert_eq!(record.chr(), &"chr1");
        assert_eq!(record.start(), 1);
        assert_eq!(record.end(), 10);
        assert_eq!(record.metadata.score(), &10.into());
    }

    #[test]
    fn gtf_macro() {
        let record = gtf!(
            "chr1",
            1,
            10,
            "source",
            "feature",
            10.into(),
            Strand::Forward,
            0.into(),
            "attribute"
        );
        assert_eq!(record.chr(), &"chr1");
        assert_eq!(record.start(), 1);
        assert_eq!(record.end(), 10);
        assert_eq!(record.metadata.source(), &"source");
        assert_eq!(record.metadata.feature(), &"feature");
        assert_eq!(record.metadata.score(), &10.into());
        assert_eq!(record.strand().unwrap(), Strand::Forward);
        assert_eq!(record.metadata.frame(), &0.into());
        assert_eq!(record.metadata.attributes(), &"attribute");
    }

    #[test]
    fn meta_macro() {
        let record = meta_interval!("chr1", 1, 10, "name");
        assert_eq!(record.chr(), &"chr1");
        assert_eq!(record.start(), 1);
        assert_eq!(record.end(), 10);
        assert_eq!(record.metadata.meta(), &"name");
    }

    #[test]
    fn stranded_bed3_macro_a() {
        let record = stranded_bed3!("chr1", 1, 10, Strand::Forward);
        assert_eq!(record.chr(), &"chr1");
        assert_eq!(record.start(), 1);
        assert_eq!(record.end(), 10);
        assert_eq!(record.metadata.strand(), &Strand::Forward);
    }

    #[test]
    fn stranded_bed3_macro_b() {
        let record = bed3!("chr1", 1, 10, Strand::Reverse);
        assert_eq!(record.chr(), &"chr1");
        assert_eq!(record.start(), 1);
        assert_eq!(record.end(), 10);
        assert_eq!(record.metadata.strand(), &Strand::Reverse);
    }
}
