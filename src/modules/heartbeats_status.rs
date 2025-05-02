use chrono::{Datelike, format};

use crate::lang_icons;
use crate::md_modules::MdModule;
use crate::wakatime_api::{HeartBeat, HeartBeats, Range, StatisticData};

pub struct HeartBeatsStatus {
    name: String,
}

impl HeartBeatsStatus {
    pub fn new() -> Self {
        HeartBeatsStatus {
            name: "What I'm doing now".to_string(),
        }
    }
    pub fn new_with_name(name: String) -> Self {
        HeartBeatsStatus { name }
    }
}

static mut HB_COUNT: usize = 0;
static mut LAST_CHANGE: i64 = 0;
const SLEEP_MESSAGE: &str =
    "> I'm currently sleeping 🛌 or I'm busy with other things than coding. \n\n";
impl MdModule for HeartBeatsStatus {
    fn render(&self, _: &StatisticData, heartbeats: &HeartBeats) -> String {
        let mut result = format!("## {} \n\n", self.name());
        //result.push_str("```text\n");
        let data = heartbeats.get_data();
        let count = data.len();
        unsafe {
            if HB_COUNT != count {
                LAST_CHANGE = chrono::Utc::now().timestamp();
            }
        }

        // if + 10 minutes from LAST_CHANGE
        if count == 0 || unsafe { chrono::Utc::now().timestamp() - LAST_CHANGE > 600 } {
            return SLEEP_MESSAGE.to_string();
        }

        let last: &HeartBeat = data.iter().last().unwrap();
        let last_chrono = chrono::Utc::now().timestamp() - last.get_time() as i64;

        if last.get_time() == 0 || last_chrono > 600 {
            return SLEEP_MESSAGE.to_string();
        }
        let icon_html = format!(
            "<img src=\"{}\" alt=\"{}\" width=\"20\" height=\"20\">",
            lang_icons::search_icon(last.get_language()),
            last.get_language()
        );
        let time: &str = {
            let timestamp = last.get_time();

            // Conversion du timestamp en DateTime<Utc>
            let datetime_utc = chrono::DateTime::<chrono::Utc>::from_timestamp(timestamp as i64, 0)
                .expect("Timestamp invalide");

            // Nous allons appliquer manuellement les décalages horaires
            // En mai, nous sommes en heure d'été en France et aux États-Unis

            // France (Paris) - UTC+2 en heure d'été
            let paris_offset = 2;
            let paris_time = datetime_utc + chrono::Duration::hours(paris_offset);
            let paris_format =
                format!("🇫🇷 (UTC+{}) - {}", paris_offset, paris_time.format("%H:%M"));
            println!("Paris time: {}", paris_format);

            // États-Unis (New York) - UTC-4 en heure d'été
            let ny_offset = -4;
            let ny_time = datetime_utc + chrono::Duration::hours(ny_offset);
            let ny_format = format!("🇺🇸 (UTC-{}) - {}", ny_offset.abs(), ny_time.format("%H:%M"));

            // Chine (Beijing) - UTC+8 (pas de changement d'heure)
            let china_offset = 8;
            let china_time = datetime_utc + chrono::Duration::hours(china_offset);
            let china_format =
                format!("🇨🇳 (UTC+{}) - {}", china_offset, china_time.format("%H:%M"));

            // Concaténer les informations de temps pour tous les fuseaux horaires
            println!(
                "Paris: {}, New York: {}, China: {}",
                paris_format, ny_format, china_format
            );
            &format!("- {}\n- {}\n- {}", paris_format, ny_format, china_format)
        };

        result.push_str(&format!(
            "#### I'm currently working on {} {}: <br/>\nLast update at:\n{}\n",
            last.get_project(),
            icon_html,
            time,
        ));

        //result.push_str("```\n");
        result
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn get_range(&self) -> &Range {
        &Range::Today
    }
}
