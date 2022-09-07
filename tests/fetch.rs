use colissimo_track::get_tracking_info;

const VAR: &str = "COLISSIMO_TRACK_TEST_ID";

#[tokio::test]
async fn fetch() {
    if let Ok(parcel) = std::env::var(VAR) {
        dbg!(get_tracking_info(&parcel).await.unwrap());
    } else {
        eprintln!("no {} set, test didn't run", VAR);
    }
}
