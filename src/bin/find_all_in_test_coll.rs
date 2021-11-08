use libplayground::mongodb::{find_all_docs_from_coll_params, TEST_COLL_PARAMS};

#[tokio::main]
async fn main() -> mongodb::error::Result<()> {
    find_all_docs_from_coll_params(TEST_COLL_PARAMS).await
}
