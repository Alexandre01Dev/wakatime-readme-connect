use chrono::format;

use crate::config::Config;
use crate::wakatime_api::{Range, Statistic, StatisticData, WrappedStatistic, get_from_range};
use std::collections::HashMap;

pub trait MdModule {
    fn render(&self, stat: &StatisticData) -> String;
    fn name(&self) -> &str;
    fn get_range(&self) -> &Range;
}
pub struct TopEditorsModule {
    name: String,
    range: Range,
}

impl TopEditorsModule {
    pub fn new(range: Range) -> Self {
        TopEditorsModule {
            name: "Editors".to_string(),
            range,
        }
    }
}

impl MdModule for TopEditorsModule {
    fn render(&self, statistic: &StatisticData) -> String {
        let mut result = format!("## {}\n\n", self.name());
        result.push_str("```text\n");

        let mut sorted_editors = statistic.get_editors().clone();
        sorted_editors.sort_by(|a, b| {
            b.get_total_seconds()
                .partial_cmp(&a.get_total_seconds())
                .unwrap()
        });

        for editor in sorted_editors {
            result.push_str(&format!(
                "{}: {:.2}h ({}%)\n",
                editor.get_name(),
                editor.get_total_seconds() as f64 / 3600.0,
                editor.get_percent_as_string()
            ));
        }

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

pub struct MdDocument {
    title: String,
    modules: Vec<Box<dyn MdModule>>,
}

impl MdDocument {
    pub fn new(title: &str) -> Self {
        MdDocument {
            title: title.to_string(),
            modules: Vec::new(),
        }
    }

    pub fn add_module(&mut self, module: Box<dyn MdModule>) {
        self.modules.push(module);
    }

    pub fn render(&self, config: &Config) -> String {
        let mut content = format!("# {}\n\n", self.title);
        let mut hash_map_range = HashMap::new();

        for module in &self.modules {
            let statistic = if let Some(stat) = hash_map_range.get(module.get_range()) {
                stat
            } else {
                let range = module.get_range().clone(); // Clone the range to pass ownership
                let stat = get_from_range(range, config).unwrap();
                hash_map_range.insert(module.get_range().clone(), stat);
                hash_map_range.get(module.get_range()).unwrap()
            };
            content.push_str(&format!("{}\n", &module.render(statistic.get_data()))); // Pass the required argument
        }

        content
    }
}
