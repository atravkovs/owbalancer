use flo_curves::bezier;
use flo_curves::*;

use crate::{AdjustSr, BezierPoint, SpecializationPoints};

const MAX_ADJUSTMENT: f64 = 50.0;
const TOP_RATING: f64 = 2000.0;

pub struct SpecializationScaler {
    any: Vec<(i32, bezier::Curve<Coord2>)>,
    primary: Vec<(i32, bezier::Curve<Coord2>)>,
    secondary: Vec<(i32, bezier::Curve<Coord2>)>,
}

impl SpecializationScaler {
    fn new(data: SpecializationPoints) -> Self {
        Self {
            any: RatingScaler::from_bezier_points(data.any),
            primary: RatingScaler::from_bezier_points(data.primary),
            secondary: RatingScaler::from_bezier_points(data.secondary),
        }
    }

    fn get(&self, specialization: &str) -> Option<&Vec<(i32, bezier::Curve<Coord2>)>> {
        match specialization {
            "any" => Some(&self.any),
            "primary" => Some(&self.primary),
            "secondary" => Some(&self.secondary),
            _ => None
        }
    }
}

pub struct RatingScaler {
    dps: SpecializationScaler,
    tank: SpecializationScaler,
    support: SpecializationScaler,
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

    pub fn scale(&self, role: &str, specialization: &str, rating: i32) -> i32 {
        match role {
            "dps" => self.scale_role(rating, self.dps.get(specialization).unwrap()),
            "tank" => self.scale_role(rating, self.tank.get(specialization).unwrap()),
            "support" => self.scale_role(rating, self.support.get(specialization).unwrap()),
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
            dps: SpecializationScaler::new(adjust.dps),
            tank: SpecializationScaler::new(adjust.tank),
            support: SpecializationScaler::new(adjust.support),
        }
    }
}
