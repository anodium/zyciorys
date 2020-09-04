use serde::{Serialize, Deserialize};

#[path = "sample.rs"]
pub mod sample;

#[path = "prelude.rs"]
pub mod prelude;

#[path = "datestamp.rs"]
pub mod datestamp;
use datestamp::Datestamp;

pub use std::collections::HashMap;
pub use std::str::FromStr;

const DEFAULT_STRFTIME_FMT: &str = "%Y %b";

#[doc = ""] // TODO Write documentation
#[derive(Debug, Clone, Serialize, Deserialize)]
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
}

#[doc = ""] // TODO Write documentation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Language {
    written: Qualitative,
    spoken: Qualitative,
}

#[doc = ""] // TODO Write documentation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Position {
    start: Datestamp,
    end: Option<Datestamp>,
    title: String,
    description: Option<String>,
    skills: Vec<Skill>,
}

#[doc = ""] // TODO Write documentation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    description: String,
    repo: Option<String>,
    skills: Vec<Skill>,
}

#[doc = ""] // TODO Write documentation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Credential {
    start: Datestamp,
    end: Option<Datestamp>,
    certification: String,
    grade: Option<String>,
}

#[doc = ""] // TODO Write documentation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Skill {
    degree: Qualitative,
    level: Qualitative,
    reason: String,
}

#[doc = ""] // TODO Write documentation
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum Qualitative {
    VeryLow,
    Low,
    Medium,
    High,
    VeryHigh,
}

// XXX: Everything below this marked for deletion. Replacement is a serde::Deserialize impl.

#[doc = "# FOR INTERNAL USE ONLY"]
macro_rules! keyed {
    ($item:ident) => {
        &Yaml::String(String::from("$item"))
    };
}

#[doc = "# FOR INTERNAL USE ONLY"]
macro_rules! access {
    ($item:expr, $key:ident, $type:ident) => {
        resolve!($item["$key"], $type)
    };

    ($item:expr, $key:expr, $type:ident) => {
        resolve!($item[$key], $type)
    };
}

#[doc = "# FOR INTERNAL USE ONLY"]
macro_rules! resolve {
    ($item:expr, bool) => {
        $item.as_bool().ok_or(format!("FATAL: item `{:#?}` cannot be coerced into bool!", $item))
    };

    ($item:expr, i64) => {
        $item.as_i64().ok_or(format!("FATAL: item `{:#?}` cannot be coerced into i64!", $item))
    };

    ($item:expr, str) => {
        $item.as_str().ok_or(format!("FATAL: item `{:#?}` cannot be coerced into &str!", $item))
    };

    ($item:expr, HashMap) => {
        $item.as_hash().ok_or(format!("FATAL: item `{:#?}` cannot be coerced into YamlHash!", $item))
    };

    ($item:expr, Vec) => {
        $item.as_vec().ok_or(format!("FATAL: item `{:#?}` cannot be coerced into YamlArray!", $item))
    };
}

#[doc = "# FOR INTERNAL USE ONLY"]
macro_rules! strftime {
    ($date:expr) => {
        Datestamp::parse_from_str($date, DEFAULT_STRFTIME_FMT).or(Err(String::from("FATAL: unable to parse date")))
    };

    ($date:expr, $fmt:expr) => {
        Datestamp::parse_from_str($date, $fmt).or(Err(String::from("FATAL: unable to parse date")))
    };
}

impl FromStr for Resume {
    type Err = String;

    #[doc = ""] // TODO: Write documentation
    fn from_str(document: &str) -> std::result::Result<Self, <Self as std::str::FromStr>::Err> {
        Resume::parse(&Yaml::from_str(document))
    }
}

impl Resume {

    #[doc = "Parse a zyciorys Resume from a Yaml document

# Arguments

* `document` - a `yaml_rust::Yaml` document, see documentation for the
`yaml_rust` crate for more information

# Return value

* a `core::result::Result<zyciorys::resume::Resume, &'static str>` struct, see documentation for more information

# Panics

This function will not panic. If a required field is missing, or if a field has an
invalid value, a Err<&'static str> will be returned explaining it. All optional
fields are encapsulated within a `core::option::Option<T>`. Please refer to the
`zyciorys::resume::Resume` struct for more information. The documentation for
each field's struct describes valid values."]
    pub fn parse<'a>(document: Yaml) -> Result<Resume<'a>, <Resume<'static> as FromStr>::Err> {
        let langs = access!(document, languages, HashMap)?;
        let mut langs_repacked = HashMap::<String, Language>::new();

