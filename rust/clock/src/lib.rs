#[derive(Debug, PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let mut h = hours;
        let mut m = minutes;

        let mut add_hour = m / 60;

        m %= 60;

        if m < 0 {
            m += 60;
            add_hour -= 1;
        }

        h += add_hour;
        h %= 24;

        h += if h < 0 { 24 } else { 0 };

        Clock {
            hours: h,
            minutes: m,
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock::new(self.hours, self.minutes + minutes)
    }
}

impl std::fmt::Display for Clock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}
