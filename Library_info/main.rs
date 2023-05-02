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
