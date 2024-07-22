use axum::{body::Body, debug_handler, http::Response, Json};
use serde::{Deserialize, Serialize};

use crate::{database::nodes::{DoneStartedChanges, NewNode, Node, TreeNode}, util::id::Id};

#[derive(Deserialize)]
pub struct RequestWithId {
    id: Option<Id>,
}

#[debug_handler]
pub async fn load(Json(req): Json<RequestWithId>) -> Response<Body> {
    if let Ok(res) = Node::load(req.id) {
        if let Ok(str) = serde_json::to_string(&res) {
            return Response::builder()
                .status(200)
                .header("Content-Type", "application/json")
                .body(Body::from(str))
                .unwrap()
        }
    }
    return Response::builder()
        .status(500)
        .header("Content-Type", "text/plain")
        .body(Body::from("Internal Server Error"))
        .unwrap();
}

pub async fn create(Json(new_task): Json<NewNode>) -> Response<Body> {
    if let Ok(task) = new_task.save() {
        let tree: TreeNode = task.into();
        if let Ok(str) = serde_json::to_string(&tree) {
            return Response::builder()
                .status(200)
                .header("Content-Type", "application/json")
                .body(Body::from(str))
                .unwrap();
        };

    };
    Response::builder()
        .status(500)
        .header("Content-Type", "text/plain")
        .body(Body::from("Internal Server Error"))
        .unwrap()
}


#[derive(Deserialize)]
pub struct ExpandOptions {
    ids: Vec<Id>,
    st: bool,
}

pub async fn expand(Json(expand_options): Json<ExpandOptions>) -> Response<Body> {
    if Node::expand_collapse(&expand_options.ids, &expand_options.st).is_ok() {
        return Response::builder()
            .status(200)
            .header("Content-Type", "application/json")
            .body(Body::from("Ok"))
            .unwrap();
    };

    Response::builder()
        .status(500)
        .header("Content-Type", "text/plain")
        .body(Body::from("Internal Server Error"))
        .unwrap()
}

pub async fn delete(Json(delete_target): Json<RequestWithId>) -> Response<Body> {
    if let Some(id) = delete_target.id {
        if Node::delete(id).is_ok() {
            return Response::builder()
                .status(200)
                .header("Content-Type", "text/plain")
                .body(Body::from("Ok"))
                .unwrap()
        }
    }

    Response::builder()
        .status(500)
        .header("Content-Type", "text/plain")
        .body(Body::from("Internal Server Error"))
        .unwrap()
}


pub async fn update_completion(Json(affected): Json<Vec<DoneStartedChanges>>) -> Response<Body> {
    if Node::update_completion(affected).is_ok() {
        return Response::builder()
            .status(200)
            .header("Content-Type", "text/plain")
            .body(Body::from("Ok"))
            .unwrap()
    }

    Response::builder()
        .status(500)
        .header("Content-Type", "text/plain")
        .body(Body::from("Internal Server Error"))
        .unwrap()
}
