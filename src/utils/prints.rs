use colored::*;
use crate::utils::project::Project;
use crate::utils::unreal_engine::Engine;


pub struct Prints{}

impl Prints {
    pub fn print_logo() {
        println!(
            "
{} {} {}
{} {} {}
{} {} {}
{} {} {}
{} {} {}
{} {} {}",

            "    ██╗   ██╗███████╗".white().bold(),
            "    █████╗ ██╗    ██╗███████╗".truecolor(255, 153, 0).bold(),
            "   ████████╗ ██████╗  ██████╗ ██╗     ███████╗".white().bold(),


            "    ██║   ██║██╔════╝".white().bold(),
            "   ██╔══██╗██║    ██║██╔════╝".truecolor(255, 153, 0).bold(),
            "   ╚══██╔══╝██╔═══██╗██╔═══██╗██║     ██╔════╝".white().bold(),


            "    ██║   ██║█████╗".white().bold(),
            "     ███████║██║ █╗ ██║███████╗".truecolor(255, 153, 0).bold(),
            "      ██║   ██║   ██║██║   ██║██║     ███████╗".white().bold(),


            "    ██║   ██║██╔══╝".white().bold(),
            "     ██╔══██║██║███╗██║╚════██║".truecolor(255, 153, 0).bold(),
            "      ██║   ██║   ██║██║   ██║██║     ╚════██║".white().bold(),


            "    ╚██████╔╝███████╗".white().bold(),
            "   ██║  ██║╚███╔███╔╝███████║".truecolor(255, 153, 0).bold(),
            "      ██║   ╚██████╔╝╚██████╔╝███████╗███████║".white().bold(),


            "     ╚═════╝ ╚══════╝".white().bold(),
            "   ╚═╝  ╚═╝ ╚══╝╚══╝ ╚══════╝".truecolor(255, 153, 0).bold(),
            "      ╚═╝    ╚═════╝  ╚═════╝ ╚══════╝╚══════╝".white().bold()
        );

        let engine_versions = Engine::get_engine_versions().unwrap();
        let projects = Engine::get_engine_projects().unwrap();
        Self::print_footer(&engine_versions, &projects);
    }

    pub fn print_footer(engine_versions: &[String], projects: &[Project]) {
        // Верхняя граница
        println!("{}", "━".repeat(60).truecolor(100, 100, 100));

        // Заголовок
        println!("{}\n",
                 " Unreal Engine 5 AWS S3 Toolkit "
                     .bold()
                     .white()
                     .on_truecolor(30, 30, 30)
        );

        println!("{}  {}",
                 " v1.0 ".bold().white().on_blue(),
                 " 2025 UE5 AWS Tools ".bold().white().on_truecolor(255, 153, 0)
        );

        println!("\n{}", "⚙️ Available Engine Versions on this PC:".bright_cyan().bold());
        for version in engine_versions {
            println!("   {} {}", "➤".green(), version.cyan());
        }

        if !projects.is_empty() {
            println!("\n{}", "📂 Recent Projects:".bright_cyan().bold());

            let max_name_len = projects.iter()
                .map(|p| p.name.len())
                .max()
                .unwrap_or(20)
                .min(30);

            for project in projects {
                let displayed_name = if project.name.len() > max_name_len {
                    format!("{}...", &project.name[..max_name_len - 3])
                } else {
                    project.name.clone()
                };

                println!(
                    "   {} {:<width$}  {} {}",
                    "➤".green(),
                    displayed_name.cyan(),
                    "🕒".yellow(),
                    project.last_open_time.truecolor(180, 180, 180),
                    width = max_name_len
                );
            }
        }

        println!("\n{} {}",
                 "💡 Tip:".yellow().bold(),
                 "Run `ue-aws --help` to see all commands".white()
        );
        println!("{}", "━".repeat(60).truecolor(100, 100, 100));
    }

    pub fn print_help() {
        println!("\x1b[1;36mAvailable commands:\x1b[0m");
        println!();
        println!("  \x1b[1;32minit\x1b[0m      - Initialize a new project. Creates config, checks AWS storage, and prepares your branch.");
        println!("              \x1b[3mUsage: init\x1b[0m");
        println!();
        println!("  \x1b[1;32mpull\x1b[0m      - Pull the latest changes for the currently selected project.");
        println!("              \x1b[3mUsage: pull\x1b[0m");
        println!("              \x1b[33mNote: Requires an active project (use 'set <name>' first)\x1b[0m");
        println!();
        println!("  \x1b[1;32mpush\x1b[0m      - Push your changes to the remote repository for the current project.");
        println!("              \x1b[3mUsage: push\x1b[0m");
        println!("              \x1b[33mNote: Requires an active project (use 'set <name>' first)\x1b[0m");
        println!();
        println!("  \x1b[1;32mset\x1b[0m       - Select a project to work with.");
        println!("              \x1b[3mUsage: set <project_name>\x1b[0m");
        println!("              \x1b[3mExample: set my_awesome_project\x1b[0m");
        println!();
        println!("  \x1b[1;32munset\x1b[0m     - Deselect the current project (return to global mode).");
        println!("              \x1b[3mUsage: unset\x1b[0m");
        println!("              \x1b[33mNote: Only works if a project is currently selected\x1b[0m");
        println!();
        println!("  \x1b[1;32mexit/quit\x1b[0m - Exit the application.");
        println!("              \x1b[3mUsage: exit | quit\x1b[0m");
        println!();
        println!("  \x1b[1;32mhelp\x1b[0m      - Display this help message.");
        println!("              \x1b[3mUsage: help\x1b[0m");
    }
}