use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
use surrealdb::{Error, Surreal};

use crate::models::user::User;
use crate::models::uuid::Uuid;

#[derive(Clone)]
pub struct Database {
    pub client: Surreal<Client>,
    pub name_space: String,
    pub db_name: String,
}

impl Database {
    pub async fn init() -> Result<Self, Error> {
        let client = Surreal::new::<Ws>("127.0.0.1:8000").await?;
        client
            .signin(Root {
                username: "root",
                password: "root",
            })
            .await?;

        client.use_ns("surreal").use_db("user").await.unwrap();

        Ok(Database {
            client,
            name_space: String::from("surreal"),
            db_name: String::from("user"),
        })
    }

    pub async fn get_all_users(&self) -> Option<Vec<User>> {
        let result = self.client.select("user").await;
        match result {
            Ok(all_users) => Some(all_users),
            Err(_) => None,
        }
    }

    pub async fn add_user(&self, user: User) -> Option<User> {
        let result = self
            .client
            .create(("user", user.uuid.clone()))
            .content(user)
            .await;
        match result {
            Ok(user) => user,
            Err(_) => None,
        }
    }
}
