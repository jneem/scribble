use druid::im::OrdMap;
use druid::kurbo::BezPath;
use druid::Point;
use serde::Deserialize;
use std::collections::BTreeMap;

use crate::{DrawSnippetId, StrokeStyle, Time};

/// This module implements serialization and deserialization for version 0 of our save file format.
/// We probably don't actually need to support reading v0 files (it was around the 0.1.0 release,
/// so probably no one actually created these files), but having this here just helps us be sure
/// that we have a framework to support version bumps in the future.
pub mod v0 {
    use super::*;

    #[derive(Deserialize)]
    pub struct Lerp {
        original_values: Vec<Time>,
        lerped_values: Vec<Time>,
    }

    impl From<Lerp> for crate::Lerp {
        fn from(lerp: Lerp) -> crate::Lerp {
            crate::Lerp {
                original_values: lerp.original_values,
                lerped_values: lerp.lerped_values,
            }
        }
    }

    #[derive(Deserialize)]
    pub struct DrawSnippet {
        curve: StrokeSeq,
        lerp: Lerp,
        end: Option<Time>,
    }

    impl From<DrawSnippet> for crate::DrawSnippet {
        fn from(data: DrawSnippet) -> crate::DrawSnippet {
            crate::DrawSnippet::new_complete(data.curve.into(), data.lerp.into(), data.end)
        }
    }

    #[derive(Deserialize)]
    #[serde(transparent)]
    pub struct DrawSnippets {
        snippets: BTreeMap<DrawSnippetId, DrawSnippet>,
    }

    impl From<DrawSnippets> for crate::DrawSnippets {
        fn from(data: DrawSnippets) -> crate::DrawSnippets {
            let max_id = data.snippets.keys().max().unwrap_or(&DrawSnippetId(0)).0;
            let snippets: OrdMap<DrawSnippetId, crate::DrawSnippet> = data
                .snippets
                .into_iter()
                .map(|(id, snip)| (id, Into::<crate::DrawSnippet>::into(snip)))
                .collect();
            crate::DrawSnippets {
                last_id: max_id,
                snippets,
            }
        }
    }

    #[derive(Deserialize)]
    #[serde(transparent)]
    pub struct StrokeSeq(Vec<SavedSegment>);

    impl From<StrokeSeq> for crate::StrokeSeq {
        fn from(s: StrokeSeq) -> crate::StrokeSeq {
            let mut curve = crate::StrokeSeq::default();

            for stroke in s.0 {
                let p = |(x, y)| Point::new(x as f64 / 10_000.0, y as f64 / 10_000.0);

                let mut path = BezPath::new();
                if stroke.elements.is_empty() {
                    continue;
                }
                path.move_to(p(stroke.elements[0]));
                for points in stroke.elements[1..].chunks(3) {
                    path.curve_to(p(points[0]), p(points[1]), p(points[2]));
                }

                let times: Vec<Time> = stroke
                    .times
                    .into_iter()
                    .map(|x| Time::from_micros(x as i64))
                    .collect();

                curve.append_path(path, times, stroke.style);
            }
            curve
        }
    }

    #[derive(Deserialize)]
    struct SavedSegment {
        elements: Vec<(i32, i32)>,
        times: Vec<u64>,
        style: StrokeStyle,
    }
}
