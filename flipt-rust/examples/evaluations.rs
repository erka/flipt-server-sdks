// cargo run --example evaluations

use std::collections::HashMap;

use flipt::api::FliptClient;
use flipt::evaluation::models::{
    BatchEvaluationRequest, EvaluationNamespaceSnapshotRequest, EvaluationRequest,
};

#[tokio::main]
#[cfg_attr(not(feature = "flipt_integration"), ignore)]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = FliptClient::default();

    let mut context: HashMap<String, String> = HashMap::new();

    context.insert("fizz".into(), "buzz".into());

    let variant_result = client
        .evaluation
        .variant(&EvaluationRequest {
            request_id: None,
            namespace_key: "default".into(),
            flag_key: "flag1".into(),
            entity_id: "entity".into(),
            context: context.clone(),
            reference: None,
        })
        .await
        .unwrap();

    println!("{:?}", variant_result);

    let boolean_result = client
        .evaluation
        .boolean(&EvaluationRequest {
            request_id: None,
            namespace_key: "default".into(),
            flag_key: "flag_boolean".into(),
            entity_id: "entity".into(),
            context: context.clone(),
            reference: None,
        })
        .await
        .unwrap();

    println!("{:?}", boolean_result);

    let requests: Vec<EvaluationRequest> = vec![
        EvaluationRequest {
            request_id: None,
            namespace_key: "default".into(),
            flag_key: "flag1".into(),
            entity_id: "entity".into(),
            context: context.clone(),
            reference: None,
        },
        EvaluationRequest {
            request_id: None,
            namespace_key: "default".into(),
            flag_key: "flag_boolean".into(),
            entity_id: "entity".into(),
            context: context.clone(),
            reference: None,
        },
    ];

    let batch_result = client
        .evaluation
        .batch(&BatchEvaluationRequest {
            request_id: None,
            requests,
            reference: None,
        })
        .await
        .unwrap();

    println!("{:?}", batch_result);

    let flags = client
        .evaluation
        .list_flags(&EvaluationNamespaceSnapshotRequest {
            key: "default".into(),
            reference: None,
        })
        .await
        .unwrap();
    println!("{:?}", flags);

    Ok(())
}
