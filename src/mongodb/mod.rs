use futures::TryStreamExt;
use mongodb::{
    bson::{doc, Document},
    Client, Collection,
};

pub struct CollectionParams<'a> {
    pub url: &'a str,
    pub db: &'a str,
    pub coll: &'a str,
}

pub static TEST_COLL_PARAMS: &'static CollectionParams = &CollectionParams {
    url: "mongodb://localhost:27017",
    db: "foo",
    coll: "bar",
};

pub static NUMBER_OF_DOCS: usize = 10_000;

pub async fn get_collection(
    coll_params: &CollectionParams<'_>,
) -> mongodb::error::Result<Collection<Document>> {
    Ok(Client::with_uri_str(coll_params.url)
        .await?
        .database(coll_params.db)
        .collection(coll_params.coll))
}

/// Insert 10k docs into the collection.
///
/// Use this function to insert some test data so that the `find` below has some
/// work to do. To reduce noise, don't run this while generating flamegraphs.
pub async fn create_test_collection(
    coll_params: &CollectionParams<'_>,
) -> mongodb::error::Result<Collection<Document>> {
    let collection = get_collection(coll_params).await?;
    collection.drop(None).await?;

    let doc = doc! {
        "hello": "world",
        "anotherKey": "anotherValue",
        "number": 1234
    };
    let docs = itertools::repeat_n(&doc, NUMBER_OF_DOCS);

    collection.insert_many(docs, None).await.unwrap();
    Ok(collection)
}

pub async fn find_all_docs_in_collection(
    coll: &Collection<Document>,
) -> mongodb::error::Result<()> {
    coll.find(doc! {}, None)
        .await?
        .try_collect::<Vec<_>>()
        .await?;

    Ok(())
}

pub async fn find_all_docs_from_coll_params(
    coll_params: &CollectionParams<'_>,
) -> mongodb::error::Result<()> {
    find_all_docs_in_collection(&get_collection(coll_params).await?).await
}
