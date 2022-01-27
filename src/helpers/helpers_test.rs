use super::*;
use mongodb::bson::bson;


#[tokio::main]
#[test]
async fn sanitize_index_int() {
   let library = Library {
    name: "hello_name".to_string(),
    friendly_name: "hiiii".to_string(),
    data_types: vec![],
    fields: vec![],
    index: IndexField { name: "hello".to_string(), friendly_name: "Heyyy".to_string(), prim_type: IndexType::Int }
   };

   let bson = bson! ({
       "hello": 3
   });

   let res = sanitize_library_item(&library, &bson).await.unwrap();

   let exp = bson! ({
       "index": 3,
       "data": {},
       "fields": {},
   });
   assert_eq!(exp, res);

}

#[tokio::main]
#[test]
async fn sanitize_index_string() {
   let library = Library {
    name: "hello_name".to_string(),
    friendly_name: "hiiii".to_string(),
    data_types: vec![],
    fields: vec![],
    index: IndexField { name: "hello".to_string(), friendly_name: "Heyyy".to_string(), prim_type: IndexType::Str }
   };

   let bson = bson! ({
       "hello": "value1"
   });

   let res = sanitize_library_item(&library, &bson).await.unwrap();

   let exp = bson! ({
       "index": "value1",
       "data": {},
       "fields": {},
   });
   assert_eq!(exp, res);

}

#[tokio::main]
#[test]
#[should_panic]
async fn sanitize_index_bad_type() {
   let library = Library {
    name: "hello_name".to_string(),
    friendly_name: "hiiii".to_string(),
    data_types: vec![],
    fields: vec![],
    index: IndexField { name: "hello".to_string(), friendly_name: "Heyyy".to_string(), prim_type: IndexType::Int }
   };

   let bson = bson! ({
       "hello": 3.14
   });

   let _res = sanitize_library_item(&library, &bson).await.unwrap();

//    let exp = bson! ({
//        "index": "value1",
//        "data": [],
//        "fields": [],
//    });
//    assert_eq!(exp.to_string(), res.to_string());

}

#[tokio::main]
#[test]
#[should_panic]
async fn sanitize_index_mismatched_type() {
   let library = Library {
    name: "hello_name".to_string(),
    friendly_name: "hiiii".to_string(),
    data_types: vec![],
    fields: vec![],
    index: IndexField { name: "hello".to_string(), friendly_name: "Heyyy".to_string(), prim_type: IndexType::Str }
   };

   let bson= bson! ({
       "hello": 3
   });

   let _res = sanitize_library_item(&library, &bson).await.unwrap();


}


#[tokio::main]
#[test]
#[should_panic]
async fn sanitize_index_mismatched_type_2() {
   let library = Library {
    name: "hello_name".to_string(),
    friendly_name: "hiiii".to_string(),
    data_types: vec![],
    fields: vec![],
    index: IndexField { name: "hello".to_string(), friendly_name: "Heyyy".to_string(), prim_type: IndexType::Int }
   };

   let bson= bson! ({
       "hello": "Hey there!"
   });

   let _res = sanitize_library_item(&library, &bson).await.unwrap();

}

#[tokio::main]
#[test]
async fn sanitize_unused_fields() {
   let library = Library {
    name: "hello_name".to_string(),
    friendly_name: "hiiii".to_string(),
    data_types: vec![],
    fields: vec![],
    index: IndexField { name: "hello".to_string(), friendly_name: "Heyyy".to_string(), prim_type: IndexType::Str }
   };

   let bson = bson! ({
       "hello": "value1",
       "unused": 1,
       "fields": "should be gone",
       "yes": false
   });

   let res = sanitize_library_item(&library, &bson).await.unwrap();

   let exp = bson! ({
       "index": "value1",
       "data": {},
       "fields": {},
   });
   assert_eq!(exp, res);

}