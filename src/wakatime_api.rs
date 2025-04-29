use std::cmp::PartialEq;
use std::fmt::format;
use reqwest::blocking::RequestBuilder;
use serde::Deserialize;
use crate::config::{Config, Platform};
use crate::wakatime_error::WakatimeApiError;




//**
//*A user's coding activity for the given time range. Optional range can be a YYYY year, YYYY-MM month, or one of last_7_days, last_30_days, last_6_months, last_year, or all_time. When range isn’t present, the user’s public profile range is used. For accounts subscribed to the free plan, time ranges >= one year are updated on the first request. It’s best to always check is_up_to_date and retry your request when the response is stale. Stats are read-only representations of Heartbeats, Durations, and Summaries, created by joining multiple Heartbeats together when they’re within 15 minutes of each other. The 15 minutes default can be changed with your account’s Keystroke Timeout preference.
//*
#[derive(Debug, Clone)]
pub enum Range {
    Today,
    Yesterday,
    Week,
    Month,
    This7Days,
    Last7Days,
    This30Days,
    Last30Days,
    This6Months,
    Last6Months,
    This12Month,
    LastYear,
    AllTime,
    Any,
}


impl Range {
    pub fn as_str(&self, config: &Config) -> String {
        let is_wakapi = config.get_wakatime_platform().eq(&Platform::Wakapi);

        match self {
            Range::Today => self.platform_specific(is_wakapi, "today", || panic!("Today is not supported for Wakatime platform")),
            Range::Yesterday => self.platform_specific(is_wakapi, "yesterday", || panic!("Yesterday is not supported for Wakatime platform")),
            Range::Week => self.platform_specific(is_wakapi, "week", || "last_7_days".to_string()),
            Range::Month => self.platform_specific(is_wakapi, "month", || "last_30_days".to_string()),
            Range::This7Days => self.platform_specific(is_wakapi, "7_days", || "last_7_days".to_string()),
            Range::Last7Days => "last_7_days".to_string(),
            Range::This30Days => self.platform_specific(is_wakapi, "30_days", || "last_30_days".to_string()),
            Range::Last30Days => "last_30_days".to_string(),
            Range::This6Months => self.platform_specific(is_wakapi, "6_months", || "last_6_months".to_string()),
            Range::Last6Months => "last_6_months".to_string(),
            Range::This12Month => self.platform_specific(is_wakapi, "12_months", || "last_year".to_string()),
            Range::LastYear => "last_year".to_string(),
            Range::AllTime => "all_time".to_string(),
            Range::Any => self.platform_specific(is_wakapi, "any", || "all_time".to_string()),
        }
    }

    fn platform_specific<F>(&self, is_wakapi: bool, wakapi_value: &str, wakatime_fn: F) -> String
    where
        F: FnOnce() -> String
    {
        if is_wakapi {
            wakapi_value.to_string()
        } else {
            wakatime_fn()
        }
    }

    pub fn to_string(&self, config: &Config) -> String {
        self.as_str(config)
    }
}
#[derive(Deserialize, Debug)]
pub struct Statistic {
    data: StatisticData,
}

#[derive(Deserialize, Debug)]
pub struct StatisticData {
    username: Option<String>,
    user_id: String,
    start: String,
    end: String,
    status: String,
    total_seconds: f64,
    daily_average: f64,
    days_including_holidays: u64,
    range: String,
    human_readable_range: String,
    human_readable_total: String,
    human_readable_daily_average: String,
    is_coding_activity_visible: Option<bool>,
    is_other_usage_visible: Option<bool>,
    editors: Vec<WrappedStatistic>,
    languages: Vec<WrappedStatistic>,
    machines: Vec<WrappedStatistic>,
    projects: Vec<WrappedStatistic>,
    operating_systems: Vec<WrappedStatistic>,
    categories: Vec<WrappedStatistic>,
}

#[derive(Deserialize, Debug)]
pub struct WrappedStatistic {
    digital: String,
    hours: u64,
    minutes: u64,
    name: String,
    percent: f64,
    seconds: Option<u64>,
    text: String,
    total_seconds: f64,
}

impl Statistic {
    fn new() -> Self {
        Statistic {
            data: StatisticData::default(),
        }
    }

    pub fn get_data(&self) -> &StatisticData {
        &self.data
    }
}

// Add Default implementation for StatisticData
impl Default for StatisticData {
    fn default() -> Self {
        StatisticData {
            username: None,
            user_id: String::new(),
            start: String::new(),
            end: String::new(),
            status: String::new(),
            total_seconds: 0.,
            daily_average: 0.0,
            days_including_holidays: 0,
            range: String::new(),
            human_readable_range: String::new(),
            human_readable_total: String::new(),
            human_readable_daily_average: String::new(),
            is_coding_activity_visible: None,
            is_other_usage_visible: None,
            editors: Vec::new(),
            languages: Vec::new(),
            machines: Vec::new(),
            projects: Vec::new(),
            operating_systems: Vec::new(),
            categories: Vec::new(),
        }
    }
}

