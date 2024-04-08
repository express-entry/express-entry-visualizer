pub mod category;
pub mod invite;
pub mod plan;
pub mod pool;

mod utils {
    use std::{fmt::Debug, ops::Index};

    use chrono::NaiveDate;
    use serde_wasm_bindgen::Serializer;

    pub trait ToTimestamp {
        fn to_timestamp(&self) -> i64;
    }

    impl ToTimestamp for NaiveDate {
        fn to_timestamp(&self) -> i64 {
            self.and_hms_opt(0, 0, 0).unwrap().timestamp_millis()
        }
    }

    pub static SERIALIZER: Serializer = Serializer::new().serialize_missing_as_null(true);

    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct Stacker<const N: usize, T>
    where
        T: Index<usize, Output = f64> + Debug + Clone + Copy + PartialEq,
    {
        data: T,
    }

    impl<const N: usize, T> Stacker<N, T>
    where
        T: Index<usize, Output = f64> + Debug + Clone + Copy + PartialEq,
    {
        pub fn new(data: T) -> Self {
            Self { data }
        }

        pub fn rev(&self, i: usize) -> f64 {
            (i..N).map(|k| self.data[k]).sum()
        }

        pub fn val(&self, i: usize) -> f64 {
            (0..=i).map(|k| self.data[k]).sum()
        }
    }
}

mod dataset {
    use serde::Serialize;

    #[derive(Serialize, Clone, Debug)]
    pub struct ChartData<T: Serialize> {
        pub labels: Vec<Label>,
        pub datasets: Vec<T>,
        pub tooltip: Tooltip,
    }

    #[derive(Serialize, Clone, Debug)]
    pub struct Tooltip {
        pub title: Vec<Vec<String>>,
        pub label: Vec<Vec<String>>,
    }

    impl Default for Tooltip {
        fn default() -> Self {
            Self {
                title: Vec::new(),
                label: Vec::new(),
            }
        }
    }

    #[derive(Serialize, Clone, Debug)]
    pub struct LineDataset {
        pub label: String,
        pub data: Vec<Option<f64>>,
        #[serde(rename = "backgroundColor")]
        pub background_color: String,
        #[serde(rename = "borderColor")]
        pub border_color: String,
        #[serde(rename = "borderDash")]
        pub border_dash: [f64; 2],
        #[serde(rename = "spanGaps")]
        pub span_gaps: bool,
        #[serde(rename = "cubicInterpolationMode")]
        pub cubic_interpolation_mode: String,
        pub tension: f64,
        pub fill: bool,
        #[serde(rename = "pointStyle")]
        pub point_style: PointStyle,
    }

    impl Default for LineDataset {
        fn default() -> Self {
            Self {
                label: "none".into(),
                data: Default::default(),
                background_color: "#ffffff".into(),
                border_color: "#ffffff".into(),
                border_dash: [0.0, 0.0],
                span_gaps: true,
                cubic_interpolation_mode: "monotone".into(),
                tension: 0.0,
                fill: false,
                point_style: PointStyle(Some("circle".into())),
            }
        }
    }

    #[derive(Clone, Debug)]
    pub struct PointStyle(pub Option<String>);

    impl Serialize for PointStyle {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            match &self.0 {
                None => serializer.serialize_bool(false),
                Some(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }

    #[derive(Clone, Debug)]
    pub enum Label {
        String(String),
        Number(f64),
    }

    impl Serialize for Label {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            match self {
                Self::String(value) => serializer.serialize_str(value),
                Self::Number(value) => serializer.serialize_f64(*value),
            }
        }
    }

    impl From<String> for Label {
        fn from(value: String) -> Self {
            Self::String(value)
        }
    }

    impl From<&str> for Label {
        fn from(value: &str) -> Self {
            Self::String(value.into())
        }
    }

    impl From<f64> for Label {
        fn from(value: f64) -> Self {
            Self::Number(value)
        }
    }

    #[derive(Serialize, Clone, Debug)]
    pub struct BarDataset {
        pub label: String,
        pub data: Vec<Option<f64>>,
        #[serde(rename = "backgroundColor")]
        pub background_color: String,
        #[serde(rename = "borderColor")]
        pub border_color: String,
        pub stack: String,
    }

    impl Default for BarDataset {
        fn default() -> Self {
            Self {
                label: "none".into(),
                data: Default::default(),
                background_color: "#ffffff".into(),
                border_color: "#ffffff".into(),
                stack: "0".into(),
            }
        }
    }

    #[derive(Serialize, Clone, Debug)]
    pub struct PieDataset {
        pub label: String,
        pub data: Vec<f64>,
        #[serde(rename = "backgroundColor")]
        pub background_color: Vec<String>,
        #[serde(rename = "borderColor")]
        pub border_color: Vec<String>,
        #[serde(rename = "hoverOffset")]
        pub hover_offset: f64,
    }

    impl Default for PieDataset {
        fn default() -> Self {
            Self {
                label: "none".into(),
                data: Default::default(),
                background_color: Default::default(),
                border_color: Default::default(),
                hover_offset: 32.0,
            }
        }
    }

    #[derive(Serialize, Clone, Debug)]
    pub struct Dropdown {
        pub label: String,
        pub key: f64,
    }
}
