/*
 * Copyright © 2024 the original author or authors.
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

// ----------------------------------------------------------------

use std::str::FromStr;

use bson::{Bson, Decimal128, Document};

// ----------------------------------------------------------------

pub fn start() {
    init().expect("Failed");
    println!("Hello, MongoDB bson!");
}

fn init() -> Result<(), Box<dyn std::error::Error>> {
    let database = sled::open("mongo.db")?;

    let mut post_doc = Document::new();
    post_doc.insert("title", Bson::String("My BSON Doc".to_string()));
    post_doc.insert("content", Bson::String("Hello BSON!".to_string()));
    post_doc.insert(
        "price",
        Bson::Decimal128(Decimal128::from_str("88.48").unwrap()),
    );

    let serialized_doc = bson::to_vec(&post_doc)?;

    database.insert("post", serialized_doc)?;
    let retrieved = database.get("post")?.unwrap();
    let deserialized_doc: Document = bson::from_slice(&retrieved)?;

    println!("Title: {:?}", deserialized_doc.get_str("title").unwrap());
    println!(
        "Content: {:?}",
        deserialized_doc.get_str("content").unwrap()
    );
    println!(
        "Price: {:?}",
        deserialized_doc
            .get_decimal128("price")
            .unwrap()
            .to_string()
    );

    Ok(())
}
