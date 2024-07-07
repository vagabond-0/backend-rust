use surrealdb::engine::remote::ws::{Client,Ws};
use surrealdb::opt::auth::Root;
use surrealdb::{Error,Surreal};

#[derive(Clone)]
pub struct Database{
    pub Client:Surreal<Client>,
    pub name_space : String,
    pub db_name: String,
}

impl Database{
    pub async fn init()->Result<Self,Error>{
        let client = Surreal::new::<Ws>("127.0.0.1.8000").await?;
        client.signin(Root{
            username:"root",
            password:"root"
        })
        .await?;
        client.use_ns("surreal").use_db("blog").await.unwrap();
        Ok(Database{
            Client:client,
            name_space:String::from("surreal"),
            db_name:String::from("blog")
        })
    }
}