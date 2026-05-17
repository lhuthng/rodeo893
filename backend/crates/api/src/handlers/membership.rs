use std::sync::Arc;
use axum::{extract::State, Json};
use application::{
    dto::membership::ActivateMembershipInput,
    use_cases::{activate_membership::ActivateMembership, get_membership_status::GetMembershipStatus},
};
use crate::{error::ApiResult, extractors::ValidatedJson, middleware::auth_user::RequireUserSession, state::AppState};

/// Get membership status
#[utoipa::path(get, path = "/membership", tag = "membership",
    responses((status = 200, description = "OK")))]
pub async fn get_status(
    State(s): State<AppState>,
    session: RequireUserSession,
) -> ApiResult<Json<serde_json::Value>> {
    let uc = GetMembershipStatus { membership_repo: Arc::clone(&s.membership_repo) };
    Ok(Json(serde_json::json!(uc.execute(session.user_id).await?)))
}

/// Activate membership by initiating top-up payment
#[utoipa::path(post, path = "/membership/activate", tag = "membership",
    responses((status = 200, description = "OK")))]
pub async fn activate(
    State(s): State<AppState>,
    session: RequireUserSession,
    ValidatedJson(input): ValidatedJson<ActivateMembershipInput>,
) -> ApiResult<Json<serde_json::Value>> {
    let uc = ActivateMembership {
        membership_repo:  Arc::clone(&s.membership_repo),
        credit_repo:      Arc::clone(&s.credit_repo),
        provider_repo:    Arc::clone(&s.payment_provider_repo),
        payment_repo:     Arc::clone(&s.payment_repo),
        payment_gateway:  Arc::clone(&s.payment_gateway),
        topup_amount_vnd: s.membership_topup_amount,
    };
    Ok(Json(serde_json::json!(uc.execute(session.user_id, input).await?)))
}
