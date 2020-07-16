use yaml_rust::{Yaml, YamlLoader};
use linked_hash_map::LinkedHashMap;
// FIXME: the `datetime` crate uses libc externs, find no_std replacement!
//use datetime::LocalDate as Date;

use super::macros;

#[macros::nop]
#[derive(Debug, Clone)]
#[doc = ""] // TODO
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
    //skills: Vec<&'a Skill>,
}

// TODO: Generation of these needs to be in a macro
impl Resume {
    pub const NAME_KEY: Yaml = Yaml::from_str("name");
}

#[doc = ""] // TODO
#[derive(Debug, Clone)]
pub struct Language {
    name: String,
    written: Qualitative,
    spoken: Qualitative,
}

#[doc = ""] // TODO
#[derive(Debug, Clone)]
pub struct Position {
    name: String,
    start: Date,
    end: Option<Date>,
    title: String,
    description: Option<String>,
    skills: Vec<Skill>,
}

#[doc = ""] // TODO
#[derive(Debug, Clone)]
pub struct Project {
    name: String,
    description: String,
    repo: Option<String>,
    skills: Vec<Skill>,
}

#[doc = ""] // TODO
#[derive(Debug, Clone)]
pub struct Credential {
    name: String,
    start: Date,
    end: Option<Date>,
    certification: String,
    grade: Option<String>,
}

#[doc = ""] // TODO
#[derive(Debug, Clone)]
pub struct Skill {
    name: String,
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

#[doc = ""] // TODO
pub fn load_from_str(source: &str) -> Result<Resume, &'static str> {
    todo!();
}

#[doc = "Parse a zyciorys Resume from a Yaml document

# Arguments

* `document` - a `yaml_rust::Yaml` document, see documentation for the
`yaml_rust` crate for more information

# Return value

* a `zyciorys::resume::Resume` struct, see documentation for more information

# Panics

This function will panic if a required field is missing, or if a field has an
invalid value. All optional fields are encapsulated within a
`core::option::Option<T>`. Please refer to the `zyciorys::resume::Resume` struct
for more information. The documentation for each field's struct describes valid
values."]
pub fn parse(document: Yaml) -> Resume {
    let langs: &LinkedHashMap<Yaml, Yaml> = document["languages"].as_hash().unwrap();
    let mut langs_repacked: Vec<Language>;

    // let node: Iter<&Yaml, &Yaml>;
    for node in langs.iter() {
        let name: String = node.0.as_str().unwrap().into();
        let unwrapped_node: &LinkedHashMap<Yaml, Yaml> = node.1.as_hash().unwrap();

        langs_repacked.push(Language {
            name: name,
            written: _str_to_qualitative(unwrapped_node[&Yaml::from_str("written")].as_str().unwrap()),
            spoken: _str_to_qualitative(unwrapped_node[&Yaml::from_str("spoken")].as_str().unwrap()),
        });
    }

    let poss: &LinkedHashMap<Yaml, Yaml> = document["experience"].as_hash().unwrap();
    let mut poss_repacked: Vec<Position>;

    for node in poss.iter() {
        poss_repacked.push(Position {
            name: node.0.as_str().unwrap().into(),
            start: Date::from_str(node.1.as_hash().unwrap()[&Yaml::from_str("start")].as_str().unwrap()).unwrap(),
            end: if let Some(v) = node.1.as_hash().unwrap()[&Yaml::from_str("end")].as_str() {
                Some(Date::from_str(v).unwrap())
            } else { 
                None
            },
            title: node.1.as_hash().unwrap()[&Yaml::from_str("title")].as_str().unwrap().into(),
            description: if let Some(v) = Option<String>,
            skills: Vec<Skill>,
        });
    }

    let prjs: &LinkedHashMap<Yaml, Yaml> = document["projects"].as_hash().unwrap();
    let mut prjs_repacked: Vec<Project>;

    for node in prjs.into_iter() {
        prjs_repacked.push(Project { // FIXME: Add values from `document`
            name: String,
            description: String,
            repo: Option<String>,
            skills: Vec<Skill>,
        });
    }

    let creds: &LinkedHashMap<Yaml, Yaml> = document["education"].as_hash().unwrap();
    let mut creds_repacked: Vec<Credential>;

    for node in poss.into_iter() {
        creds_repacked.push(Credential {
            name: String,
            start: Date,
            end: Option<Date>,
            certification: String,
            grade: Option<String>,
        });
    }

    // TODO: Finish implementing
    Resume {
        name: document["name"].as_str().unwrap().to_owned(),
        midname: _reown_and_repack(document["midname"].as_str()),
        lastname: _reown_and_repack(document["lastname"].as_str()),
        phone: _reown_and_repack(document["phone"].as_str()),
        email: _reown_and_repack(document["email"].as_str()),
        fingerprint: _reown_and_repack(document["fingerprint"].as_str()),
        website: _reown_and_repack(document["website"].as_str()),
        github: _reown_and_repack(document["github"].as_str()),
        reddit: _reown_and_repack(document["reddit"].as_str()),
        twitter: _reown_and_repack(document["twitter"].as_str()),
        address: _reown_and_repack(document["address"].as_str()),
        languages: langs_repacked,
        experience: todo!(),
        projects: todo!(),
        education:todo!()
    }
}

#[inline]
fn _reown_and_repack(opt: Option<&str>) -> Option<String> {
    if let Some(t) = opt {
        Some(t.to_owned())
    } else {
        None
    }
}

#[doc = "Match a str to a Qualitative

This function matches all the possible variations of phrasing that's usable
within a zyciorys document. I don't know what to use so I just picked these
keywords to start, maybe more could be added? Maybe it'll bring about a parsing
nightmare later...

# Arguments

* `input` - a `str` that has a qualitative descriptor word

# Return value

* a `zyciorys::resume::Qualitative` enum, see documentation for more information

# Panics

This function will panic with `core::unimplemented!` if it can't match a
descriptor to a `zyciorys::resume::Qualitative`."]
#[inline]
fn _str_to_qualitative(input: &str) -> Qualitative { // TODO: Should I make this implement Qualitative::into()?
    match input {
        "none" => Qualitative::VeryLow,
        "poor" => Qualitative::Low,
        "okay" => Qualitative::Medium,
        "good" => Qualitative::High,
        "1010" => Qualitative::VeryHigh,
        _ => unimplemented!()
    }
}

#[doc = ""] // TODO
pub fn get_sample_resume() -> Resume {
    parse(YamlLoader::load_from_str(SAMPLE_RESUME).unwrap()[0].clone())
}

#[doc = ""] // TODO
pub const SAMPLE_RESUME: &'static str =
r#"--- !<zyciorys::resume::SAMPLE_RESUME>
name: Saniyya
surname: Dane

phone: +11 1111 1111 11
email: saniyya@dane.example.com
fingerprint: AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA
website: https://saniyya.example.net/
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
"#;
