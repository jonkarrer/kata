fn format_duration(seconds: u64) -> String {
    if seconds == 0 {
        return String::from("now");
    }

    let units: [(&str, u64); 5] = [
        ("year", 31536000),
        ("day", 86400),
        ("hour", 3600),
        ("minute", 60),
        ("second", 1),
    ];

    let mut secs = seconds;
    let mut times = Vec::new();

    for unit in units {
        let chunk = secs / unit.1;

        if chunk >= 1 {
            secs = secs - (chunk * unit.1);

            if chunk > 1 {
                times.push(format!("{} {}s", chunk, unit.0));
            } else {
                times.push(format!("{} {}", chunk, unit.0));
            }
        }
    }

    match times.len() {
        1 => times[0].to_string(),
        2 => format!("{} and {}", times[0], times[1]),
        3 => format!("{}, {} and {}", times[0], times[1], times[2]),
        4 => format!("{}, {}, {} and {}", times[0], times[1], times[2], times[3]),
        _ => format!(
            "{}, {}, {}, {} and {}",
            times[0], times[1], times[2], times[3], times[4]
        ),
    }
}

fn main() {
    let s = 6231;
    let f = format_duration(s);
    println!("{f}");
}
