use crate::db::EMAIL_MAX;
use crate::db::USERNAME_MAX;

#[derive(Debug)]
pub struct InsertStatement {
    pub id: u32,
    pub username: String,
    pub email: String,
}

impl InsertStatement {
    pub fn new(input: &[String]) -> Result<InsertStatement, String> {
        if input.len() < 4 {
            return Err("Not enough arguments".to_string());
        }
        if input[2].len() > USERNAME_MAX {
            return Err(format!(
                "Username too long, must be less than {USERNAME_MAX}"
            ));
        }
        if input[3].len() > EMAIL_MAX {
            return Err(format!("email too long, must be less than {EMAIL_MAX}"));
        }
        let id = input[1].parse::<u32>().map_err(|_| "Failed to parse id")?;

        Ok(InsertStatement {
            id,
            username: input[2].clone(),
            email: input[3].clone(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_too_few_args() {
        let input: Vec<String> = vec!["insert".into(), "1".into()];
        assert!(InsertStatement::new(&input).is_err());
    }

    #[test]
    fn rejects_username_too_long() {
        let long = "a".repeat(USERNAME_MAX + 1);
        let input = vec!["insert".into(), "1".into(), long, "e@e.com".into()];
        assert!(InsertStatement::new(&input).is_err());
    }

    #[test]
    fn rejects_bad_id() {
        let input = vec![
            "insert".into(),
            "notanumber".into(),
            "user".into(),
            "e@e.com".into(),
        ];
        assert!(InsertStatement::new(&input).is_err());
    }
}
