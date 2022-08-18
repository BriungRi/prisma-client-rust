use std::sync::Arc;

use rspc::{Config, Router};

mod db;

type Ctx = Arc<db::PrismaClient>;

#[tokio::main]
async fn main() {
    Router::<Ctx>::new()
        .config(Config::new().export_ts_bindings("./bindings.ts"))
        .query("users", |db: Ctx, _: ()| async move {
            db.user().find_many(vec![]).exec().await.map_err(Into::into)
        })
        .query("userNames", |db: Ctx, _: ()| async move {
            db.user()
                .find_many(vec![])
                .select(db::user::select!(display_name))
                .exec()
                .await
                .map_err(Into::into)
        })
        .query("usersSelectPosts", |db: Ctx, _: ()| async move {
            db.user()
                .find_many(vec![])
                .select(db::user::select! {
                    posts(vec![]).skip(1) {
                        id
                        content
                        user {
                            id
                            display_name
                        }
                    }
                })
                .exec()
                .await
                .map_err(Into::into)
        })
        .build();
}
