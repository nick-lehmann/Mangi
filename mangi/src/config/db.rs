use diesel::{r2d2::ConnectionManager, PgConnection};
use r2d2::Pool;

pub type PgPool = Pool<ConnectionManager<PgConnection>>;

pub struct DbConfig<'a> {
    pub host: &'a str,
    pub port: u16,
    pub user: &'a str,
    pub password: &'a str,
    pub database: &'a str,
}

impl<'a> Into<String> for DbConfig<'a> {
    fn into(self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.user, self.password, self.host, self.port, self.database
        )
    }
}

pub fn get_pool() -> Result<PgPool, r2d2::Error> {
    let config = DbConfig {
        host: "db",
        port: 5432,
        user: "mangi",
        password: "mangi",
        database: "mangi",
    };

    let manager = ConnectionManager::<PgConnection>::new(config);
    r2d2::Pool::builder().max_size(5).build(manager)
}
