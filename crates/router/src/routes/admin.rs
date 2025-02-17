use actix_web::{web, HttpRequest, HttpResponse};
use router_env::{instrument, tracing, Flow};

use super::app::AppState;
use crate::{
    core::admin::*,
    services::{api, authentication as auth},
    types::api::admin,
};

/// Merchant Account - Create
///
/// Create a new account for a merchant and the merchant could be a seller or retailer or client who likes to receive and send payments.
#[utoipa::path(
    post,
    path = "/account",
    request_body= CreateMerchantAccount,
    responses(
        (status = 200, description = "Merchant Account Created", body = MerchantAccountResponse),
        (status = 400, description = "Invalid data")
    )
)]
#[instrument(skip_all, fields(flow = ?Flow::MerchantsAccountCreate))]
// #[post("")]
pub async fn merchant_account_create(
    state: web::Data<AppState>,
    req: HttpRequest,
    json_payload: web::Json<admin::CreateMerchantAccount>,
) -> HttpResponse {
    api::server_wrap(
        &state,
        &req,
        json_payload.into_inner(),
        |state, _, req| create_merchant_account(&*state.store, req),
        *auth::jwt_auth_or(&auth::AdminApiAuth, req.headers()),
    )
    .await
}

#[instrument(skip_all, fields(flow = ?Flow::MerchantsAccountRetrieve))]
// #[get("/{id}")]
pub async fn retrieve_merchant_account(
    state: web::Data<AppState>,
    req: HttpRequest,
    mid: web::Path<String>,
) -> HttpResponse {
    let payload = web::Json(admin::MerchantId {
        merchant_id: mid.into_inner(),
    })
    .into_inner();
    api::server_wrap(
        &state,
        &req,
        payload,
        |state, _, req| get_merchant_account(&*state.store, req),
        *auth::jwt_auth_or(&auth::AdminApiAuth, req.headers()),
    )
    .await
}

#[instrument(skip_all, fields(flow = ?Flow::MerchantsAccountUpdate))]
// #[post["/{id}"]]
pub async fn update_merchant_account(
    state: web::Data<AppState>,
    req: HttpRequest,
    mid: web::Path<String>,
    json_payload: web::Json<admin::CreateMerchantAccount>,
) -> HttpResponse {
    let merchant_id = mid.into_inner();
    api::server_wrap(
        &state,
        &req,
        json_payload.into_inner(),
        |state, _, req| merchant_account_update(&*state.store, &merchant_id, req),
        *auth::jwt_auth_or(&auth::AdminApiAuth, req.headers()),
    )
    .await
}

#[instrument(skip_all, fields(flow = ?Flow::MerchantsAccountDelete))]
// #[delete("/{id}")]
pub async fn delete_merchant_account(
    state: web::Data<AppState>,
    req: HttpRequest,
    mid: web::Path<String>,
) -> HttpResponse {
    let payload = web::Json(admin::MerchantId {
        merchant_id: mid.into_inner(),
    })
    .into_inner();
    api::server_wrap(
        &state,
        &req,
        payload,
        |state, _, req| merchant_account_delete(&*state.store, req.merchant_id),
        *auth::jwt_auth_or(&auth::AdminApiAuth, req.headers()),
    )
    .await
}

/// PaymentsConnectors - Create
///
/// Create a new Payment Connector for the merchant account. The connector could be a payment processor / facilitator / acquirer or specialized services like Fraud / Accounting etc."
#[utoipa::path(
    post,
    path = "/account/{account_id}/connectors",
    request_body = PaymentConnectorCreate,
    responses(
        (status = 200, description = "Payment Connector Created", body = PaymentConnectorCreate),
        (status = 400, description = "Missing Mandatory fields")
    )
)]
#[instrument(skip_all, fields(flow = ?Flow::PaymentConnectorsCreate))]
pub async fn payment_connector_create(
    state: web::Data<AppState>,
    req: HttpRequest,
    path: web::Path<String>,
    json_payload: web::Json<admin::PaymentConnectorCreate>,
) -> HttpResponse {
    let merchant_id = path.into_inner();
    api::server_wrap(
        &state,
        &req,
        json_payload.into_inner(),
        |state, _, req| create_payment_connector(&*state.store, req, &merchant_id),
        *auth::jwt_auth_or(&auth::AdminApiAuth, req.headers()),
    )
    .await
}

#[instrument(skip_all, fields(flow = ?Flow::PaymentConnectorsRetrieve))]
// #[get("/{merchant_id}/connectors/{merchant_connector_id}")]
pub async fn payment_connector_retrieve(
    state: web::Data<AppState>,
    req: HttpRequest,
    path: web::Path<(String, i32)>,
) -> HttpResponse {
    let (merchant_id, merchant_connector_id) = path.into_inner();
    let payload = web::Json(admin::MerchantConnectorId {
        merchant_id,
        merchant_connector_id,
    })
    .into_inner();
    api::server_wrap(
        &state,
        &req,
        payload,
        |state, _, req| {
            retrieve_payment_connector(&*state.store, req.merchant_id, req.merchant_connector_id)
        },
        *auth::jwt_auth_or(&auth::AdminApiAuth, req.headers()),
    )
    .await
}

#[instrument(skip_all, fields(flow = ?Flow::PaymentConnectorsList))]

pub async fn payment_connector_list(
    state: web::Data<AppState>,
    req: HttpRequest,
    path: web::Path<String>,
) -> HttpResponse {
    let merchant_id = path.into_inner();
    api::server_wrap(
        &state,
        &req,
        merchant_id,
        |state, _, merchant_id| list_payment_connectors(&*state.store, merchant_id),
        *auth::jwt_auth_or(&auth::AdminApiAuth, req.headers()),
    )
    .await
}

#[instrument(skip_all, fields(flow = ?Flow::PaymentConnectorsUpdate))]
// #[post("/{merchant_id}/connectors/{merchant_connector_id}")]
pub async fn payment_connector_update(
    state: web::Data<AppState>,
    req: HttpRequest,
    path: web::Path<(String, i32)>,
    json_payload: web::Json<admin::PaymentConnectorCreate>,
) -> HttpResponse {
    let (merchant_id, merchant_connector_id) = path.into_inner();
    api::server_wrap(
        &state,
        &req,
        json_payload.into_inner(),
        |state, _, req| {
            update_payment_connector(&*state.store, &merchant_id, merchant_connector_id, req)
        },
        *auth::jwt_auth_or(&auth::AdminApiAuth, req.headers()),
    )
    .await
}

#[instrument(skip_all, fields(flow = ?Flow::PaymentConnectorsDelete))]
// #[delete("/{merchant_id}/connectors/{merchant_connector_id}")]
pub async fn payment_connector_delete(
    state: web::Data<AppState>,
    req: HttpRequest,
    path: web::Path<(String, i32)>,
) -> HttpResponse {
    let (merchant_id, merchant_connector_id) = path.into_inner();
    let payload = web::Json(admin::MerchantConnectorId {
        merchant_id,
        merchant_connector_id,
    })
    .into_inner();
    api::server_wrap(
        &state,
        &req,
        payload,
        |state, _, req| {
            delete_payment_connector(&*state.store, req.merchant_id, req.merchant_connector_id)
        },
        *auth::jwt_auth_or(&auth::AdminApiAuth, req.headers()),
    )
    .await
}
