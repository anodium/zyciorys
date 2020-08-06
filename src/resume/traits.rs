use super::types::*;

pub use yaml_rust::yaml::Yaml;

pub use std::str::FromStr;

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
        Date::parse_from_str($date, DEFAULT_STRFTIME_FMT).or(Err(String::from("FATAL: unable to parse date")))
    };

    ($date:expr, $fmt:expr) => {
        Date::parse_from_str($date, $fmt).or(Err(String::from("FATAL: unable to parse date")))
    };
}

impl FromStr for Resume {
    type Err = String;

    #[doc = ""] // TODO: Write documentation
    fn from_str(document: &str) -> std::result::Result<Self, <Self as std::str::FromStr>::Err> {
        Resume::parse(&Yaml::from_str(document))
    }
}

const DEFAULT_STRFTIME_FMT: &'static str = "%Y %b";

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
    pub fn parse(document: &Yaml) -> Result<Resume, <Resume as FromStr>::Err> {
        let langs = access!(document, languages, HashMap)?;
        let mut langs_repacked = HashMap::<String, Language>::new();

        for (key, value) in langs {
            let name = resolve!(key, str)?;
            let node = resolve!(value, HashMap)?;

            langs_repacked.insert(name.to_owned(), Language {
                written: Qualitative::from_str(access!(node, keyed!(written), str)?)?,
                spoken: Qualitative::from_str(access!(node, keyed!(spoken), str)?)?,
            });
        }

        let poss = access!(document, experience, HashMap)?;
        let mut poss_repacked = HashMap::<String, Position>::new();

        for (key, value) in poss {
            let skills_raw = access!(value, skills, Vec);

            poss_repacked.insert(String::from(resolve!(key, str)?), Position {
                start: strftime!(access!(value, start, str)?, DEFAULT_STRFTIME_FMT)?,
                end: strftime!(access!(value, end, str)?, DEFAULT_STRFTIME_FMT).ok(),
                title: String::from(access!(value, title, str)?),
                description: Some(String::from(access!(value, description, str)?)),
                skills: Vec::<Skill>::new(), // TODO: write a skills parser and make the skill struct more concrete. for now, an empty Vec<Skill>
            });
        }

        let prjs = document["projects"].as_hash().unwrap();
        let mut prjs_repacked = HashMap::<String, Project>::new();

        for node in prjs.into_iter() {
            prjs_repacked.insert(String::from(""), Project {
                name: String,
                description: String,
                repo: Option<String>,
                skills: Vec<Skill>,
            });
        }

        let creds = document["education"].as_hash().unwrap();
        let mut creds_repacked = HashMap::<String, Credential>::new();;

        for node in poss.into_iter() {
            creds_repacked.push(Credential {
                name: String,
                start: Date,
                end: Option<Date>,
                certification: String,
                grade: Option<String>,
            });
        }

        Ok(Resume {
            name: document["name"].as_str().unwrap().to_owned(),
            midname: document["midname"].as_str().map(str::to_string),
            lastname: document["lastname"].as_str().map(str::to_string),
            phone: document["phone"].as_str().map(str::to_string),
            email: document["email"].as_str().map(str::to_string),
            fingerprint: document["fingerprint"].as_str().map(str::to_string),
            website: document["website"].as_str().map(str::to_string),
            github: document["github"].as_str().map(str::to_string),
            reddit: document["reddit"].as_str().map(str::to_string),
            twitter: document["twitter"].as_str().map(str::to_string),
            address: document["address"].as_str().map(str::to_string),
            languages: langs_repacked,
            experience: todo!(),
            projects: todo!(),
            education: todo!(),
            skills: todo!(),
            document: &document
        })
    }
}

#[doc = "Performs the conversion

# Panics

This function will panic with `core::unimplemented!` if it can't match a
descriptor to a `zyciorys::resume::Qualitative`."] // TODO:
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

#[doc = "Performs the conversion"] // TODO:
impl From<Yaml> for Skill {
    #[inline]
    fn from(ident: Yaml) -> Self {
        <Self as FromStr>::from_str(ident).unwrap()
    }
}
