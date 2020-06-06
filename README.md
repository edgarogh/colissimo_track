# colissimo_track

This crate allows you to fetch information about the status of a "Colissimo" shipment and its delivery progress (from the receiver's perspective). It uses a reverse-engineered private but not-so-complicated API used by the web page you are supposed to browse as a human.

The crate is very simple, it exports a few structs and an `async fn get_tracking_info(shipment_id: &str) -> Result<Shipment, Error>` function to query the shipment API.

## How does it work ?

 * A first request to `https://www.laposte.fr/outils/suivre-vos-envois?code={}` is made with `hyper`. The only interesting part we keep is the `access-token=[...jwt...]` Set-Cookie header.
 * A second request, to `https://api.laposte.fr/ssu/v1/suivi-unifie/idship/{}?lang=fr_FR`, is made with the cookie. The server should, under normal conditions, respond with a JSON object representing the shipment.
 * A bit of `serde` magic is used to deserialize this object into a provided data structure (see `crate::model::*`).

## Target developers / users / projects

This crate covers the very niche market of Colissimo users, that is, people living in locations where the national public French group "La Poste" operates its parcel delivery services. Since the library allows parcel tracking from the delivery receiver's perspective, the only usages I can find are various client softwares, like mobile apps or home (automation) dashboards.

## I-am-a-beginner disclaimer

This is literally my first crate, and my first "finished" Rust project. While I'd love to see this crate being used (although very niche, I admit), make sure to take a look at [the code inside](https://github.com/edgarogh/colissimo_track/tree/master/src) before putting yourself and your project at potential risk !

Obviously any feedback, comment, bug report, contribution, or anything else that can help improve this crate `&&`/`||` my Rust programming level are 100% welcome.

## License

The code inside this repository is licensed under the MIT license.
