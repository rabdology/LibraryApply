use crate::book::Book;
use chrono::offset::Utc;
use google_calendar3::api::Event;
use google_calendar3::CalendarHub;
use hyper::net::HttpsConnector;
use hyper_rustls::TlsClient;
use std::default::Default;

pub async fn add_return_event(book: &Book) -> Result<(), Box<dyn std::error::Error>> {
    let secret = yup_oauth2::read_application_secret("client_secret.json")
        .await
        .expect("client_secret.jsonが見つかりませんでした。");
    let auth = yup_oauth2::InstalledFlowAuthenticator::builder(secret, yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect)
        .persist_tokens_to_disk("token_cache.json")
        .build()
        .await?;
    let http = hyper::Client::with_connector(HttpsConnector::new(TlsClient::new()));

    let hub = CalendarHub::new(http, auth);

    let event_start = book.due_date.and_hms(9, 0, 0).with_timezone(&Utc);
    let event_end = book.due_date.and_hms(10, 0, 0).with_timezone(&Utc);

    let mut event = Event::default();
    event.summary = Some(format!("{}を返却", book.title));
    event.start = Some(EventDateTime {
        date_time: Some(event_start.to_rfc3339()),
        ..Default::default()
    });
    event.end = Some(EventDateTime {
        date_time: Some(event_end.to_rfc3339()),
        ..Default::default()
    });

    let calendar_id = "primary"; // ここでカレンダーIDを指定します。
    let result = hub.events().insert(event, calendar_id).execute().await?;

    if let Some(event_id) = result.id {
        println!("イベントが作成されました。ID: {}", event_id);
    } else {
        println!("イベントの作成に失敗しました。");
    }

    Ok(())
}
