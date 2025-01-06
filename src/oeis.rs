use serde_json::Value;

use crate::sequence::Sequence;

/// An OEIS sequence is a sequence that is in the OEIS database.
/// We query the database by getting a json from a url like this:
///    `https://oeis.org/search?q=1,2,4,8&fmt=json`
pub(crate) struct OeisSeq {
    data: Vec<i128>,
}

impl Sequence for OeisSeq {
    fn infer(seq: &[i128]) -> Option<Self> {
        // construct url
        let url = format!(
            "https://oeis.org/search?q={:?}&fmt=json",
            seq.iter()
                .map(|x| x.to_string())
                .collect::<Vec<_>>()
                .join(",")
        );

        // get the json
        let response = reqwest::blocking::get(url).unwrap();
        let body = response.text().unwrap();

        // parse json
        let json: Value = serde_json::from_str(&body).unwrap();

        // we expect an array of sequences
        let Value::Array(arr) = json else {
            return None;
        };

        // we want the first sequence
        let first = arr.first().unwrap();
        let Value::Object(obj) = first else {
            return None;
        };

        // we want the data field of that sequence
        let Value::String(name) = obj.get("data")? else {
            return None;
        };

        // this is a string that contains a comma separated list of numbers
        let data = name
            .split(',')
            .map(|x| {
                x.parse()
                    .expect(&format!("unable to parse {:?} as integer", x))
            })
            .collect();

        Some(OeisSeq { data })
    }

    fn generate(&self, seq: &[i128], end: i128) -> Vec<i128> {
        let start_idx = (0..self.data.len())
            .into_iter()
            .find(|idx| self.data[*idx..*idx + seq.len()] == *seq)
            .expect(&format!(
                "Sequence not found in data. Sequence: {:?}. Data: {:?}",
                seq, self.data
            ));

        let mut seq = Vec::new();
        let mut terminated = false;
        for x in &self.data[start_idx..] {
            if *x >= end {
                terminated = true;
                break;
            }
            seq.push(*x);
        }
        if !terminated {
            panic!(
                "OEIS sequence is incomplete and terminates with an value less than the upper bound of {:?}. OEIS sequence: {:?}",
                end, seq
            );
        }
        seq
    }
}
