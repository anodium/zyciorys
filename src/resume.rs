use yaml_rust::{Yaml, YamlLoader};

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
    pub languages: Vec<Language>,
    pub experience: Vec<Position>,
    pub projects: Vec<Project>,
    pub education: Vec<Credential>,
}

#[derive(Debug, Clone)]
pub struct Language {
    name: String,
    written: Qualitative,
    spoken: Qualitative,
}

#[derive(Debug, Clone)]
pub struct Position {
    name: String,
    start: Date,
    end: Option<Date>,
    title: String,
    description: Option<String>,
    skills: Vec<Skill>,
}

#[derive(Debug, Clone)]
pub struct Project {
    description: String,
    repo: Option<String>,
    skills: Vec<Skill>,
}

#[derive(Debug, Clone)]
pub struct Credential {
    name: String,
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
    name: String,
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

pub fn load_from_str(source: &str) -> Result<Resume, &'static str> {
    todo!();
}

pub fn parse(document: Yaml) -> Resume {
    let langs:Box<[Yaml]> = document["languages"].as_vec().unwrap().into_boxed_slice();
    let mut langs_repacked:Vec<Language>;

    for node in langs.into_iter() {
        langs_repacked.push(Language {
            name: node.as_str().unwrap().to_owned(),
            written: match node["written"].as_str().unwrap() {
                "none" => Qualitative::VeryLow,
                "poor" => Qualitative::Low,
                _ => panic!()
            },
            spoken: match document["spoken"].as_str().unwrap() {
                "none" => Qualitative::VeryLow,
                "poor" => Qualitative::Low,
                _ => panic!()
            }
        });
    }

    Resume {
        name: document["name"].as_str().unwrap().to_owned(),
        midname: if let Some(s) = document["midname"].as_str() { Some(s.to_owned()) } else { None },
        lastname: if let Some(s) = document["lastname"].as_str() { Some(s.to_owned()) } else { None },
        phone: if let Some(s) = document["phone"].as_str() { Some(s.to_owned()) } else { None },
        email: if let Some(s) = document["email"].as_str() { Some(s.to_owned()) } else { None },
        fingerprint: if let Some(s) = document["fingerprint"].as_str() { Some(s.to_owned()) } else { None },
        website: if let Some(s) = document["website"].as_str() { Some(s.to_owned()) } else { None },
        github: if let Some(s) = document["github"].as_str() { Some(s.to_owned()) } else { None },
        reddit: if let Some(s) = document["reddit"].as_str() { Some(s.to_owned()) } else { None },
        twitter: if let Some(s) = document["twitter"].as_str() { Some(s.to_owned()) } else { None },
        address: if let Some(s) = document["address"].as_str() { Some(s.to_owned()) } else { None },
        languages: document["languages"].as_vec().unwrap().to_owned(),
        experience: document["experience"].as_vec().unwrap().to_owned(),
        projects: document["projects"].as_vec().unwrap().to_owned(),
        education: document["education"].as_vec().unwrap().to_owned()
    }
}

pub fn get_sample_resume() -> Resume {
    parse(YamlLoader::load_from_str(_get_sample_resume()).unwrap()[0])
}

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
