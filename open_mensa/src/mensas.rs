pub enum OpenMensaEndpoint {
    TUDresden,
}

impl OpenMensaEndpoint {
    pub fn as_str(&self) -> &'static str {
        match self {
            OpenMensaEndpoint::TUDresden => "https://api.studentenwerk-dresden.de/openmensa/v2",
        }
    }
}
