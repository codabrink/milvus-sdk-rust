// Licensed to the LF AI & Data foundation under one
// or more contributor license agreements. See the NOTICE file
// distributed with this work for additional information
// regarding copyright ownership. The ASF licenses this file
// to you under the Apache License, Version 2.0 (the
// "License"); you may not use this file except in compliance
// with the License. You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use anyhow::Result;
use milvus::{prelude::*, proto::schema::FieldData};

#[tokio::test]
async fn create_collection() -> Result<()> {
    let client = Client::new(None).await;

    assert!(client.is_ok());
    let mut client = client?;

    let collection_name = "test_schema";

    if client.has_collection(collection_name).await? {
        client.drop_collection(collection_name).await?;
    }

    assert!(!client.has_collection(collection_name).await?);

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

    client.create_collection(schema, 1).await?;

    client
        .insert(
            collection_name,
            None,
            vec![
                FieldData::long_data("book_id", vec![0; 12]),
                FieldData::int_data("Age", vec![0; 12]),
                FieldData::float_vector("calories", vec![12.; 12 * 24], 24),
            ],
            12,
        )
        .await?;

    client
        .delete(collection_name, None, "book_id in [0,1]")
        .await?;

    Ok(())
}
