use serde::{Deserialize, Serialize};

pub fn get_data(response: &str) -> Result<Vec<Post>, Box<dyn std::error::Error>>{
    let data: ResponseWrapper = serde_json::from_str(response)?;
    let mut returns = vec![];
    for post in data.data.children {
        returns.push(Post {
            title: post.data.title,
            selftext: post.data.selftext,
            created: post.data.created,
            author: post.data.author,
            num_comments: post.data.num_comments,
            url: post.data.url
        });
    }
    Ok(returns)
}

#[derive(Serialize, Deserialize, Debug)]
struct ResponseWrapper {
    kind: String,
    data: Response
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Response {
    dist: u32,
    children: Vec<PostWrapper>
}
#[derive(Serialize, Deserialize, Debug)]
struct PostWrapper {
    kind: String,
    data: Post
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Post {
    pub title: String,
    pub selftext: String,
    pub author: String,
    pub url: String,
    pub created: f64,
    pub num_comments: u32
}
