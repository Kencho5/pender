use crate::utils::upload_struct;
use crate::{imports::*, utils};

#[derive(serde::Deserialize, Debug)]
struct Filters {
    animal: Option<String>,
    breed: Option<String>,
    age: Option<String>,
    age_type: Option<String>,
    city: Option<String>,
    price_min: Option<i16>,
    price_max: Option<i16>,
    post_type: Option<String>,
    sex: Option<String>,
}

pub async fn search_handler(mut req: Request<AppState>) -> tide::Result {
    let mut context = utils::common::get_context(&req).await?;

    let filters: Filters = req.query()?;
    let db_result = search_posts(&mut req, filters).await?;
    context.insert("posts", &db_result.0);
    context.insert("count", &db_result.1);

    let state = req.state();
    let response = state.tera.render_response("search.html", &context)?;

    Ok(response)
}

async fn search_posts(
    req: &mut Request<AppState>,
    filters: Filters,
) -> tide::Result<(Vec<upload_struct::PostStruct>, i64)> {
    let mut pg_conn = req.sqlx_conn::<Postgres>().await;

    // Base SQL query
    let mut query = "SELECT * FROM posts".to_string();
    let mut conditions = Vec::new();

    // Build conditions based on filter values
    if let Some(animal) = &filters.animal {
        if animal != "all" {
            conditions.push(format!("animal = '{}'", animal));
        }
    }
    if let Some(breed) = &filters.breed {
        conditions.push(format!("breed ILIKE '%{}%'", breed));
    }
    if let Some(age_type) = &filters.age_type {
        if age_type != "all" {
            conditions.push(format!("age <= '{}'", &filters.age.unwrap()));
            conditions.push(format!("age_type = '{}'", age_type));
        }
    }
    if let Some(city) = &filters.city {
        conditions.push(format!("city = '{}'", city));
    }
    if let Some(price_min) = &filters.price_min {
        conditions.push(format!("price >= {}", price_min));
    }
    if let Some(price_max) = &filters.price_max {
        conditions.push(format!("price <= {}", price_max));
    }
    if let Some(post_type) = &filters.post_type {
        conditions.push(format!("post_type = '{}'", post_type));
    }
    if let Some(sex) = &filters.sex {
        conditions.push(format!("sex = '{}'", sex));
    }

    if !conditions.is_empty() {
        query.push_str(" WHERE ");
        query.push_str(&conditions.join(" AND "));
    }

    query.push_str(" ORDER BY time_posted DESC LIMIT 8");

    let posts = sqlx::query_as::<_, upload_struct::PostStruct>(&query)
        .fetch_all(pg_conn.acquire().await?)
        .await?;

    let count_query = query
        .replace("SELECT *", "SELECT COUNT(*)")
        .replace("ORDER BY time_posted DESC LIMIT 8", "");

    let count: (i64,) = sqlx::query_as(&count_query)
        .fetch_one(pg_conn.acquire().await?)
        .await?;

    Ok((posts, count.0))
}
