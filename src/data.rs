//import libraries
//Catch errors and use hash
use std::collections::HashMap;
use std::error::Error;
//External imports
//Transmits to storage
use serde::Deserialize;
//Reads external file
use csv::ReaderBuilder;
//
//Debug
#[derive(Debug, Deserialize)]
//Struct meant to represent the raw data from the file
//so each column of data is grouped and matched with its specific field
//The values are saved as a string to ensure exact values
struct RawDistrictRecord {
    #[serde(rename = "District name")]
    district_name: String,

    #[serde(rename = "Student group")]
    student_group: String,

    #[serde(rename = "2022-2023 attendance rate - year to date")]
    rate_2023: String,

    #[serde(rename = "2021-2022 attendance rate")]
    rate_2022: String,

    #[serde(rename = "2020-2021 attendance rate")]
    rate_2021: String,

    #[serde(rename = "2019-2020 attendance rate")]
    rate_2020: String,
}

//debug
#[derive(Debug)]
//Struct in order to parse data in hopes of cleaning as needed
//end values should be as follows:
pub struct DistrictRecord {
    pub district_name: String,
    pub student_group: String,
    //Attendance is saved as a map to cover each year and f64 to allow for more exact percentages
    pub rates_by_year: HashMap<String, f64>,
}
//loads the CSV file and adds value to each District Record
pub fn load_data(path: &str) -> Result<Vec<DistrictRecord>, Box<dyn Error>> {
    let mut rdr = ReaderBuilder::new().has_headers(true).from_path(path)?;
    let mut records = Vec::new();

    //loops each row in order to format the data into a rust structure
    for result in rdr.deserialize() {
        let raw: RawDistrictRecord = result?;
        
        let mut rates_by_year = HashMap::new();
        //formats the rate of each year from string to f64
        for (year, value) in [
            ("2023", &raw.rate_2023),
            ("2022", &raw.rate_2022),
            ("2021", &raw.rate_2021),
            ("2020", &raw.rate_2020),
        ] {
            //removes % and converts to float
            let clean = value.trim_end_matches('%').trim().parse::<f64>().unwrap_or(0.0);
            rates_by_year.insert(year.to_string(), clean);
        }
        //store the cleaned information
        records.push(DistrictRecord {
            district_name: raw.district_name,
            student_group: raw.student_group,
            rates_by_year,
        });
    }
//return the cleaned data
    Ok(records)
}

//Test for mod
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    //manually construct a test record
    fn test_district_record_construction() {
        let mut rates = HashMap::new();
        rates.insert("2020".to_string(), 90.0);
        rates.insert("2021".to_string(), 91.5);

        let record = DistrictRecord {
            district_name: "Sample District".to_string(),
            student_group: "Students with Disabilities".to_string(),
            rates_by_year: rates,
        };
        //confirms if data is stored proper
        assert_eq!(record.district_name, "Sample District");
        assert!(record.rates_by_year.contains_key("2021"));
    }
}
