use clap::{App, Arg, ArgMatches};
use std::collections::BTreeMap;
use std::env;
use std::fs;
use std::os::unix::fs::MetadataExt;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;

fn main() {
    let (opts, targets) = Opts::parse();
    let mut info = BTreeMap::new();
    match targets {
        Some(targets) => {
            let mut files = DirInfo::new();
            for target in targets.iter() {
                match fs::metadata(target) {
                    Ok(metadata) => {
                        if metadata.is_dir() {
                            let dir = DirInfo::new_dir(&target, metadata);
                            info.insert(target.clone(), dir);
                        } else if metadata.is_file() {
                            files.add_item(target, metadata)
                        }
                    }
                    Err(_) => eprintln!("{}: No such file or directory", target),
                }
            }
            if files.items.len() > 0 {
                info.insert(String::new(), files);
            }
        }
        None => {
            let cur_dir = env::current_dir().unwrap();
            info.insert(
                String::new(),
                DirInfo::new_dir(
                    &cur_dir.to_str().unwrap().to_string(),
                    fs::metadata(cur_dir).unwrap(),
                ),
            );
        }
    }

    let print_path = info.iter().len() > 1;
    for (path, info) in info.iter() {
        if print_path {
            println!("{}", path);
        }
        info.print(&opts);
    }
}

#[derive(Debug, Clone)]
struct Opts {
    all: bool,
    long: bool,
}

impl Opts {
    fn parse() -> (Self, Option<Vec<String>>) {
        let app = App::new("ls")
            .arg(Arg::with_name("targets").min_values(0))
            .arg(Arg::with_name("all").short("a").long("all"))
            .arg(Arg::with_name("long").short("l"));
        // TODO : any more options

        let m: ArgMatches = app.get_matches();

        let targets = match m.values_of("targets") {
            Some(files) => Some(files.map(|f| f.to_string()).collect()),
            None => None,
        };

        (
            Self {
                all: m.is_present("all"),
                long: m.is_present("long"),
            },
            targets,
        )
    }
}

struct DirInfo {
    metadata: Option<fs::Metadata>,
    parent: Option<fs::Metadata>,
    items: BTreeMap<String, fs::Metadata>,
}

impl DirInfo {
    fn new() -> Self {
        DirInfo {
            metadata: None,
            parent: None,
            items: BTreeMap::new(),
        }
    }

    fn new_dir(dir: &String, metadata: fs::Metadata) -> Self {
        let path = Path::new(dir);

        let parent = if let Some(parent) = path.parent() {
            let metadata = fs::metadata(parent).unwrap();
            Some(metadata)
        } else {
            None
        };

        let mut items = BTreeMap::new();
        for entry in fs::read_dir(path).unwrap() {
            let entry = entry.unwrap();
            let metadata = fs::metadata(entry.path()).unwrap();
            items.insert(
                entry
                    .path()
                    .file_name()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_string(),
                metadata,
            );
        }
        DirInfo {
            metadata: Some(metadata),
            parent,
            items,
        }
    }

    fn add_item<S: Into<String>>(&mut self, name: S, metadata: fs::Metadata) {
        self.items.insert(name.into(), metadata);
    }

    fn print(&self, opts: &Opts) {
        let mut items = Vec::new();
        if opts.all {
            if let Some(ref metadata) = self.metadata {
                items.push((".", metadata));
            }
            if let Some(ref metadata) = self.parent {
                items.push(("..", metadata));
            }
        }

        for (path, info) in self.items.iter() {
            if !opts.all && path.starts_with('.') {
                continue;
            }
            items.push((path, info));
        }
        self.print_items(&items, opts);
    }

    fn print_items(&self, items: &Vec<(&str, &fs::Metadata)>, opts: &Opts) {
        // TODO: format, color
        if !opts.long {
            println!(
                "{}",
                items
                    .iter()
                    .map(|(file, _)| *file)
                    .collect::<Vec<_>>()
                    .join("  ")
            );
            return;
        }

        for (file, metdata) in items.iter() {
            let metadata: &fs::Metadata = *metdata;
            let (user, group) = owner(metadata);
            let (month, day, year_or_time) = date_time(metadata);
            println!(
                "{} {} {} {: >8} {: >2} {: >2} {: >5} {}",
                mode(metadata),
                user,
                group,
                metadata.len(),
                month,
                day,
                year_or_time,
                file
            );
        }
    }
}

fn mode(meta: &fs::Metadata) -> String {
    let mode = meta.permissions().mode();
    let bit = |bit, ch| if mode & bit == bit { ch } else { '-' };
    format!(
        "{}{}{}{}{}{}{}{}{}{}",
        if meta.is_dir() { 'd' } else { '-' },
        bit(0x0100, 'r'),
        bit(0x0080, 'w'),
        bit(0x0040, 'x'),
        bit(0x0020, 'r'),
        bit(0x0010, 'w'),
        bit(0x0008, 'x'),
        bit(0x0004, 'r'),
        bit(0x0002, 'w'),
        bit(0x0001, 'x'),
    )
}

fn owner(meta: &fs::Metadata) -> (String, String) {
    (
        match users::get_user_by_uid(meta.uid()) {
            Some(user) => user.name().to_string_lossy().to_string(),
            None => meta.uid().to_string(),
        },
        match users::get_group_by_gid(meta.gid()) {
            Some(group) => group.name().to_string_lossy().to_string(),
            None => meta.gid().to_string(),
        },
    )
}

fn date_time(meta: &fs::Metadata) -> (u32, u32, String) {
    use chrono::{DateTime, Datelike, Local, NaiveDateTime, Timelike, Utc};
    let t = meta.modified().unwrap();
    let t = t
        .duration_since(std::time::UNIX_EPOCH)
        .expect("back to the future");
    let dt = DateTime::<Utc>::from_utc(
        NaiveDateTime::from_timestamp(t.as_secs() as i64, t.subsec_nanos()),
        Utc,
    );
    let dt: DateTime<Local> = DateTime::from(dt);
    let now = Local::now();
    (
        dt.month(),
        dt.day(),
        if now.year() == dt.year() {
            format!("{}:{}", dt.hour(), dt.minute())
        } else {
            format!("{}", dt.year())
        },
    )
}
