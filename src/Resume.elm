module Resume exposing (Qualitative, Resume)

import Dict exposing (Dict)
import Yaml.Decode as Yaml


type alias Resume =
    { name : String
    , surname : String
    , phone : Maybe String
    , email : Maybe String
    , website : Maybe String
    , github : Maybe String
    , address : Maybe String
    , languages : Dict String Language
    , experience : Dict String Position
    , projects : Dict String Project
    , education : Dict String Credential
    }


type alias Language =
    { written : Qualitative, spoken : Qualitative }


type alias Position =
    { start : Date }


type alias Project =
    {}


type alias Credential =
    {}


type alias Date =
    {}


type Qualitative
    = VeryLow
    | Low
    | Medium
    | High
    | VeryHigh


intToQualitative : Int -> Maybe Qualitative
intToQualitative id =
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


stringToQualitative : String -> Maybe Qualitative
stringToQualitative id =
    Maybe.withDefault -1 (String.toInt id) |> intToQualitative



{--
decoder : Yaml.Decoder Resume
decoder =
    Yaml.map Resume {}
--}
