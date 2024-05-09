use crate::models::Container;
use askama::Template;

#[derive(Template)]
#[template(path = "index.html")]
pub struct Index {}

#[derive(Template)]
#[template(path = "containers.html")]
pub struct Containers {
    pub containers: Vec<Container>,
}
