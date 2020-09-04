use super::FromStr;
use super::Resume;

#[doc = ""] // TODO Write documentation
pub fn get_sample_resume() -> Resume {
    Resume::from_str(SAMPLE_RESUME).unwrap()
}

const SAMPLE_RESUME: &str = include_str!("sample.yaml");