impl StatisticData {
    pub fn get_username(&self) -> &str {
        self.username.as_deref().unwrap_or("current")
    }
    pub fn get_total_seconds(&self) -> u64 {
        self.total_seconds as u64
    }
    pub fn get_daily_average(&self) -> f64 {
        self.daily_average
    }
    pub fn get_days_including_holidays(&self) -> u64 {
        self.days_including_holidays
    }
    pub fn get_range(&self) -> &str {
        &self.range
    }
    pub fn get_human_readable_range(&self) -> &str {
        &self.human_readable_range
    }
    pub fn get_human_readable_total(&self) -> &str {
        &self.human_readable_total
    }
    pub fn get_human_readable_daily_average(&self) -> &str {
        &self.human_readable_daily_average
    }

    pub fn get_is_coding_activity_visible(&self) -> bool {
        self.is_coding_activity_visible.unwrap_or(false)
    }

    pub fn get_is_other_usage_visible(&self) -> bool {
        self.is_other_usage_visible.unwrap_or(false)
    }

    pub fn get_editors(&self) -> &Vec<WrappedStatistic> {
        &self.editors
    }

    pub fn get_languages(&self) -> &Vec<WrappedStatistic> {
        &self.languages
    }

    pub fn get_machines(&self) -> &Vec<WrappedStatistic> {
        &self.machines
    }

    pub fn get_projects(&self) -> &Vec<WrappedStatistic> {
        &self.projects
    }

    pub fn get_operating_systems(&self) -> &Vec<WrappedStatistic> {
        &self.operating_systems
    }

    pub fn get_categories(&self) -> &Vec<WrappedStatistic> {
        &self.categories
    }
}

impl WrappedStatistic {
    pub fn get_name(&self) -> &str {
        &self.name
    }
    pub fn get_hours(&self) -> u64 {
        self.hours
    }
    pub fn get_minutes(&self) -> u64 {
        self.minutes
    }

    pub fn get_seconds(&self) -> u64 {
        self.seconds.unwrap_or(self.total_seconds as u64)
    }
    pub fn get_text(&self) -> &str {
        &self.text
    }

    pub fn get_total_seconds(&self) -> u64 {
        self.total_seconds as u64
    }

    pub fn get_percent(&self) -> f64 {
        self.percent
    }

    pub fn get_digital(&self) -> &str {
        &self.digital
    }

    pub fn get_percent_as_string(&self) -> String {
        format!("{:.2}%", self.percent)
    }

    pub fn get_total_seconds_as_string(&self) -> String {
        format!("{:.2}", self.total_seconds as f64 / 3600.0)
    }

    pub fn get_hours_as_string(&self) -> String {
        format!("{:.2}", self.hours as f64)
    }
}

fn create_req(f_url: &str, config: &Config) -> RequestBuilder {
    let mut base_url = config.get_wakatime_url().to_string();
    let user = config.get_wakatime_user();
    if !base_url.ends_with('/') && !f_url.starts_with('/') {
        base_url.push('/');
    }
    let url_with_base = format!("{}{}", base_url, f_url);
    let final_url = url_with_base.replacen("{}", &user, 1);

    let client = reqwest::blocking::Client::new();

    let authorization_header = if config.get_wakatime_platform().eq(&Platform::Wakapi) {
        format!("Bearer {}", config.get_wakatime_api_token())
    } else {
        let base64_token = base64::Engine::encode(
            &base64::engine::general_purpose::STANDARD,
            config.get_wakatime_api_token()
        );
        format!("Basic {}", base64_token)
    };

    client
        .get(final_url)
        .header("Authorization", authorization_header)
        .header("Accept", "application/json")
}
pub fn get_from_range(range: Range, config: &Config) -> Result<Statistic, WakatimeApiError> {
    let url = format!("/v1/users/{{}}/stats/{}", range.as_str(config));
    get_from(&url, config)
}
pub fn get_from(url: &str, config: &Config) -> Result<Statistic, WakatimeApiError> {
    let request = create_req(url, config);
    let response = request.send()?;

    // Check status before extracting text
    if !response.status().is_success() {
        return Err(WakatimeApiError::RequestError(
            response.error_for_status().unwrap_err(),
        ));
    }

    // Extract the text (moves response)
    let text = response.text()?;
    println!("Response body: {}", text);

    // Deserialize the response
    match serde_json::from_str::<Statistic>(&text) {
        Ok(statistic) => Ok(statistic),
        Err(e) => {
            eprintln!("Serialize error: {}", e);
            // Create a compatible error using io::Error as an intermediary
            let io_error = std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("Deserialization error: {}", e)
            );
            Err(WakatimeApiError::DeserializationError(io_error))
        }
    }
}