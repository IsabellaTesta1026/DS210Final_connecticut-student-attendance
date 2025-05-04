//Purpose: Responsible for transforming raw attendance records into grouped numerical feature vectors
//for each district, sorted by year. These feature vectors are used for similarity comparisons
//and graph construction will be discussed later in the analysis.

//Hash to store, BT to order the year as keys
use std::collections::{HashMap, BTreeSet};
use crate::data::DistrictRecord; //loads data

//Function 1: build_grouped_feature_vectors
//Constructs a nested mapping structure that groups attendance rate vectors by student group
//and district name. Each vector contains attendance rates ordered chronologically by year.
//Inputs:
    //`records`: A slice of `DistrictRecord` values
//Output:
    //A `HashMap<String, HashMap<String, Vec<f64>>>`
        //Outer key: student group name (e.g., "All Students")
        //Inner key: district name
        //Value: vector of attendance rates ordered by year
//Logic:
    //Extracts and sorts all years from the dataset
    //Initializes feature vectors of appropriate length per group and district
    //Populates vectors with attendance rates in correct year order
pub fn build_grouped_feature_vectors(records: &[DistrictRecord]) -> HashMap<String, HashMap<String, Vec<f64>>> {
    let mut group_to_vectors: HashMap<String, HashMap<String, Vec<f64>>> = HashMap::new();

    //step 1:
    // Collect all year keys across all records
    //ensuring all years are sorted through BT
    let mut all_years = BTreeSet::new();
    for rec in records {
        all_years.extend(rec.rates_by_year.keys().cloned());
    }
    //step 2:convert years into ordered pairs for indexing 
    let year_order: Vec<String> = all_years.into_iter().collect();

    //step 3: interate through each record
    for rec in records {
        let group = &rec.student_group;
        let district = &rec.district_name;
        
        //create group entries
        let group_map = group_to_vectors.entry(group.clone()).or_default();
        
        //Initialize the district vector with 0.0
        let district_map = group_map.entry(district.clone()).or_insert_with(|| vec![0.0; year_order.len()]);

        //fill the rates by matching the correct year index
        for (i, year) in year_order.iter().enumerate() {
            if let Some(rate) = rec.rates_by_year.get(year) {
                //set values when applicable
                district_map[i] = *rate;
            }
        }
    }

    group_to_vectors
}

//testsss
#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use crate::data::DistrictRecord;

    #[test]
    //does a single record generates the correct nested vector structure
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
        //does outer key exist
        assert!(vectors.contains_key("All Students"));
        //does inner key exist
        assert!(vectors["All Students"].contains_key("Test District"));
    }
}
