use std::fmt::{self, Debug, Display, Formatter};

pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        if (0..24).contains(&hours) && (0..60).contains(&minutes) {
            return Self { hours, minutes };
        }

        if (0..24).contains(&hours) && (-60..0).contains(&minutes) {
            if hours == 0 {
                return Self {
                    hours: 23,
                    minutes: 60 + minutes,
                };
            }
            return Self {
                hours: hours - 1,
                minutes: 60 + minutes,
            };
        }

        if (-24..0).contains(&hours) && (0..60).contains(&minutes) {
            return Self {
                hours: 24 + hours,
                minutes,
            };
        }

        if (-24..0).contains(&hours) && (-60..0).contains(&minutes) {
            return Self {
                hours: 24 + hours - 1,
                minutes: 60 + minutes,
            };
        }

        return Self::new((hours + minutes / 60) % 24, minutes % 60);
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        if minutes == 0 {
            return self.clone();
        }
        Self::new(self.hours, self.minutes + minutes)
    }
}

impl Clone for Clock {
    fn clone(&self) -> Self {
        Self {
            hours: self.hours,
            minutes: self.minutes,
        }
    }
}

impl Debug for Clock {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("Clock")
            .field("hours", &self.hours)
            .field("minutes", &self.minutes)
            .finish()
    }
}

impl Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}

impl PartialEq for Clock {
    fn eq(&self, other: &Self) -> bool {
        self.hours == other.hours && self.minutes == other.minutes
    }
}
