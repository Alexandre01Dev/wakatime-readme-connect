use crate::wakatime_api::{Range, WrappedStatistic};
pub mod top_languages;
pub trait MdModule {
    fn render(&self) -> String;
    fn name(&self) -> &str;
}
pub struct TopEditorsModule {
    name: String,
    editors: Vec<WrappedStatistic>,
}

impl TopEditorsModule {
    pub fn new(editors: Vec<WrappedStatistic>) -> Self {
        TopEditorsModule {
            name: "Editors".to_string(),
            editors,
        }
    }
}

impl MdModule for TopEditorsModule {
    fn render(&self) -> String {
        let mut result = format!("## {}\n\n", self.name());
        result.push_str("```text\n");

        let mut sorted_editors = self.editors.clone();
        sorted_editors.sort_by(|a, b| b.get_total_seconds().partial_cmp(&a.get_total_seconds()).unwrap());

        for editor in sorted_editors {
            result.push_str(&format!("{}: {:.2}h ({}%)\n",
                                     editor.get_name(),
                                     editor.get_total_seconds() / 3600.0,
                                     editor.get_percent_as_string()
            ));
        }

        result.push_str("```\n");
        result
    }

    fn name(&self) -> &str {
        &self.name
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

    pub fn render(&self) -> String {
        let mut content = format!("# {}\n\n", self.title);

        for module in &self.modules {
            content.push_str(&module.render());
            content.push_str("\n");
        }

        content
    }
}