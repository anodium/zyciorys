use yaml_rust::{Yaml, YamlLoader, YamlEmitter};

fn _get_sample_resume() -> &'static str {
r#"%YAML 1.2
---
name: Saniyya
surname: Dane

phone: +11 1111 1111 11
email: saniyya@dane.example.com
fingerprint: AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA
website:
    - https://saniyya.example.net/
    - https://dane.example.com/
address: |
    1100 TEST DRIVE
    SUITE #4
    MINNEAPOLIS, MN 50000
    UNITED STATES
languages:
    Rust:
        written: excellent
        spoken: none
    Python:
        written: none
        spoken: excellent


experience:
    Test Project:
        start: 2010 January
        end: 2015 June
        title: Lead Leader
        description: >
            Lorem ipsum dolorum...
            Multi-line
            comment.
            Test1.
        skills:
            reasoning:
                involvement: low
                complexity: moderate
                reason: preventing illogical conclusions
            consistency:
                involvement: high
                complexity: high
                reason: comprehension of static types
            repetition:
                involvement: moderate
                complexity: low
                reason: no reason no reason no reason no reason no reason no reason

projects:
    example-inator:
        description: >
            A device to produce examples with a deadly laser!
        repo: https://git.example.org/inator.git
        skills:
            Internet:
                involvement: high
                complexity: high
                reason: making the black blinky box go
            LibreOffice:
                involvement: low
                complexity: high
                reason: forgetting to write documentation
            Test:
                involvement: medium
                complexity: medium
                reason: finding a control case

education:
    Example University:
        start: 2000 January
        end: 1999 December
        certification: Test Certificate
        grade: It's at least a C+!
...
"#}

pub fn get_sample_resume() -> Resume {
    todo!(); //parse(YamlLoader::load_from_str(_get_sample_resume().unwrap()[0].clone())
}

use std::collections::HashMap;

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
    pub experience: String,
    pub projects: String,
    pub education: String,
}

#[derive(Debug, Copy, Clone)]
pub struct Language {
    written: Qualitative,
    spoken: Qualitative,
}

#[derive(Debug, Clone)]
pub struct Position {
    start: Date,
    end: Option<Date>,
    title: String,
    description: Option<String>,
    skills: HashMap<String, Skill>,
}

#[derive(Debug, Clone)]
pub struct Project {
    description: String,
    repo: Option<String>,
    skills: HashMap<String, Skill>,
}

#[derive(Debug, Clone)]
pub struct Credential {
    start: Date,
    end: Option<Date>,
    certification: String,
    grade: Option<String>,
}

#[derive(Debug, Copy, Clone)]
pub struct Date {
    year: u16,
    month: u8,
    day: u8,
}

#[derive(Debug, Clone)]
pub struct Skill {
    degree: Qualitative,
    level: Qualitative,
    reason: String,
}

#[derive(Debug, Copy, Clone)]
pub enum Qualitative {
    VeryLow,
    Low,
    Medium,
    High,
    VeryHigh,
}

pub fn parse(yml: Yaml) -> Result<Resume, &'static str> {
    todo!();
}
