use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    fn new_from_minutes(mut minutes: i32) -> Self {
        if minutes < 0 {
            minutes = 1440 + (minutes % 1440);
        }
        Self {
            hours: (minutes / 60) % 24,
            minutes: (minutes % 60),
        }
    }

    pub fn new(hours: i32, minutes: i32) -> Self {
        Self::new_from_minutes((60 * hours) + minutes)
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Self::new_from_minutes((60 * self.hours) + self.minutes + minutes)
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}
