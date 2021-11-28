fn reversed(s: &str) -> String {
    s.chars().rev().collect()
}

#[derive(PartialEq, Eq)]
enum PaliStep {
    Mirror,
    MergedMirror,
}
impl PaliStep {
    fn toggle(&mut self) {
        *self = match self {
            Self::Mirror => Self::MergedMirror,
            Self::MergedMirror => Self::Mirror,
        }
    }
}

struct Pali {
    current_root: u64,
    current_step: PaliStep,
}
impl Pali {
    fn new() -> Self {
        Self {
            current_root: 1,
            current_step: PaliStep::Mirror,
        }
    }
}
impl Iterator for Pali {
    type Item = u64;
    fn next(&mut self) -> Option<Self::Item> {
        use PaliStep::*;
        self.current_step.toggle();
        let s = self.current_root.to_string();
        if self.current_step == MergedMirror && s.len() > 1 {
            Some(
                format!("{}{}", s, reversed(s.as_str())[1..].to_owned())
                    .parse()
                    .expect("Failed to parse palidrome string as u64"),
            )
        } else {
            self.current_root += 1;
            Some(
                format!("{}{}", s, reversed(s.as_str()))
                    .parse()
                    .expect("Failed to parse palidrome string as u64"),
            )
        }
    }
}

fn unix_timestamp_to_datetime_string(ts: u64) -> Option<String> {
    use chrono::prelude::DateTime;
    use chrono::Utc;
    use std::time::{Duration, UNIX_EPOCH};

    let d = UNIX_EPOCH.checked_add(Duration::from_secs(ts));
    if d == None {
        return None;
    }
    let d = d.unwrap();
    let datetime = DateTime::<Utc>::from(d);
    Some(datetime.format("%Y-%m-%d %H:%M:%S").to_string())
}

fn main() {
    let p = Pali::new();
    for x in p {
        if let Some(s) = unix_timestamp_to_datetime_string(x) {
            if s.starts_with("2021") {
                println!("{}\t{}", x, s)
            }
        }
    }
}
