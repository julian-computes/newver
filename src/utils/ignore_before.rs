use chrono::{Duration};
use crate::prelude::*;

pub fn ignore_before(value: &str) -> Result<Duration> {
    let chars = value.chars();
    let num = chars.clone().by_ref().take_while(|c| c.is_digit(10))
        .collect::<String>().parse::<i64>()?;
    let unit = chars.last().ok_or("failed to find unit in ignore_before")?;
    let result = match unit {
        'w' => Duration::weeks(num),
        'd' => Duration::days(num),
        'm' => Duration::days(num * 30),
        'y' => Duration::days(num * 365),
        _ => return Err(f!("invalid ignore_before {value}").into()),
    };

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_ignore_before() {
        let data = vec![
            ("1w", Duration::weeks(1)),
            ("2d", Duration::days(2)),
            ("3m", Duration::days(30 * 3)),
            ("4y", Duration::days(365 * 4)),
        ];

        for (input, expected) in data {
            let result = ignore_before(input).unwrap();
            assert_eq!(result, expected);
        }
    }
}