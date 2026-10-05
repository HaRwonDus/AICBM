use serde::{Deserialize, Serialize};
use std::collections::HashSet;

/// Half-open XZ rectangle: min inclusive, max exclusive. Heights are separate.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Rect {
    pub min_x: i32,
    pub min_z: i32,
    pub max_x: i32,
    pub max_z: i32,
}
impl Rect {
    pub fn valid(&self) -> bool {
        self.min_x < self.max_x && self.min_z < self.max_z
    }
    pub fn contains(&self, b: &Rect) -> bool {
        self.min_x <= b.min_x
            && self.min_z <= b.min_z
            && self.max_x >= b.max_x
            && self.max_z >= b.max_z
    }
    pub fn overlaps(&self, b: &Rect) -> bool {
        self.min_x < b.max_x && b.min_x < self.max_x && self.min_z < b.max_z && b.min_z < self.max_z
    }
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Building {
    pub id: String,
    pub footprint: Rect,
    pub base_y: i32,
    pub height: u32,
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Plan {
    pub schema_version: u32,
    pub region: Rect,
    pub min_y: i32,
    pub max_y_exclusive: i32,
    pub protected: Vec<Rect>,
    pub buildings: Vec<Building>,
}
/// Structural validation only. Does NOT certify roads, terrain, or buildability.
pub fn validate(plan: &Plan) -> Vec<String> {
    let mut errors = Vec::new();
    if plan.schema_version != 1 {
        errors.push("unsupported schema_version".into());
    }
    if !plan.region.valid() {
        errors.push("invalid region".into());
    }
    if plan.min_y >= plan.max_y_exclusive {
        errors.push("invalid height bounds".into());
    }
    for (i, r) in plan.protected.iter().enumerate() {
        if !r.valid() || !plan.region.contains(r) {
            errors.push(format!("protected[{i}]: invalid bounds"));
        }
    }
    let mut ids = HashSet::new();
    for (i, b) in plan.buildings.iter().enumerate() {
        if b.id.trim().is_empty() || !ids.insert(&b.id) {
            errors.push(format!("building[{i}]: empty or duplicate id"));
        }
        if !b.footprint.valid() || !plan.region.contains(&b.footprint) {
            errors.push(format!("{}: invalid footprint", b.id));
        }
        if b.height == 0
            || b.base_y < plan.min_y
            || i64::from(b.base_y) + i64::from(b.height) > i64::from(plan.max_y_exclusive)
        {
            errors.push(format!("{}: invalid height", b.id));
        }
        if plan.protected.iter().any(|r| r.overlaps(&b.footprint)) {
            errors.push(format!("{}: protected area collision", b.id));
        }
        for other in &plan.buildings[..i] {
            if b.footprint.overlaps(&other.footprint) {
                errors.push(format!("{}: overlaps {}", b.id, other.id));
            }
        }
    }
    errors
}
#[cfg(test)]
mod tests {
    use super::*;
    fn sample() -> Plan {
        serde_json::from_str(include_str!("../examples/plan.json")).unwrap()
    }
    #[test]
    fn accepts_valid_plan() {
        assert!(validate(&sample()).is_empty());
    }
    #[test]
    fn rejects_outside_region() {
        let mut p = sample();
        p.buildings[0].footprint.max_x = 200;
        assert!(!validate(&p).is_empty());
    }
    #[test]
    fn rejects_protected_collision() {
        let mut p = sample();
        p.protected.push(p.buildings[0].footprint.clone());
        assert!(validate(&p).iter().any(|e| e.contains("protected")));
    }
    #[test]
    fn rejects_overlap() {
        let mut p = sample();
        p.buildings[1].footprint = p.buildings[0].footprint.clone();
        assert!(validate(&p).iter().any(|e| e.contains("overlaps")));
    }
    #[test]
    fn allows_adjacent_footprints() {
        let a = Rect {
            min_x: 0,
            min_z: 0,
            max_x: 5,
            max_z: 5,
        };
        let b = Rect {
            min_x: 5,
            min_z: 0,
            max_x: 10,
            max_z: 5,
        };
        assert!(!a.overlaps(&b));
    }
    #[test]
    fn rejects_duplicate_ids() {
        let mut p = sample();
        p.buildings[1].id = p.buildings[0].id.clone();
        assert!(validate(&p).iter().any(|e| e.contains("duplicate")));
    }
    #[test]
    fn handles_large_height_without_overflow() {
        let mut p = sample();
        p.buildings[0].height = u32::MAX;
        assert!(!validate(&p).is_empty());
    }
    #[test]
    fn rejects_unknown_fields() {
        assert!(serde_json::from_str::<Rect>(
            r#"{"min_x":0,"min_z":0,"max_x":1,"max_z":1,"extra":0}"#
        )
        .is_err());
    }
    #[test]
    fn rejects_invalid_contract_bounds() {
        let mut p = sample();
        p.schema_version = 2;
        p.region.max_x = p.region.min_x;
        p.max_y_exclusive = p.min_y;
        let errors = validate(&p);
        assert!(errors.iter().any(|e| e == "unsupported schema_version"));
        assert!(errors.iter().any(|e| e == "invalid region"));
        assert!(errors.iter().any(|e| e == "invalid height bounds"));
    }
    #[test]
    fn accepts_exact_height_boundary() {
        let mut p = sample();
        p.buildings[0].base_y = p.max_y_exclusive - 12;
        assert!(validate(&p).is_empty());
        p.buildings[0].height = 13;
        assert!(validate(&p).iter().any(|e| e.contains("invalid height")));
    }
    #[test]
    fn rejects_empty_id_zero_height_and_invalid_protection() {
        let mut p = sample();
        p.buildings[0].id = "  ".into();
        p.buildings[0].height = 0;
        p.protected[0].max_x = 200;
        let errors = validate(&p);
        assert!(errors.iter().any(|e| e.contains("empty or duplicate")));
        assert!(errors.iter().any(|e| e.contains("invalid height")));
        assert!(errors.iter().any(|e| e.contains("protected[0]")));
    }
}
