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
            name: "Top_Languages".to_string(),
            limit,
            range,
        }
    }
}

impl MdModule for TopLanguagesModule {
    fn render(&self, statistic: &StatisticData) -> String {
        let mut result = format!("## {} (Top {})\n\n", self.name(), self.limit);
        result.push_str("```text\n");
        statistic.get_languages().iter().for_each(|language| {
            if language.get_total_seconds() > 0 {
                result.push_str(&format!(
                    "{}: {:.2}h ({}%)\n",
                    language.get_name(),
                    (language.get_total_seconds() as f64) / 3600.0,
                    language.get_percent_as_string()
                ));
            }
        });

        result.push_str("```\n");
        result
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn get_range(&self) -> &Range {
        &self.range
    }
}
