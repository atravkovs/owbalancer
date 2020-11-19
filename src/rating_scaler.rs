use flo_curves::bezier;
use flo_curves::*;

use crate::{AdjustSr, BezierPoint};

const MAX_ADJUSTMENT: f64 = 50.0;
const TOP_RATING: f64 = 5000.0;

pub struct RatingScaler {
    dps: Vec<(i32, bezier::Curve<Coord2>)>,
    tank: Vec<(i32, bezier::Curve<Coord2>)>,
    support: Vec<(i32, bezier::Curve<Coord2>)>,
}

impl RatingScaler {
    fn scale_role(&self, rating: i32, curves: &Vec<(i32, bezier::Curve<Coord2>)>) -> i32 {
        let mut curve_index = 0;

        for i in 0..curves.len() {
            if curves[i].0 < rating {
                curve_index = i + 1
            }
        }

        let curve = &curves[curve_index].1;
        let mut min_rating = 0;
        let max_rating = curves[curve_index].0;

        if curve_index > 0 {
            min_rating = curves[curve_index - 1].0;
        }

        let t = (rating as f64 - min_rating as f64) / (max_rating as f64 - min_rating as f64);
        let pos = curve.point_at_pos(t);
        let scale = 1.0 + pos.y() / 100.0;

        (rating as f64 * scale).floor() as i32
    }

    pub fn scale(&self, role: &str, rating: i32) -> i32 {
        match role {
            "dps" => self.scale_role(rating, &self.dps),
            "tank" => self.scale_role(rating, &self.tank),
            "support" => self.scale_role(rating, &self.support),
            _ => 0,
        }
    }

    fn from_bezier_points(points: Vec<BezierPoint>) -> Vec<(i32, bezier::Curve<Coord2>)> {
        let mut curves = Vec::default();

        for i in 1..points.len() {
            let previous = &points[i - 1];
            let current = &points[i];

            let curve = bezier::Curve::from_points(
                Coord2(
                    previous.position.x * TOP_RATING,
                    previous.position.y * MAX_ADJUSTMENT,
                ),
                (
                    Coord2(
                        previous.position.x * TOP_RATING + previous.control.x * TOP_RATING,
                        previous.position.y * MAX_ADJUSTMENT
                            + (previous.control.y - 1.0) * MAX_ADJUSTMENT,
                    ),
                    Coord2(
                        current.position.x * TOP_RATING - current.control.x * TOP_RATING,
                        current.position.y * MAX_ADJUSTMENT
                            - (current.control.y - 1.0) * MAX_ADJUSTMENT,
                    ),
                ),
                Coord2(
                    current.position.x * TOP_RATING,
                    current.position.y * MAX_ADJUSTMENT,
                ),
            );

            curves.push(((current.position.x * TOP_RATING).floor() as i32, curve))
        }

        curves
    }
}

impl From<AdjustSr> for RatingScaler {
    fn from(adjust: AdjustSr) -> Self {
        Self {
            dps: Self::from_bezier_points(adjust.dps),
            tank: Self::from_bezier_points(adjust.tank),
            support: Self::from_bezier_points(adjust.support),
        }
    }
}
