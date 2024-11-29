use std::fmt;

#[allow(unused)]
#[derive(Debug, Clone, Copy)]
pub enum ScaleDegree {
    D1,
    Db2,
    D2,
    Db3,
    D3,
    D4,
    Db5,
    D5,
    Db6,
    D6,
    Db7,
    D7,
}

impl fmt::Display for ScaleDegree {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let str = format!("{:?}", self);

        let str = str
            .strip_prefix("D")
            .expect("ScaleDegree enum variant does not have 'D' prefix");

        write!(f, "{}", str)
    }
}

impl ScaleDegree {
    pub fn major_scale_degrees() -> impl Iterator<Item = ScaleDegree> {
        use ScaleDegree::*;
        static DEGREES: [ScaleDegree; 7] = [D1, D2, D3, D4, D5, D6, D7];

        DEGREES.into_iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scale_degree_display() {
        let test_cases = [
            (ScaleDegree::D1, "1"),
            (ScaleDegree::Db2, "b2"),
            (ScaleDegree::D2, "2"),
            (ScaleDegree::Db3, "b3"),
            (ScaleDegree::D3, "3"),
            (ScaleDegree::D4, "4"),
            (ScaleDegree::Db5, "b5"),
            (ScaleDegree::D5, "5"),
            (ScaleDegree::Db6, "b6"),
            (ScaleDegree::D6, "6"),
            (ScaleDegree::Db7, "b7"),
            (ScaleDegree::D7, "7"),
        ];

        for (degree, expected_display) in test_cases.iter() {
            let result = format!("{}", degree);
            assert_eq!(result, *expected_display, "Failed for {:?}", degree);
        }
    }
}
