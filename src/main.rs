use chrono::{Local, NaiveDate, NaiveDateTime, ParseError};
use clap::{arg, Command};
use crossterm::{
    cursor,
    event::{poll, read, Event, KeyCode, KeyEvent},
    execute,
    style::{Color, Print, ResetColor, SetForegroundColor, Stylize},
    terminal,
};
use serde::{Deserialize, Serialize};
use std::{
    cmp,
    collections::HashMap,
    fs::{File, OpenOptions},
    io::{self, stdout, Read, Write},
    thread,
    time::Duration,
};

fn print_info(info: &str, t: bool, c: Color) {
    let color = c;
    if t {
        execute!(
            stdout(),
            cursor::MoveUp(1),
            terminal::Clear(terminal::ClearType::CurrentLine),
            cursor::MoveToColumn(0),
            SetForegroundColor(color),
            Print(info),
            ResetColor,
            cursor::MoveDown(1),
            cursor::MoveToColumn(0),
        )
        .expect("Failed to execute commands");
    } else {
        execute!(
            stdout(),
            cursor::MoveUp(1),
            terminal::Clear(terminal::ClearType::CurrentLine),
            cursor::MoveToColumn(0),
            SetForegroundColor(color),
            Print(info),
            ResetColor,
            cursor::MoveDown(1),
            terminal::Clear(terminal::ClearType::CurrentLine),
            cursor::MoveToColumn(0),
        )
        .expect("Failed to execute commands");
    }
}

