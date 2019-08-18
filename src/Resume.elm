module Resume exposing (Qualitative, Resume)

import Dict exposing (Dict)
import Json.Decode as Decode exposing (..)
import Json.Decode.Pipeline exposing (required, optional, hardcoded)
import ISO8601 exposing (Time, fromString, sub)
--import Yaml.Decode as Yaml


type alias Resume =
    { name : String
    , midname : Maybe String
    , surname : Maybe String
    , phone : Maybe String
    , email : Maybe String
    , fingerprint : Maybe String
    , website : Maybe String
    , github : Maybe String
    , reddit : Maybe String
    , twitter : Maybe String
    , address : Maybe String
    , languages : Dict String Language
    , experience : Dict String Position
    , projects : Dict String Project
    , education : Dict String Credential
    }


type alias Language =
    { written : Qualitative, spoken : Qualitative }


type alias Position =
    { start : Date
    , end : Maybe Date
    , title : String
    , description : Maybe String
    , skills : Dict String Skill
    }


type alias Project =
    { description : String
    , repo : Maybe String
    , skills : Dict String Skill
    }


type alias Credential =
    { start : Date
    , end : Maybe Date
    , certification : String
    , grade : Maybe String
    }


type alias Date =
    { year : Int
    , month : Int
    , day : Int
    }


type alias Skill =
    { degree : Qualitative
    , level : Qualitative
    , reason : String
    }


type Qualitative
    = VeryLow
    | Low
    | Medium
    | High
    | VeryHigh


intToMaybeQualitative : Int -> Maybe Qualitative
intToMaybeQualitative id =
    case id of
        0 ->
            Just VeryLow

        1 ->
            Just Low

        2 ->
            Just Medium

        3 ->
            Just High

        4 ->
            Just VeryHigh

        _ ->
            Nothing


intToQualitative : Int -> Result String Qualitative
intToQualitative id =
        case id of
        0 ->
            Ok VeryLow

        1 ->
            Ok Low

        2 ->
            Ok Medium

        3 ->
            Ok High

        4 ->
            Ok VeryHigh

        _ ->
            Err "Invalid Qualitative ID"

stringToDate : String -> Result String Date
stringToDate s =
    case ISO8601.fromString s of
        Ok v ->
            Ok (timeToDate v)

        Err m ->
            Err m


QualitativeDecoder : Decoder (String -> b) -> Decoder b
QualitativeDecoder a b =
    -- TODO: Json decoder for Qualitative type


timeToDate : Time -> Date
timeToDate t =
    { year = t.year
    , month = t.month
    , day = t.day
    }

decoder : Decoder Resume
decoder =
    Decode.succeed Resume
        |> required "name" string
        |> optional "midname" (nullable string) Nothing
        |> optional "surname" (nullable string) Nothing
        |> optional "phone" (nullable string) Nothing
        |> optional "email" (nullable string) Nothing
        |> optional "fingerprint" (nullable string) Nothing
        |> optional "website" (nullable string) Nothing
        |> optional "github" (nullable string) Nothing
        |> optional "reddit" (nullable string) Nothing
        |> optional "twitter" (nullable string) Nothing
        |> optional "address" (nullable string) Nothing
        |> required "languages" (dict (Decode.map2 Language
                                        (required "written" int)
                                        (intToQualitative (required "spoken" int))))
        |> required "experience" (dict (Decode.map5 Position
                                         (stringToDate (required "start" string))
                                         (stringToDate (optional "end" (nullable string)))
                                         (required "title" string)
                                         (optional "description" (nullable string) Nothing)
                                         (required "skills" (dict (Decode.map3 Skill
                                                                    (intToQualitative (required "degree" int))
                                                                    (intToQualitative (required "level" int))
                                                                    (required "reason" string))))))
        |> required "projects" () -- FIXME: Finish structs
        |> required "education" ()



{--
type alias Name =
    { name : String
    , midname : Maybe String
    , surname : Maybe String}

decoder : Yaml.Decoder Resume
decoder =
    Yaml.map6 Resume
        (Yaml.map3 Name -- Name[s]
            (Yaml.field ["name"] Yaml.string)
            (Yaml.sometimes ["midname"] Yaml.string)
            (Yaml.sometimes ["surname"] Yaml.string))
        (Yaml.map8 -- Contact Info
            (Yaml.sometimes ["phone"] Yaml.string)
            (Yaml.sometimes ["email"] Yaml.string)
            (Yaml.sometimes ["fingerprint"] Yaml.string)
            (Yaml.sometimes ["website"] Yaml.string)
            (Yaml.sometimes ["github"] Yaml.string)
            (Yaml.sometimes ["reddit"] Yaml.string)
            (Yaml.sometimes ["twitter"] Yaml.string)
            (Yaml.sometimes ["address"] Yaml.string))
        (Yaml.field "languages" -- Languages
            (Yaml.list
                (Yaml.map2 Language
                    (Yaml.at ["written"] Yaml.int)
                    (Yaml.at ["spoken"] Yaml.int))))
        (Yaml.succeed) -- Experience
        (Yaml.succeed) -- Projects
        (Yaml.succeed) -- Education
--}
