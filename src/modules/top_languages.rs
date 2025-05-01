use chrono::format;

use crate::lang_icons;
use crate::md_modules::MdModule;
use crate::wakatime_api::{Range, StatisticData};

pub struct TopLanguagesModule {
    name: String,
    limit: usize,
    range: Range,
}

impl TopLanguagesModule {
    pub fn new(range: Range, limit: usize) -> Self {
        TopLanguagesModule {
            name: "Top Languages".to_string(),
            limit,
            range,
        }
    }
    pub fn new_with_name(name: String, range: Range, limit: usize) -> Self {
        TopLanguagesModule { name, limit, range }
    }
}

impl MdModule for TopLanguagesModule {
    fn render(&self, statistic: &StatisticData) -> String {
        let mut result = format!("## {} \n\n", self.name());
        //result.push_str("```text\n");
        let mut count = 0;
        statistic.get_languages().iter().for_each(|language| {
            if count == self.limit {
                return;
            }
            count += 1;
            if language.get_total_seconds() > 0 {
                let icon_html = format!(
                    "<img src=\"{}\" alt=\"{}\" width=\"20\" height=\"20\">",
                    lang_icons::search_icon(language.get_name()),
                    language.get_name()
                );
                result.push_str(&format!(
                    "- {} {} : {:.2}h ({})\n",
                    language.get_name(),
                    icon_html,
                    (language.get_total_seconds() as f64) / 3600.0,
                    language.get_percent_as_string()
                ));
            }
        });

        //result.push_str("```\n");
        result
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn get_range(&self) -> &Range {
        &self.range
    }
}
