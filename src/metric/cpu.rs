use crate::response::Info;
use crate::metric::Delta;

#[derive(Default)]
pub struct Cpu {
    pub used_cpu_user: f64,
    pub used_cpu_sys: f64,
    pub last_delta_cpu_sys: f64,
    pub last_delta_cpu_user: f64,
}

impl Delta for Cpu {
    fn calc_delta(&mut self, other: &Self, tick_rate: f64) {
        self.last_delta_cpu_user = if other.used_cpu_user == 0.0 {
            0.00001
        } else {
            let delta = (self.used_cpu_user - other.used_cpu_user) / tick_rate;
            if delta <= 0.00001 {
                0.00001
            } else {
                delta
            }
        } * 100.0;

        self.last_delta_cpu_sys = if other.used_cpu_sys == 0.0 {
            0.00001
        } else {
            let delta = (self.used_cpu_sys - other.used_cpu_sys) / tick_rate;
            if delta <= 0.00001 {
                0.00001
            } else {
                delta
            }
        } * 100.0;
    }
}

impl From<&Info> for Cpu {
    fn from(i: &Info) -> Self {
        let used_cpu_user = if let Some(used_cpu_user) = i.0.get("used_cpu_user") {
            used_cpu_user.parse::<f64>().unwrap_or(0.0)
        } else {
            0.0
        };

        let used_cpu_sys = if let Some(used_cpu_sys) = i.0.get("used_cpu_sys") {
            used_cpu_sys.parse::<f64>().unwrap_or(0.0)
        } else {
            0.0
        };

        Cpu {
            used_cpu_user,
            used_cpu_sys,
            last_delta_cpu_sys: 0.0,
            last_delta_cpu_user: 0.0,
        }
    }
}