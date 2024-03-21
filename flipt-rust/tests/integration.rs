use flipt::api::FliptClient;
use flipt::evaluation::models::evaluation_response::Response;
use flipt::evaluation::models::{
    BatchEvaluationRequest, ErrorEvaluationReason, EvaluationReason, EvaluationRequest,
    EvaluationResponseType,
};
use flipt::{ClientTokenAuthentication, Config};
use std::{collections::HashMap, env};
use url::Url;

#[tokio::test]
async fn tests() {
    let url = env::var("FLIPT_URL").unwrap_or("http://localhost:8080".into());
    let token = env::var("FLIPT_AUTH_TOKEN").unwrap_or("".into());

    let flipt_client = FliptClient::new(Config::new(
        Url::parse(&url).unwrap(),
        ClientTokenAuthentication::new(token),
        60,
    ))
    .unwrap();

    let mut context: HashMap<String, String> = HashMap::new();
    context.insert("fizz".into(), "buzz".into());

    let variant_request = EvaluationRequest {
        namespace_key: "default".into(),
        flag_key: "flag1".into(),
        entity_id: "entity".into(),
        context: context.clone(),
        reference: None,
        request_id: None,
    };
    let boolean_request = EvaluationRequest {
        namespace_key: "default".into(),
        flag_key: "flag_boolean".into(),
        entity_id: "entity".into(),
        context: context.clone(),
        reference: None,
        request_id: None,
    };

    let variant = flipt_client
        .evaluation
        .variant(&variant_request)
        .await
        .unwrap();

    assert!(variant.r#match);
    assert_eq!(variant.variant_key, "variant1");
    assert_eq!(
        EvaluationReason::try_from(variant.reason).expect("valid"),
        EvaluationReason::Match
    );
    assert_eq!(variant.segment_keys.get(0).unwrap(), "segment1");

    let boolean = flipt_client
        .evaluation
        .boolean(&boolean_request)
        .await
        .unwrap();
    assert!(boolean.enabled);
    assert_eq!(boolean.flag_key, "flag_boolean");
    assert_eq!(
        EvaluationReason::try_from(boolean.reason).expect("valid"),
        EvaluationReason::Match
    );

    let mut requests: Vec<EvaluationRequest> = Vec::new();
    requests.push(variant_request);
    requests.push(boolean_request);
    requests.push(EvaluationRequest {
        namespace_key: "default".into(),
        flag_key: "notfound".into(),
        entity_id: "entity".into(),
        context: context.clone(),
        reference: None,
        request_id: None,
    });

    let batch_request = BatchEvaluationRequest {
        requests,
        reference: None,
        request_id: None,
    };
    let batch = flipt_client.evaluation.batch(&batch_request).await.unwrap();

    // Variant
    let first_response = batch.responses.get(0).unwrap();
    assert_eq!(
        EvaluationResponseType::try_from(first_response.r#type).expect("valid"),
        EvaluationResponseType::Variant.into()
    );

    let variant = match first_response.response.clone().unwrap() {
        Response::VariantResponse(r) => r,
        _ => todo!(),
    };
    assert!(variant.r#match);
    assert_eq!(variant.variant_key, "variant1");
    assert_eq!(
        EvaluationReason::try_from(variant.reason).expect("valid"),
        EvaluationReason::Match.into()
    );
    assert_eq!(variant.segment_keys.get(0).unwrap(), "segment1");

    // Boolean
    let second_response = batch.responses.get(1).unwrap();
    assert_eq!(
        EvaluationResponseType::try_from(second_response.r#type).expect("valid"),
        EvaluationResponseType::Boolean
    );
    let boolean = match second_response.response.clone().unwrap() {
        Response::BooleanResponse(r) => r,
        _ => todo!(),
    };
    assert!(boolean.enabled);
    assert_eq!(boolean.flag_key, "flag_boolean");
    assert_eq!(
        EvaluationReason::try_from(boolean.reason).expect("valid"),
        EvaluationReason::Match
    );

    // Error
    let third_response = batch.responses.get(2).unwrap();
    assert_eq!(
        EvaluationResponseType::try_from(third_response.r#type).expect("valid"),
        EvaluationResponseType::Error
    );
    let error = match third_response.response.clone().unwrap() {
        Response::ErrorResponse(r) => r,
        _ => todo!(),
    };

    assert_eq!(error.flag_key, "notfound");
    assert_eq!(error.namespace_key, "default");
    assert_eq!(
        ErrorEvaluationReason::try_from(error.reason).expect("valid"),
        ErrorEvaluationReason::NotFound
    );
}
