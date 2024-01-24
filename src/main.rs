fn format_duration(seconds: u64) -> String {
    if seconds == 0 {
        return String::from("now");
    }

    let mut secs = seconds;

    // Find seconds in a:
    // * year = 31,536,000
    // * day = 86400
    // * hour = 3600
    // * minute = 60

    let year_in_sec = 31536000;
    let day_in_sec = 86400;
    let hour_in_sec = 3600;
    let minute_in_sec = 60;

    let mut timeMap = Vec::new();

    // Divide and bucket
    let years = seconds / year_in_sec;
    if years >= 1 {
        secs = secs - (years * year_in_sec);
        timeMap.push(if years > 1 {
            format!("{} years", years)
        } else {
            format!("{} year", years)
        });

        dbg!(years, secs);
    }

    let days = secs / day_in_sec;
    if days >= 1 {
        secs = secs - (days * day_in_sec);
        timeMap.push(if days > 1 {
            format!("{} days", days)
        } else {
            format!("{} day", days)
        });
        dbg!(days, secs);
    }

    let hours = secs / hour_in_sec;
    if hours >= 1 {
        secs = secs - (hours * hour_in_sec);
        timeMap.push(if hours > 1 {
            format!("{} hours", hours)
        } else {
            format!("{} hour", hours)
        });
        dbg!(hours, secs);
    }

    let minutes = secs / minute_in_sec;
    if minutes >= 1 {
        secs = secs - (minutes * minute_in_sec);
        timeMap.push(if minutes > 1 {
            format!("{} minutes", minutes)
        } else {
            format!("{} minute", minutes)
        });
        dbg!(minutes, secs);
    }

    if secs >= 1 {
        timeMap.push(if secs > 1 {
            format!("{} seconds", secs)
        } else {
            format!("{} second", secs)
        });
    }

    let mut finalString = String::new();
    let mapLen = timeMap.len();
    for (i, item) in timeMap.iter().enumerate() {
        if mapLen == 1 {
            finalString.push_str(&item);
        }

        if i + 1 == mapLen {
            finalString.push_str(format!("and {}", item).as_str());
        } else if i == mapLen {
            finalString.push_str(format!("{} ", &item).as_str());
        } else {
            finalString.push_str(format!("{}, ", &item).as_str());
        }
    }

    dbg!(finalString);
    String::new()
}

fn main() {
    let s = 3662;
    let f = format_duration(s);
    println!("{f}");
}
