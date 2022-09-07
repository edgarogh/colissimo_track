pub mod errors;
pub mod model;

use crate::errors::Error;
use crate::model::{APIResponse, Shipment};

use hyper::{header, Body, Client, Request};
use hyper_tls::HttpsConnector;

fn extract_access_token(set_cookie_value: &str) -> Option<&str> {
    set_cookie_value.split(';').next()
}

pub async fn get_tracking_info(shipment_id: &str) -> Result<Shipment, Error> {
    if shipment_id.is_empty() || !shipment_id.chars().all(char::is_alphanumeric) {
        return Err(Error::IllegalParcelId);
    }

    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, Body>(https);

    // First request: Get access token in "Set Cookie" header

    let first_request = Request::builder()
        .uri(format!(
            "https://www.laposte.fr/outils/suivre-vos-envois?code={}",
            shipment_id,
        ))
        .body(Body::default())
        .map_err(|_| Error::IllegalParcelId)?;

    let first_response = client
        .request(first_request)
        .await
        .map_err(Error::Request)?;

    let cookie_header = first_response
        .headers()
        .get(header::SET_COOKIE)
        .ok_or(Error::Response)?;

    let cookie_header = cookie_header.to_str().map_err(|_| Error::Response)?;

    let access_token = extract_access_token(cookie_header).ok_or(Error::Response)?;

    // Second request: API

    let api_request = Request::builder()
        .uri(format!(
            "https://api.laposte.fr/ssu/v1/suivi-unifie/idship/{}?lang=fr_FR",
            shipment_id,
        ))
        .header(header::ACCEPT, "application/json")
        .header(header::COOKIE, access_token)
        .body(Body::default())
        .map_err(|_| Error::Response)?;

    let api_response = client.request(api_request).await.map_err(Error::Request)?;

    let body = hyper::body::to_bytes(api_response.into_body())
        .await
        .map_err(|_| Error::Response)?;

    let api_response: APIResponse = serde_json::from_slice(body.as_ref()).unwrap();

    api_response.into()
}
