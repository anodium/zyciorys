#[path = "types.rs"]
mod types;
pub use types::*;

#[path = "traits.rs"]
mod traits;
pub use traits::*;

mod sample {
    use super::traits::FromStr;
    use super::types::Resume;

    #[doc = ""] // TODO
    pub fn get_sample_resume() -> Resume {
        Resume::from_str(SAMPLE_RESUME).unwrap()
    }

    const SAMPLE_RESUME: &'static str = include_str!("sample.yaml");
}
pub use sample::get_sample_resume;
