pub enum OpenMensa {
    TUDresden,
}

impl OpenMensa {
    pub fn as_str(&self) -> &'static str {
        match self {
            OpenMensa::TUDresden => "https://api.studentenwerk-dresden.de/openmensa/v2",
        }
    }
}
