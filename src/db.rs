use mongodb::{options::ClientOptions, Client};
use std::env;
use tokio::sync::OnceCell;

static DB: OnceCell<Client> = OnceCell::const_new();

async fn init_db() -> Client {
    let db_usr = env::var("MONGODB_USR").unwrap_or_default();
    let db_pwd = env::var("MONGODB_PWD").unwrap_or_default();
    let db_clstr = env::var("MONGODB_CLSTR").unwrap_or_default();
    let db_name = env::var("MONGODB_DB_NAME").unwrap_or_default();

    let conn = if db_usr.is_empty() || db_pwd.is_empty() {
        "mongodb://localhost:27017".to_string()
    } else {
        format!(
            "mongodb+srv://{}:{}@{}/?retryWrites=true&w=majority",
            db_usr, db_pwd, db_clstr
        )
    };

    let mut client_options = ClientOptions::parse(&conn)
        .await
        .expect("Client Options must be parsed.");

    if !db_name.is_empty() {
        client_options.app_name = Some(db_name);
    }

    Client::with_options(client_options).expect("Client must be instantiated.")
}

pub async fn get_client() -> &'static Client {
    DB.get_or_init(init_db).await
}
