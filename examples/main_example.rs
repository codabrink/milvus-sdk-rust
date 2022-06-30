use anyhow::Result;
use milvus::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
  let mut client = Client::new(None).await?;

  let collection_name = "test_schema";

  if client.has_collection(collection_name).await? {
    client.drop_collection(collection_name).await?;
  }

  let schema = CollectionDef {
    name: collection_name.to_owned(),
    description: "description".to_owned(),
    auto_id: false,
    fields: vec![
      FieldDef::primary_key_field("book_id", false),
      FieldDef::float_field("Age"),
      FieldDef::float_vector_field("calories", 24),
    ],
  };

  client.create_collection(schema, 2).await?;

  client
    .insert(
      collection_name,
      None,
      vec![
        ("book_id", vec![0i64; 12]).into(),
        ("Age", vec![0i32; 12]).into(),
        ("calories", vec![12f32; 12 * 24], 24i64).into(),
      ],
      12,
    )
    .await?;

  // client
  // .delete(collection_name, None, "book_id in [0,1]")
  // .await?;

  Ok(())
}
