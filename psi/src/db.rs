use std::env;

use native_tls::TlsConnector;
use postgres::Client;
use postgres_native_tls::MakeTlsConnector;

fn client() -> Client {
    Client::connect(
        &env::var("POSTGRES_CONNECTION").unwrap(),
        MakeTlsConnector::new(
            TlsConnector::builder()
                .danger_accept_invalid_certs(true)
                .build()
                .unwrap(),
        ),
    )
    .unwrap()
}

pub fn get_usernames() -> Vec<String> {
    let mut psql_client = client();
    psql_client
        .query(&env::var("POSTGRES_QUERY").unwrap(), &[])
        .unwrap()
        .iter()
        .map(|x| x.get(0))
        .collect()
}

pub fn update_intersection(data: Vec<String>) {
    let mut psql_client = client();
    let intersection_table = env::var("INTERSECTION_TABLE").unwrap();
    let intersection_column = env::var("INTERSECTION_COLUMN").unwrap();
    psql_client // haha sql injection goes brrrr
        .execute(
            &format!("DELETE FROM {} WHERE true", &intersection_table),
            &[],
        )
        .unwrap();
    for value in data {
        psql_client
            .execute(
                &format!(
                    "INSERT INTO {} ({}) VALUES ($1)",
                    &intersection_table, &intersection_column
                ),
                &[&value],
            )
            .unwrap();
    }
}
