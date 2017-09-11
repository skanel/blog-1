use dal::models::post::*;
use dal::diesel_pool::DB;
use std::collections::HashMap;
use rocket::response::Redirect;
use rocket_contrib::Template;

use rocket_contrib::Json;

#[get("/show_post")]
pub fn show_post(db: DB) -> Json<Vec<PostView>> {
    let result = Post::query_latest_five(db.conn());
    let view_posts: Vec<PostView> = result
        .iter()
        .map(PostView::model_convert_to_postview)
        .collect::<Vec<PostView>>();
    Json(view_posts)
}

#[get("/<id>")]
pub fn get_post_by_id(id: i32, db: DB) -> Template {
    let result = Post::query_by_id(db.conn(), id);
    let mut context = HashMap::new();
    context.insert("post", result.first());
    Template::render("post", &context)
    // Json(result)
}

#[get("/post")]
pub fn get_post(db: DB) -> Template {
    let result = Post::query_all_published(db.conn());
    let mut map = HashMap::new();
    map.insert("posts", result);
    Template::render("list", &map)
}
#[get("/post_list")]
pub fn get_post_list(db: DB) -> Json<Vec<PostView>> {
    let result = Post::query_all_published(db.conn());
    let view_posts: Vec<PostView> = result
        .iter()
        .map(PostView::model_convert_to_postview)
        .collect::<Vec<PostView>>();
    Json(view_posts)
}