use serde::{Deserialize, Serialize};

type URL<'a> = &'a str;

#[derive(Serialize, Deserialize)]
pub struct PropertyValue {}

#[derive(Serialize, Deserialize)]
pub struct Action {}

pub enum Text<'a> {
    URL(URL<'a>),
    PropertyValue(PropertyValue),
    Text(&'a str),
}

#[derive(Serialize, Deserialize)]
pub struct Thing<'a> {
    #[serde(rename = "additionalType")]
    pub additional_type: URL<'a>,
    #[serde(rename = "alternateName")]
    pub alternate_name: &'a str,
    pub description: &'a str,
    #[serde(rename = "disambiguatingDescription")]
    pub disambiguating_description: &'a str,
    // pub identifier: Text<'a>,
    // pub image
    // main_entity_of_page:
    name: &'a str,
    #[serde(rename = "potentialAction")]
    potential_action: Action,
    #[serde(rename = "sameAs")]
    same_as: URL<'a>, // subjectOf
    url: URL<'a>,
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
