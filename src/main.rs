use crate::wakatime_api::{get_from, get_from_range, Range};

mod config;
mod wakatime_api;
mod wakatime_error;
mod github_api;

fn main() {
    let config = config::load_config();

    let response = get_from_range(Range::This7Days,&config);

    match response {
        Ok(statistic) => {
            let stat = statistic.get_data();
            println!("Username: {}", stat.get_username());
            println!("Total seconds: {}", stat.get_total_seconds());
            println!("Daily average: {}", stat.get_daily_average());
            println!("Days including holidays: {}", stat.get_days_including_holidays());
            println!("Range: {}", stat.get_range());
            println!("Human readable range: {}", stat.get_human_readable_range());
            println!("Human readable total: {}", stat.get_human_readable_total());
            println!("Human readable daily average: {}", stat.get_human_readable_daily_average());
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
        }
        Err(e) => eprintln!("Error: {}", e),
    }

}