fn get_input(prompt: &str) -> String {
    terminal::disable_raw_mode().expect("Failed to enable raw mode");
    execute!(
        stdout(),
        cursor::MoveUp(1),
        terminal::Clear(terminal::ClearType::CurrentLine),
        cursor::MoveToColumn(0),
        Print(prompt),
    )
    .expect("Failed to execute commands");
    io::stdout().flush().expect("Failed to flush stdout");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    terminal::enable_raw_mode().expect("Failed to enable raw mode");
    input.trim().to_string()
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Project {
    pub name: String,
    pub start_date: String,
    pub hours_per_day: HashMap<String, String>,
    pub total_time: String,
}

impl Project {
    pub fn new(name: &str, start_date: &str) -> Self {
        Project {
            name: name.to_string(),
            start_date: start_date.to_string(),
            hours_per_day: HashMap::new(),
            total_time: String::from("00:00:00"),
        }
    }

    pub fn display(&mut self, last: bool, solo: bool, namelen: usize) {
        if solo {
            println!("╭{}╮", "─".repeat(cmp::max(namelen, 33)));
            println!(
                "╰─{}{}│\n  ├─{}: {}{}│\n  ╰─{}: {}{}│",
                self.name.clone().negative(),
                " ".repeat(35 - cmp::min(self.name.len() + 3, 34)),
                "Start Date".underlined(),
                self.start_date.clone().italic(),
                " ".repeat(cmp::max(cmp::max(namelen, 32) - 32, 1)),
                "Total Time".underlined(),
                self.total_time.clone().italic(),
                " ".repeat(cmp::max(cmp::max(namelen, 23) - 23, 10))
            );

            let binding = self.order_hours_per_day().unwrap().clone();
            for (i, proj) in binding.iter().enumerate() {
                if i == binding.len() - 1 {
                    println!(
                        "    ╰─{}: {}{}│",
                        proj.0,
                        proj.1.clone().italic(),
                        " ".repeat(cmp::max(cmp::max(namelen, 23) - 23, 10))
                    );
                } else {
                    println!(
                        "    ├─{}: {}{}│",
                        proj.0,
                        proj.1.clone().italic(),
                        " ".repeat(cmp::max(cmp::max(namelen, 23) - 23, 10))
                    );
                }
            }

            println!("─{}╯", "─".repeat(cmp::max(namelen, 33)));
        } else if last {
            println!(
                "╰─{}{}│\n  ├─{}: {}{}│\n  ╰─{}: {}{}│",
                self.name.clone().negative(),
                " ".repeat(namelen + 2 - cmp::min(self.name.len() + 3, namelen + 1)),
                "Start Date".underlined(),
                self.start_date.clone().italic(),
                " ".repeat(cmp::max(cmp::max(namelen, 32) - 32, 10)),
                "Total Time".underlined(),
                self.total_time.clone().italic(),
                " ".repeat(cmp::max(cmp::max(namelen, 23) - 23, 10))
            );

            let binding = self.order_hours_per_day().unwrap().clone();
            for (i, proj) in binding.iter().enumerate() {
                if i == binding.len() - 1 {
                    println!(
                        "    ╰─{}: {}{}│",
                        proj.0,
                        proj.1.clone().italic(),
                        " ".repeat(cmp::max(cmp::max(namelen, 23) - 23, 10))
                    );
                } else {
                    println!(
                        "    ├─{}: {}{}│",
                        proj.0,
                        proj.1.clone().italic(),
                        " ".repeat(cmp::max(cmp::max(namelen, 23) - 23, 10))
                    );
                }
            }

            println!("─{}╯", "─".repeat(cmp::max(namelen, 34)));
        } else {
            println!(
                "╰─{}{}│\n  ├─{}: {}{}│\n  ╰─{}: {}{}│",
                self.name.clone().negative(),
                " ".repeat(namelen + 2 - cmp::min(self.name.len() + 3, namelen + 1)),
                "Start Date".underlined(),
                self.start_date.clone().italic(),
                " ".repeat(cmp::max(cmp::max(namelen, 32) - 32, 10)),
                "Total Time".underlined(),
                self.total_time.clone().italic(),
                " ".repeat(cmp::max(cmp::max(namelen, 23) - 23, 10))
            );

            let binding = self.order_hours_per_day().unwrap().clone();
            for (i, proj) in binding.iter().enumerate() {
                if i == binding.len() - 1 {
                    println!(
                        "    ╰─{}: {}{}│",
                        proj.0,
                        proj.1.clone().italic(),
                        " ".repeat(cmp::max(cmp::max(namelen, 23) - 23, 10))
                    );
                } else {
                    println!(
                        "    ├─{}: {}{}│",
                        proj.0,
                        proj.1.clone().italic(),
                        " ".repeat(cmp::max(cmp::max(namelen, 23) - 23, 10))
                    );
                }
            }
        }
    }

    fn order_hours_per_day(&mut self) -> Result<Vec<(&String, &String)>, ParseError> {
        let mut sorted_hours: Vec<(_, _)> = self.hours_per_day.iter().collect::<Vec<(_, _)>>();

        sorted_hours.sort_by(|day1, day2| {
            NaiveDate::parse_from_str(day1.0, "%m/%d/%y")
                .unwrap()
                .cmp(&NaiveDate::parse_from_str(day2.0, "%m/%d/%y").unwrap())
        });

        // Create a new HashMap from the sorted Vec
        // let new_hours_per_day: HashMap<_, _> = sorted_hours
        //     .into_iter()
        //     .map(|day| (day.0.to_owned(), day.1.to_owned()))
        //     .collect::<HashMap<String, String>>();
        //self.hours_per_day = new_hours_per_day;

        Ok(sorted_hours)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Session {
    pub elapsed: u64,
    pub paused: bool,
}

impl Session {
    pub fn new() -> Self {
        Session {
            elapsed: 0,
            paused: false,
        }
    }

    pub fn format_elapsed(&self) -> String {
        format!(
            "{:02}:{:02}:{:02}",
            self.elapsed / 3600,
            (self.elapsed / 60) % 60,
            self.elapsed % 60
        )
    }
}

impl Default for Session {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TimeTracking {
    pub today: String,
    pub projects: HashMap<String, Project>,
    pub current_project: Option<Project>,
    pub current_session: Option<Session>,
}

impl TimeTracking {
    pub fn new() -> Self {
        return TimeTracking {
            today: Local::now().format("%m/%d/%y").to_string(),
            projects: TimeTracking::load_from_file().unwrap_or_default(),
            current_project: None,
            current_session: None,
        };
    }

    pub fn sort_projects(&mut self) -> Vec<Project> {
        let mut sorted_projects: Vec<_> = self.projects.clone().into_values().collect();

        sorted_projects.sort_by_key(|project| {
            NaiveDateTime::parse_from_str(&project.start_date, "%m/%d/%y %H:%M:%S").unwrap()
        });

        // Create a new HashMap with sorted entries
        // let sorted_projects_map: HashMap<_, _> = sorted_projects
        //     .into_iter()
        //     .map(|project| (project.name.clone(), project))
        //     .collect();

        sorted_projects
    }

    pub fn start_project(&mut self, project_name: &str) {
        print_info(&format!("\u{2714} Starting project: {:?}. Press [a] to stop and quit, [s] to switch projects or [p] to pause.\n", project_name), false, Color::Green);
        let session = Session::new();
        self.current_session = Some(session);

        if let Some(project) = self.projects.get_mut(project_name) {
            if project.hours_per_day.get_mut(&self.today).is_none() {
                project
                    .hours_per_day
                    .insert(self.today.clone(), "00:00:00".to_string());
            }

            self.current_project = Some(project.clone());
        } else {
            let mut new_project = Project::new(
                project_name,
                &Local::now().format("%m/%d/%y %H:%M:%S").to_string(),
            );
            new_project
                .hours_per_day
                .insert(self.today.clone(), "00:00:00".to_string());
            self.projects
                .insert(project_name.to_owned(), new_project.clone());
            self.current_project = Some(new_project);
        }
    }

    pub fn pause(&mut self) {
        if let Some(ref mut session) = self.current_session {
            if !session.paused {
                session.paused = true;
                print_info("\u{1f6c8} Paused. Press [r] to resume.", true, Color::Blue);
            } else {
                print_info("\u{26a0} Project is already paused.", true, Color::Red);
            }
        }
    }

    pub fn resume(&mut self) {
        if let Some(ref mut session) = self.current_session {
            if session.paused {
                print_info(&format!("\u{1f6c8} Resumed project: {:?}. Press [a] to stop and quit, [s] to switch projects or [p] to pause.", self.current_project.clone().unwrap().name), false, Color::Blue);
                session.paused = false;
            } else {
                print_info("\u{26a0} Project is already running.", false, Color::Red);
            }
        }
    }

    pub fn switch_project(&mut self, project_name: &str) {
        self.stop_project();
        self.start_project(project_name);
    }

    pub fn stop_project(&mut self) {
        if let Some(ref mut session) = self.current_session {
            if !session.paused {
                session.paused = true;
            }

            let elapsed_time_str = session.format_elapsed();

            if let Some(ref mut project) = self
                .projects
                .get_mut(&self.current_project.clone().unwrap().name)
            {
                let mut carry_seconds = 0;
                let mut carry_minutes = 0;
                project.total_time = project
                    .total_time
                    .split(':')
                    .rev()
                    .zip(elapsed_time_str.split(':').rev())
                    .map(|(a, b)| {
                        let a_val = a.parse::<i64>().unwrap_or(0);
                        let b_val = b.parse::<i64>().unwrap_or(0);
                        let mut sum = a_val + b_val + carry_seconds;

                        carry_seconds = sum / 60;
                        sum %= 60;

                        sum += carry_minutes;
                        carry_minutes = sum / 60;
                        sum %= 60;

                        format!("{:02}", sum)
                    })
                    .collect::<Vec<_>>()
                    .into_iter()
                    .rev()
                    .collect::<Vec<_>>()
                    .join(":");

                if let Some(entry) = project.hours_per_day.get_mut(&self.today) {
                    let mut carry_seconds = 0;
                    let mut carry_minutes = 0;
                    let updated_time = entry
                        .split(':')
                        .rev()
                        .zip(elapsed_time_str.split(':').rev())
                        .map(|(a, b)| {
                            let a_val = a.parse::<i64>().unwrap_or(0);
                            let b_val = b.parse::<i64>().unwrap_or(0);
                            let mut sum = a_val + b_val + carry_seconds;

                            carry_seconds = sum / 60;
                            sum %= 60;

                            sum += carry_minutes;
                            carry_minutes = sum / 60;
                            sum %= 60;

                            format!("{:02}", sum)
                        })
                        .collect::<Vec<_>>()
                        .into_iter()
                        .rev()
                        .collect::<Vec<_>>()
                        .join(":");
                    project
                        .hours_per_day
                        .insert(self.today.to_owned(), updated_time);
                }
            }

            self.current_project = None;
            self.current_session = None;
            self.save_to_file().expect("unable to save to file");
        }
    }

    pub fn list_project_or_all(&mut self, project_name: Option<&str>) {
        match project_name {
            Some(name) => {
                let sorted_projects = self.sort_projects();
                let project = sorted_projects.iter().find(|proj| proj.name == name);
                match project {
                    Some(project) => {
                        println!(
                            "{}",
                            format!(
                                "\u{1f6c8} Displaying tracking information for project: {:?}",
                                name
                            )
                            .blue()
                        );
                        project.clone().display(false, true, project.name.len() + 2);
                    }
                    None => {
                        println!(
                            "{}",
                            format!("\u{26a0} Project {:?} not found!", name).red()
                        );
                    }
                }
            }
            None => {
                let sorted_projects = self.sort_projects();
                let namelen = sorted_projects.iter().fold(0, |l, proj| {
                    if proj.name.len() > l {
                        proj.name.len()
                    } else {
                        l
                    }
                });
                let projects = sorted_projects.iter().enumerate();
                for (i, project) in projects {
                    if i == sorted_projects.len() - 1 {
                        println!("╭{}┤", "─".repeat(cmp::max(namelen + 2, 33)));
                        project.clone().display(true, false, namelen + 2);
                    } else if i == 0 {
                        println!("╭{}╮", "─".repeat(cmp::max(namelen + 2, 33)));
                        project.clone().display(false, false, namelen + 2)
                    } else {
                        println!("╭{}┤", "─".repeat(cmp::max(namelen + 2, 33)));
                        project.clone().display(false, false, namelen + 2);
                    }
                }
            }
        }
    }

    pub fn get_day_info(&mut self, day: &String) {
        let hours = self
            .sort_projects()
            .into_iter()
            .filter(|proj| proj.hours_per_day.contains_key(day))
            .map(|proj| (proj.name, proj.hours_per_day.get(day).unwrap().clone()))
            .collect::<Vec<(String, String)>>();
        if hours.is_empty() {
            println!("{}", format!("\u{26a0} Day {:?} not found!", day).red());
            return;
        }
        println!(
            "{}",
            format!(
                "\u{1f6c8} Displaying tracking information for day {:?}",
                day
            )
            .blue()
        );
        let namelen = hours.iter().map(|x| x.0.clone().len()).max().unwrap_or(0) + 16;
        let day_total_time = hours
            .iter()
            .map(|a| a.1.split(':'))
            .collect::<Vec<std::str::Split<'_, char>>>()
            .into_iter()
            .map(|b| b.clone().collect::<Vec<&str>>())
            .collect::<Vec<Vec<&str>>>();
        let formatted_day_total_time = (0..day_total_time[0].len())
            .rev()
            .fold((0, Vec::new()), |(mut carry, mut acc), i| {
                let sum: i64 = day_total_time
                    .iter()
                    .map(|inner| inner[i].trim().parse::<i64>().unwrap_or(0))
                    .sum();
                let total = sum + carry;
                carry = total / 60;
                acc.insert(0, total % 60);
                (carry, acc)
            })
            .1
            .iter()
            .map(|&x| format!("{:02}", x))
            .collect::<Vec<_>>()
            .join(":");
        println!("╭{}╮", "─".repeat(cmp::max(namelen, 24)));
        println!(
            "╰─{}{}│\n  ╰─{}: {}{}│",
            day.clone().negative(),
            " ".repeat(cmp::max(cmp::max(namelen, 9) - 9, 15)),
            "Total Time".underlined(),
            formatted_day_total_time.clone().italic(),
            " ".repeat(cmp::max(cmp::max(namelen, 23) - 23, 1))
        );

        for (i, proj) in hours.iter().enumerate() {
            if i == hours.len() - 1 {
                println!(
                    "    ╰─{}: {}{}│",
                    proj.0,
                    proj.1.clone().italic(),
                    " ".repeat(cmp::max(
                        namelen - 15 - cmp::min(proj.0.len(), namelen - 16),
                        if namelen < 25 { 9 - proj.0.len() } else { 1 }
                    ))
                );
            } else {
                println!(
                    "    ├─{}: {}{}│",
                    proj.0,
                    proj.1.clone().italic(),
                    " ".repeat(cmp::max(
                        namelen - 15 - cmp::min(proj.0.len(), namelen - 16),
                        if namelen < 25 { 9 - proj.0.len() } else { 1 }
                    ))
                );
            }
        }

        println!("─{}╯", "─".repeat(cmp::max(namelen, 24)));
    }

    pub fn save_to_file(&self) -> io::Result<()> {
        let json_data = serde_json::to_string_pretty(&self.projects)
            .expect("Failed to serialize time tracking data to JSON");
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .open("time_tracker_data.json")?;
        file.write_all(json_data.as_bytes())?;
        Ok(())
    }

    pub fn load_from_file() -> io::Result<HashMap<String, Project>> {
        let mut file = File::open("time_tracker_data.json")?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        serde_json::from_str(&contents).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
    }
}

impl Default for TimeTracking {
    fn default() -> Self {
        Self::new()
    }
}

fn main() {
    terminal::enable_raw_mode().expect("Failed to enable raw mode");
    let mut tt = TimeTracking::new();
    let matches = Command::new("Rust Time Tracker")
        .author("Rafael Zaccaro")
        .version("1.0.0")
        .about("Time tracker for projects")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("start")
                .short_flag('s')
                .about("Starts tracking a project")
                .arg(arg!(<PROJECT_NAME>).help("Name of the project to start")),
        )
        .subcommand(
            Command::new("list")
                .short_flag('l')
                .about(
                    "Displays tracking info of a project, if provided. If not, lists all projects",
                )
                .arg(
                    arg!([PROJECT_NAME])
                        .required(false)
                        .help("Name of the project to retrieve tracking info from"),
                ),
        )
        .subcommand(
            Command::new("day")
                .short_flag('d')
                .about("Displays tracking info of a specific day (using the format \"mm/dd/yy\")")
                .arg(
                    arg!(<DAY>)
                        .help("Day to retrieve tracking info from (using the format \"mm/dd/yy\")"),
                ),
        )
        .get_matches(); //hint format to user

    match matches.subcommand() {
        Some(("start", sub_matches)) => {
            tt.start_project(sub_matches.get_one::<String>("PROJECT_NAME").unwrap());
            loop {
                if poll(Duration::from_millis(100)).expect("Failed to poll events") {
                    if let Ok(Event::Key(KeyEvent {
                        code,
                        state: _,
                        modifiers: _,
                        kind,
                    })) = read()
                    {
                        match code {
                            KeyCode::Char('a') => {
                                tt.stop_project();
                                //maybe print out something like "today's total time {}, project total time {}"
                                break;
                            }
                            KeyCode::Char('s') if kind == crossterm::event::KeyEventKind::Press => {
                                tt.switch_project(&get_input("\u{1f5cb} New project name: "));
                            }
                            KeyCode::Char('p') if kind == crossterm::event::KeyEventKind::Press => {
                                tt.pause();
                            }
                            KeyCode::Char('r') if kind == crossterm::event::KeyEventKind::Press => {
                                tt.resume();
                            }
                            _ => {}
                        }
                    }
                }
                if let Some(ref current_project) = tt.current_project {
                    if let Some(ref mut current_session) = tt.current_session {
                        if !current_session.paused {
                            current_session.elapsed += 1;
                            execute!(
                                stdout(),
                                terminal::Clear(terminal::ClearType::CurrentLine),
                                cursor::MoveToColumn(0),
                                Print(format!(
                                    "\u{23f1} Project: {} \u{2016} Elapsed time: {}",
                                    current_project.name,
                                    current_session.format_elapsed()
                                ))
                            )
                            .expect("f");
                        }
                    }
                }
                thread::sleep(Duration::from_secs(1));
            }
            terminal::disable_raw_mode().expect("Failed to disable raw mode");
        }
        Some(("list", sub_matches)) => match sub_matches.get_one::<String>("PROJECT_NAME") {
            Some(proj) => {
                tt.list_project_or_all(Some(proj));
            }
            None => {
                println!(
                    "{}",
                    "\u{1f6c8} Displaying tracking information for all projects".blue()
                );
                tt.list_project_or_all(None);
            }
        },
        Some(("day", sub_matches)) => {
            tt.get_day_info(sub_matches.get_one::<String>("DAY").unwrap());
        }
        _ => unreachable!("Exhausted list of subcommands and subcommand_required prevents `None`"),
    }
}
