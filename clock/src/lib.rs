use std::fmt;
const MINUTES_IN_DAY: i32 = 1440;

#[derive(Debug, PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let mut t_minutes = ((60 * hours) + minutes) % MINUTES_IN_DAY;

        if t_minutes < 0 {
            t_minutes += MINUTES_IN_DAY;
        }

        let c_hours = t_minutes / 60;
        let c_mins = t_minutes % 60;

        Clock {
            hours: c_hours,
            minutes: c_mins,
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock::new(self.hours, self.minutes + minutes)
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}
