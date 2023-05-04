use std::io;
use chrono::{DateTime, Utc, TimeZone};
use serde::{Deserialize, Serialize};
use reqwest::Error;
use hyper::Client;
use hyper_rustls::HttpsConnector;
use oauth2::{
    basic::BasicClient, reqwest::http_client, AccessToken, AuthUrl, ClientId, ClientSecret,
    RedirectUrl, Scope, TokenUrl,
};
use google_calendar3::{api::EventsInsertCall, CalendarHub};

// Googleのアクセストークンを取得する定義
async fn authenticate_google() -> AccessToken {
    // ここに `authenticate_google` 関数の実装を追加
}
#[derive(Debug, Deserialize, Serialize)]
struct Book {
    title: String,
    author: String,
    due_date: DateTime<Utc>,
}
