use crate::errors::Error;

use serde::{self, Deserialize};

type DateTime = chrono::DateTime<chrono::FixedOffset>;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Event {
    /// Short code describing the event
    ///
    /// TODO: Reverse engineer the possible values and their meaning
    pub code: String,

    #[serde(deserialize_with = "rfc3339::deserialize")]
    pub date: DateTime,

    pub label: String,

    pub order: u8,
}

/// Represents one of the five possible timeline events
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TimelineEvent {
    pub code: String,

    #[serde(default, deserialize_with = "rfc3339::deserialize_option")]
    pub date: Option<DateTime>,

    /// 1; 2; 3; 4 or 5
    pub id: u8,

    #[serde(rename = "shortLabel")]
    pub label: String,

    /// If true, this event is "achieved" or "unlocked".
    /// This probably implies that previous events are too.
    #[serde(rename = "status")]
    pub achieved: bool,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Shipment {
    /// Date on which the shipment entered the logistic circuit
    #[serde(deserialize_with = "rfc3339::deserialize")]
    pub entry_date: DateTime,

    /// Estimated delivery date
    #[serde(rename = "estimDate")]
    #[serde(deserialize_with = "rfc3339::deserialize")]
    pub estimated_date: DateTime,

    /// Arbitrary-length list of events associated with a date
    #[serde(rename = "event")]
    pub events: Vec<Event>,

    /// Shipment ID. Corresponds to the ID used to fetch all of this data.
    #[serde(rename = "idShip")]
    pub id: String,

    /// # Example
    ///
    /// `"colissimo"`
    #[serde(rename = "product")]
    pub service: String,

    /// A five-step timeline of key events that the shipment has or will "achieve"
    pub timeline: [TimelineEvent; 5],
}

impl Shipment {
    pub fn timeline_step(&self) -> u8 {
        self.timeline
            .iter()
            .find(|event| event.achieved)
            .map(|event| event.id)
            .unwrap_or(0)
    }
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct APIResponse {
    shipment: Option<Shipment>,
    return_message: Option<String>,
}

impl From<APIResponse> for Result<Shipment, Error> {
    fn from(api_response: APIResponse) -> Self {
        match api_response {
            APIResponse {
                shipment: Some(shipment),
                return_message: None,
            } => Result::Ok(shipment),
            APIResponse {
                shipment: None,
                return_message: Some(error),
            } => Result::Err(Error::Server(error)),
            _ => Result::Err(Error::Response),
        }
    }
}

mod rfc3339 {
    use super::*;
    use serde::{de::Error, Deserialize, Deserializer};

    pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<DateTime, D::Error> {
        let time: &str = Deserialize::deserialize(deserializer)?;
        DateTime::parse_from_rfc3339(time).map_err(D::Error::custom)
    }

    pub fn deserialize_option<'de, D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<Option<DateTime>, D::Error> {
        #[derive(Deserialize)]
        struct Wrapper(#[serde(deserialize_with = "deserialize")] DateTime);

        Ok(Option::deserialize(deserializer)?.map(|Wrapper(datetime)| datetime))
    }
}
