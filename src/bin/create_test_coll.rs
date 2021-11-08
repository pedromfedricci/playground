use libplayground::mongodb::{create_test_collection, TEST_COLL_PARAMS};

#[tokio::main]
async fn main() -> mongodb::error::Result<()> {
    create_test_collection(TEST_COLL_PARAMS).await?;
    Ok(())
}
