use linked_hash_map::LinkedHashMap;
use marked_yaml::Node;
use std::pin::Pin;
use std::rc::Rc;

pub use chrono::NaiveDate as Date;
pub use std::collections::HashMap;

#[doc = ""] // TODO
#[derive(Debug, Clone)]
pub struct Resume {
    pub name: String,
    pub midname: Option<String>,
    pub lastname: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub fingerprint: Option<String>,
    pub website: Option<String>,
    pub github: Option<String>,
    pub reddit: Option<String>,
    pub twitter: Option<String>,
    pub address: Option<String>,
    pub languages: HashMap<String, Language>,
    pub experience: HashMap<String, Position>,
    pub projects: HashMap<String, Project>,
    pub education: HashMap<String, Credential>,
    skills: LinkedHashMap<String, Vec<Pin<Rc<Skill>>>>,
    document: Node,
}

#[doc = ""] // TODO
#[derive(Debug, Clone)]
pub struct Language {
    written: Qualitative,
    spoken: Qualitative,
}

#[doc = ""] // TODO
#[derive(Debug, Clone)]
pub struct Position {
    start: Date,
    end: Option<Date>,
    title: String,
    description: Option<String>,
    skills: Vec<Skill>,
}

#[doc = ""] // TODO
#[derive(Debug, Clone)]
pub struct Project {
    description: String,
    repo: Option<String>,
    skills: Vec<Skill>,
}

#[doc = ""] // TODO
#[derive(Debug, Clone)]
pub struct Credential {
    start: Date,
    end: Option<Date>,
    certification: String,
    grade: Option<String>,
}

#[doc = ""] // TODO
#[derive(Debug, Clone)]
pub struct Skill {
    degree: Qualitative,
    level: Qualitative,
    reason: String,
}

#[doc = ""] // TODO
#[derive(Debug, Copy, Clone)]
pub enum Qualitative {
    VeryLow,
    Low,
    Medium,
    High,
    VeryHigh,
}
