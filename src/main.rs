use md_modules::{MdModule, TopEditorsModule};
use std::{thread, time::Duration};
use wakatime_api::get_heartbeats_today;

use crate::wakatime_api::{Range, get_from_range};

mod config;
mod github_api;
mod lang_icons;
mod md_modules;
mod modules;
mod wakatime_api;
mod wakatime_error;

fn main() {
    let config = config::load_config();

    let response = get_from_range(Range::This7Days, &config);

    let test_heartbeat_resp = get_heartbeats_today(&config);
    println!("Test heartbeat response: {:?}", test_heartbeat_resp);

    match response {
        Ok(statistic) => {
            let stat = statistic.get_data();
            println!("Username: {}", stat.get_username());
            println!("Total seconds: {}", stat.get_total_seconds());
            println!("Daily average: {}", stat.get_daily_average());
            println!(
                "Days including holidays: {}",
                stat.get_days_including_holidays()
            );
            println!("Range: {}", stat.get_range());
            println!("Human readable range: {}", stat.get_human_readable_range());
            println!("Human readable total: {}", stat.get_human_readable_total());
            println!(
                "Human readable daily average: {}",
                stat.get_human_readable_daily_average()
            );
            for editor in stat.get_editors() {
                println!("Editor: {}", editor.get_name());
                println!("Total seconds: {}", editor.get_total_seconds());
                println!("Percentage: {}", editor.get_percent());
            }

            for project in stat.get_projects() {
                println!("Project: {}", project.get_name());
                println!("Total seconds: {}", project.get_total_seconds());
                println!("Percentage: {}", project.get_percent());
            }

            for language in stat.get_languages() {
                println!("Language: {}", language.get_name());
                println!("Total seconds: {}", language.get_total_seconds());
                println!("Percentage: {}", language.get_percent());
            }

            for machine in stat.get_machines() {
                println!("Machine: {}", machine.get_name());
                println!("Total seconds: {}", machine.get_total_seconds());
                println!("Percentage: {}", machine.get_percent());
            }

            for operating_system in stat.get_operating_systems() {
                println!("Operating system: {}", operating_system.get_name());
                println!("Total seconds: {}", operating_system.get_total_seconds());
                println!("Percentage: {}", operating_system.get_percent());
            }

            for category in stat.get_categories() {
                println!("Category: {}", category.get_name());
                println!("Total seconds: {}", category.get_total_seconds());
                println!("Percentage: {}", category.get_percent());
            }

            loop {
                // Creation of the modules.
                let module_editors: TopEditorsModule =
                    md_modules::TopEditorsModule::new(Range::AllTime);

                let module_languages_week: modules::top_languages::TopLanguagesModule =
                    modules::top_languages::TopLanguagesModule::new_with_name(
                        "My TOP **5** Languages This Week".to_string(),
                        Range::This7Days,
                        5,
                    );

                let module_languages_all_time: modules::top_languages::TopLanguagesModule =
                    modules::top_languages::TopLanguagesModule::new_with_name(
                        "My TOP **5** Language ALL Time".to_string(),
                        Range::AllTime,
                        5,
                    );
                let module_heartbeats_status: modules::heartbeats_status::HeartBeatsStatus =
                    modules::heartbeats_status::HeartBeatsStatus::new_with_name(
                        "What I'm doing now ?".to_string(),
                    );

                // Create modules document
                let mut modules = md_modules::MdDocument::new("Wakatime Statistics", 60);
                let time = modules.get_refresh_time();
                // Add modules to the document
                modules.add_module(Box::new(module_heartbeats_status));
                modules.add_module(Box::new(module_editors));
                modules.add_module(Box::new(module_languages_week));
                modules.add_module(Box::new(module_languages_all_time));
                let result = github_api::get_and_update(&config, modules);
                match result {
                    Ok(_) => println!("GitHub API call succeeded"),
                    Err(e) => eprintln!("GitHub API call failed: {}", e),
                }
                thread::sleep(Duration::from_secs(time));
                println!("Reloading modules...");
            }
        }

        Err(e) => eprintln!("Error: {}", e),
    }
}
