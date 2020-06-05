extern crate chrono;
extern crate hyper;
extern crate hyper_tls;
extern crate serde;
extern crate serde_json;

mod errors;
mod model;

use crate::errors::Error;
use crate::model::{APIResponse, Shipment};

use hyper::{header, Body, Client, Request};
use hyper_tls::HttpsConnector;

fn extract_access_token(set_cookie_value: &str) -> Option<&str> {
    let end = set_cookie_value.find(';')?;
    Some(&set_cookie_value[..end])
}

pub async fn get_tracking_info(shipment_id: &str) -> Result<Shipment, Error> {
    if !shipment_id.chars().all(char::is_alphanumeric) {
        return Result::Err(Error::IllegalParcelId);
    }

    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);

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
        .map_err(|_| Error::Request)?;

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

    let api_response = client
        .request(api_request)
        .await
        .map_err(|_| Error::Request)?;

    let body = hyper::body::to_bytes(api_response.into_body())
        .await
        .map_err(|_| Error::Response)?;

    let body = String::from_utf8(body.to_vec()).map_err(|_| Error::Response)?;

    println!("{:?}", body);

    let api_response: APIResponse = serde_json::from_str(&body).unwrap();

    api_response.into()
}
