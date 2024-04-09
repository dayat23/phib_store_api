use loco_rs::prelude::*;

use crate::models::_entities::users::{Entity, Model};
use crate::{models::_entities::users, views::user::CurrentResponse};

async fn current(auth: auth::JWT, State(ctx): State<AppContext>) -> Result<Json<CurrentResponse>> {
    let user = users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;
    format::json(CurrentResponse::new(&user))
}

async fn list(auth: auth::JWT, State(ctx): State<AppContext>) -> Result<Json<Vec<Model>>> {
    let _user = users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;
    format::json(Entity::find().all(&ctx.db).await?)
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("users")
        .add("/", get(list))
        .add("/current", post(current))
}
