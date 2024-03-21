include!("flipt.evaluation.rs");

#[derive(Clone, Debug, PartialEq)]
pub struct Flag {
    pub key: String,
    pub enabled: bool,
    pub r#type: EvaluationFlagType,
}
