use crate::{
    imports::*,
    utils::{self},
};

pub async fn user_handler(mut req: Request<AppState>) -> tide::Result {
    let user_id = req.param("user_id").unwrap().to_string();
    let user = get_user(&mut req, user_id).await?;

    let mut context = utils::common::get_context(&req).await?;
    context.insert("user", &user);

    let state = req.state();
    let response = state.tera.render_response("user.html", &context)?;

    Ok(response)
}

async fn get_user(
    req: &mut Request<AppState>,
    user_id: String,
) -> tide::Result<Vec<auth_struct::UserPub>> {
    let mut pg_conn = req.sqlx_conn::<Postgres>().await;
    let posts = sqlx::query_as::<_, auth_struct::UserPub>(
        "SELECT name, phone, city, facebook FROM users where id = $1",
    )
    .bind(user_id)
    .fetch_all(pg_conn.acquire().await?)
    .await?;
    Ok(posts)
}
