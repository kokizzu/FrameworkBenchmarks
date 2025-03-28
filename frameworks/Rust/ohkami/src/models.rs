use ohkami::serde;

#[derive(serde::Serialize)]
pub struct Message {
    pub message: &'static str,
}

#[cfg(feature = "db")]
pub use db::*;
#[cfg(feature = "db")]
mod db {
    use super::*;

    #[derive(sqlx::FromRow)]
    pub struct Fortune {
        pub id:      i32,
        pub message: String,
    }

    #[derive(sqlx::FromRow)]
    #[derive(serde::Serialize)]
    #[allow(non_snake_case)]
    pub struct World {
        pub id:           i32,
        #[serde(rename = "randomNumber")]
        pub randomnumber: i32,
    }

    #[derive(serde::Deserialize)]
    pub struct WorldsMeta<'req> {
        q: Option<&'req str>,
    }
    impl WorldsMeta<'_> {
        #[inline(always)]
        pub fn parse(self) -> usize {
            match self.q.unwrap_or("1").parse::<usize>().unwrap_or(1) {
                n @ 1..=500 => n,
                0           => 1,
                501..       => 500,
            }
        }
    }
}
