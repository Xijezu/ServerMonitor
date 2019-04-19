use mysql as my;
use mysql::params;

pub struct Database {
    pub connection_string: String,
}

impl Database {
    pub fn do_query(&self, name: &String, user_count: i32) {
        let pool = my::Pool::new(&self.connection_string).unwrap();
        for mut stmt in pool
            .prepare(r"INSERT INTO Log.ServerMonitor VALUES (:name, now(), :user_count)")
            .into_iter()
        {
            stmt.execute(params! {
            "name" => &name,
            "user_count" => &user_count
            })
            .unwrap();
        }
    }
}