        for (key, value) in langs {
            let node = resolve!(value, HashMap)?;

            langs_repacked.insert(resolve!(key, str)?.into(), Language {
                written: Qualitative::from_str(access!(node, keyed!(written), str)?)?,
                spoken: Qualitative::from_str(access!(node, keyed!(spoken), str)?)?,
            });
        }

        let poss = access!(document, experience, HashMap)?;
        let mut poss_repacked = HashMap::<String, Position>::new();

        for (key, value) in poss {
            let skills_raw = access!(value, skills, Vec);

            poss_repacked.insert(resolve!(key, str)?.into(), Position {
                start: strftime!(access!(value, start, str)?, DEFAULT_STRFTIME_FMT)?,
                end: strftime!(access!(value, end, str)?, DEFAULT_STRFTIME_FMT).ok(),
                title: access!(value, title, str)?.into(),
                description: Some(access!(value, description, str)?.into()),
                skills: Vec::<Skill>::new(), // TODO: write a skills parser and make the skill struct more concrete. for now, an empty Vec<Skill>
            });
        }

        let prjs = access!(document, projects, HashMap)?;
        let mut prjs_repacked = HashMap::<String, Project>::new();

        for (key, value) in prjs {
            prjs_repacked.insert(resolve!(key, str)?.into(), Project {
                description: access!(value, description, str)?.into(),
                repo: Some(access!(value, repo, str)?.into()),
                skills: Vec::<Skill>::new(), // TODO:
            });
        }

        let creds = access!(document, education, HashMap)?;
        let mut creds_repacked = HashMap::<String, Credential>::new();

        for (key, value) in poss {
            creds_repacked.insert(resolve!(key, str)?.into(), Credential {
                start: strftime!(access!(value, start, str)?, DEFAULT_STRFTIME_FMT)?,
                end: strftime!(access!(value, end, str)?, DEFAULT_STRFTIME_FMT).ok(),
                certification: access!(value, certification, str)?.into(),
                grade: Some(access!(value, grade, str)?.into()),
            });
        }

        Ok(Resume {
            name: access!(document, name, str)?.into(),
            midname: Some(access!(document, midname, str)?.into()),
            lastname: Some(access!(document, lastname, str)?.into()),
            phone: Some(access!(document, phone, str)?.into()),
            email: Some(access!(document, email, str)?.into()),
            fingerprint: Some(access!(document, fingerprint, str)?.into()),
            website: Some(access!(document, website, str)?.into()),
            github: Some(access!(document, github, str)?.into()),
            reddit: Some(access!(document, reddit, str)?.into()),
            twitter: Some(access!(document, twitter, str)?.into()),
            address: Some(access!(document, address, str)?.into()),
            languages: langs_repacked,
            experience: poss_repacked,
            projects: prjs_repacked,
            education: creds_repacked,
            skills: Vec::<Skill>::new(), // TODO:
        })
    }
}

#[doc = "Performs the conversion

# Panics

This function will panic with `core::unimplemented!` if it can't match a
descriptor to a `zyciorys::resume::Qualitative`."] // TODO: Write documentation
impl From<&str> for Qualitative {
    #[inline]
    fn from(ident: &str) -> Self {
        <Self as FromStr>::from_str(ident).unwrap()
    }
}

#[doc = "Performs the conversion from &str to Qualitative

This function matches all the possible variations of phrasing that's usable
within a zyciorys document. I don't know what to use so I just picked these
keywords to start, maybe more could be added? Maybe it'll bring about a parsing
nightmare later...

# Arguments

* `expr` - a `str` that has a qualitative descriptor word

# Return value

* a `zyciorys::resume::Qualitative` enum, see documentation for more information
"]
impl FromStr for Qualitative {
    type Err = String;

    fn from_str(ident: &str) -> std::result::Result<Self, <Self as FromStr>::Err> {
        match ident {
            "none" => Ok(Qualitative::VeryLow),
            "poor" | "low" => Ok(Qualitative::Low),
            "okay" | "moderate" => Ok(Qualitative::Medium),
            "good" | "high" => Ok(Qualitative::High),
            "excelllent" => Ok(Qualitative::VeryHigh),
            _ => Err(String::from("Bad value for std::str, cannot perform conversion into zyciorys::resume::Qualitative"))
        }
    }
}

#[doc = "Performs the conversion"] // TODO: Write documentation
impl From<Yaml> for Skill {
    #[inline]
    fn from(ident: Yaml) -> Self {
        <Self as FromStr>::from_str(ident).unwrap()
    }
}
