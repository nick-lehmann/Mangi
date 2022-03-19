#[derive(Debug, Clone, PartialEq)]
pub enum UserType {
    Student,
    Employee,
    // Guest,
}

impl TryFrom<String> for UserType {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "student" => Ok(UserType::Student),
            "employee" => Ok(UserType::Employee),
            _ => Err(format!("Unable to convert {} into a user type", value)),
        }
    }
}

impl Into<String> for UserType {
    fn into(self) -> String {
        match self {
            UserType::Student => "student",
            UserType::Employee => "employee",
        }
        .to_string()
    }
}
