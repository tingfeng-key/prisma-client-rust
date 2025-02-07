use crate::db::{new_client, PrismaClient};
use prisma_client_rust::queries::QueryError;

pub type TestResult = Result<(), QueryError>;

pub async fn client() -> PrismaClient {
    let client = new_client().await.unwrap();

    client
        ._batch((
            client.file_path().delete_many(vec![]),
            client.category().delete_many(vec![]),
            client.post().delete_many(vec![]),
            client.profile().delete_many(vec![]),
            client.user().delete_many(vec![]),
            client.types().delete_many(vec![]),
        ))
        .await
        .unwrap();

    client
}

pub async fn cleanup(client: PrismaClient) -> TestResult {
    client
        ._batch((
            client.file_path().delete_many(vec![]),
            client.category().delete_many(vec![]),
            client.post().delete_many(vec![]),
            client.profile().delete_many(vec![]),
            client.user().delete_many(vec![]),
            client.types().delete_many(vec![]),
        ))
        .await
        .unwrap();

    Ok(())
}
