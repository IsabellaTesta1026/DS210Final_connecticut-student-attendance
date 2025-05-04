use std::collections::{HashMap, BTreeSet};
use crate::data::DistrictRecord;

/// Build grouped feature vectors:
/// Output: HashMap<student_group, HashMap<district, Vec<f64>>>
pub fn build_grouped_feature_vectors(records: &[DistrictRecord]) -> HashMap<String, HashMap<String, Vec<f64>>> {
    let mut group_to_vectors: HashMap<String, HashMap<String, Vec<f64>>> = HashMap::new();

    // Collect all year keys across all records
    let mut all_years = BTreeSet::new();
    for rec in records {
        all_years.extend(rec.rates_by_year.keys().cloned());
    }
    let year_order: Vec<String> = all_years.into_iter().collect();

    for rec in records {
        let group = &rec.student_group;
        let district = &rec.district_name;

        let group_map = group_to_vectors.entry(group.clone()).or_default();
        let district_map = group_map.entry(district.clone()).or_insert_with(|| vec![0.0; year_order.len()]);

        for (i, year) in year_order.iter().enumerate() {
            if let Some(rate) = rec.rates_by_year.get(year) {
                district_map[i] = *rate;
            }
        }
    }

    group_to_vectors
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use crate::data::DistrictRecord;

    #[test]
    fn test_build_grouped_feature_vectors() {
        let mut rates = HashMap::new();
        rates.insert("2020".to_string(), 95.0);
        rates.insert("2021".to_string(), 96.0);
        let record = DistrictRecord {
            district_name: "Test District".to_string(),
            student_group: "All Students".to_string(),
            rates_by_year: rates,
        };
        let vectors = build_grouped_feature_vectors(&[record]);
        assert!(vectors.contains_key("All Students"));
        assert!(vectors["All Students"].contains_key("Test District"));
    }
}
