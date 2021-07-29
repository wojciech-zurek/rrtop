pub trait Formatter {
    fn num_format(&self) -> String;
}

impl Formatter for f64 {
    fn num_format(&self) -> String {
        match *self {
            x if x >= 10000.0 && x < 1000000.0 => format!("{:.2}K", x / 1000.0),
            x if x >= 1000000.0 => format!("{:.2}M", x / 1000000.0),
            _ => { format!("{:.1}", self) }
        }
    }
}

impl Formatter for u64{
    fn num_format(&self) -> String {
        match *self {
            10000..=999999 => format!("{:.2}K", *self as f64 / 1000.0),
            1000000..=u64::MAX => format!("{:.2}M", *self as f64 / 1000000.0),
            _ => { format!("{}", self) }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::widget::formatter::Formatter;

    #[test]
    fn test_num_format_f64() {
        assert_eq!(0.5.num_format(), "0.5");
        assert_eq!(1.0.num_format(), "1.0");
        assert_eq!(10.0.num_format(), "10.0");
        assert_eq!(1000.0.num_format(), "1000.0");
        assert_eq!(1001.0.num_format(), "1001.0");
        assert_eq!(1101.0.num_format(), "1101.0");
        assert_eq!(1950.0.num_format(), "1950.0");
        assert_eq!(1956.0.num_format(), "1956.0");
        assert_eq!(2999.0.num_format(), "2999.0");
        assert_eq!(9999.0.num_format(), "9999.0");
        assert_eq!(10000.0.num_format(), "10.00K");
        assert_eq!(900000.0.num_format(), "900.00K");
        assert_eq!(999999.0.num_format(), "1000.00K");
        assert_eq!(1000000.0.num_format(), "1.00M");
        assert_eq!(1000001.0.num_format(), "1.00M");
    }

    #[test]
    fn test_num_format_u64() {
        assert_eq!(0.num_format(), "0");
        assert_eq!(1.num_format(), "1");
        assert_eq!(10.num_format(), "10");
        assert_eq!(1000.num_format(), "1000");
        assert_eq!(1956.num_format(), "1956");
        assert_eq!(2999.num_format(), "2999");
        assert_eq!(9999.num_format(), "9999");
        assert_eq!(10000.num_format(), "10.00K");
        assert_eq!(900000.num_format(), "900.00K");
        assert_eq!(999999.num_format(), "1000.00K");
        assert_eq!(1000000.num_format(), "1.00M");
        assert_eq!(1000001.num_format(), "1.00M");
    }
}
