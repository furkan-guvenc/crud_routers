use serde::Serialize;
use serde_json::{json, Value};

pub struct TestApp{
    pub address: String,
    pub api_client: reqwest::Client
}

impl TestApp {
    pub fn new(mut address: String, prefix: &str) -> Self{
        address.push('/');
        address.push_str(prefix);
        Self{
            address,
            api_client: reqwest::Client::new()
        }
    }
    async fn list_all(&self, skip: Option<u32>, limit: Option<u32>) -> reqwest::Response {
        let mut r = self.api_client.get(&self.address);
        if let Some(skip) = skip {
            r = r.query(&[("skip", skip.to_string())]);
        }

        if let Some(limit) = limit {
            r = r.query(&[("limit", limit.to_string())]);
        }

        r
        .send()
        .await
        .expect("Failed to execute request.")
    }
    async fn get(&self, id: i64) -> reqwest::Response {
        self.api_client.get(&format!("{}/{}", &self.address, id))
            .send()
            .await
            .expect("Failed to execute request.")
    }
    async fn create(&self, body: impl Serialize) -> reqwest::Response {
        self.api_client.post(&self.address)
            .header("Content-Type", mime::APPLICATION_JSON.as_ref())
            .body(reqwest::Body::from(serde_json::to_vec(&body).unwrap()))
            .send()
            .await
            .expect("Failed to execute request.")
    }
    async fn update(&self, id: i64, body: impl Serialize) -> reqwest::Response {
        self.api_client.put(&format!("{}/{}", &self.address, id))
            .body(reqwest::Body::from(serde_json::to_vec(&body).unwrap()))
            .header("Content-Type", mime::APPLICATION_JSON.as_ref())
            .send()
            .await
            .expect("Failed to execute request.")
    }
    async fn delete(&self, id: i64) -> reqwest::Response {
        self.api_client.delete(&format!("{}/{}", &self.address, id))
            .send()
            .await
            .expect("Failed to execute request.")
    }
    async fn delete_all(&self) -> reqwest::Response {
        self.api_client.delete(&self.address)
            .send()
            .await
            .expect("Failed to execute request.")
    }
}

pub async fn e2e_test(app: TestApp){
    // no posts in the beginning
    let response = app.list_all(None, None).await;

    assert!(response.status().is_success());
    assert_eq!("[]", response.text().await.unwrap(), "All posts have to be deleted in db");

    // insert a post
    let response = app.create(
        &json!({"title": "Post", "body": "Body", "published": false})
    ).await;

    assert!(response.status().is_success());

    let body = response.bytes().await.unwrap();
    let body: Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(*body.get("title").unwrap(), json!("Post"));
    assert_eq!(*body.get("body").unwrap(), json!("Body"));
    assert_eq!(*body.get("published").unwrap(), json!(false));

    // insert 2 more and get all
    let _ = app.create(
        &json!({"title": "Post", "body": "Body", "published": false})
    ).await;

    let _ = app.create(
        &json!({"title": "Post", "body": "Body", "published": false})
    ).await;

    // get all 3 of them
    let response = app.list_all(None, None).await;

    assert!(response.status().is_success());

    let body = response.bytes().await.unwrap();
    let mut body: Value = serde_json::from_slice(&body).unwrap();
    let posts = body.as_array_mut().unwrap();
    assert_eq!(posts.len(), 3);
    for body in posts.iter() {
        assert_eq!(*body.get("title").unwrap(), json!("Post"));
        assert_eq!(*body.get("body").unwrap(), json!("Body"));
        assert_eq!(*body.get("published").unwrap(), json!(false));
    }

    let post_ids = posts.iter_mut().map(|p| p.as_object_mut().unwrap().remove("id").unwrap().as_i64().unwrap()).collect::<Vec<i64>>();

    // skip 1 and limit 1
    let response = app.list_all(Some(1), Some(1)).await;

    assert!(response.status().is_success());

    let body = response.bytes().await.unwrap();
    let mut body: Value = serde_json::from_slice(&body).unwrap();
    let posts = body.as_array_mut().unwrap();
    assert_eq!(posts.len(), 1);

    assert_eq!(posts[0].get("id").unwrap().as_i64().unwrap(), post_ids[1]);

    // update the first one
    let first_post_id = post_ids[0];
    *posts[0].get_mut("body").unwrap() = json!("First Post Body");

    let response = app.update(first_post_id, &posts[0]).await;

    assert!(response.status().is_success());

    // get the updated one
    let response = app.get(first_post_id).await;
    assert!(response.status().is_success());
    let body = response.bytes().await.unwrap();
    let body: Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(*body.get("id").unwrap(), json!(first_post_id));
    assert_eq!(*body.get("title").unwrap(), json!("Post"));
    assert_eq!(*body.get("body").unwrap(), json!("First Post Body"));
    assert_eq!(*body.get("published").unwrap(), json!(false));

    // delete first one
    let response = app.delete(first_post_id).await;

    assert!(response.status().is_success());

    // try to get the deleted one
    let response = app.get(first_post_id).await;

    assert!(response.status().is_success());
    let body = response.bytes().await.unwrap();
    assert_eq!(&body[..], b"null");

    // get 2 of them
    let response = app.list_all(None, None).await;

    assert!(response.status().is_success());

    let body = response.bytes().await.unwrap();
    let mut body: Value = serde_json::from_slice(&body).unwrap();
    let posts = body.as_array_mut().unwrap();
    assert_eq!(posts.len(), 2);
    for body in posts.iter() {
        assert_eq!(*body.get("title").unwrap(), json!("Post"));
        assert_eq!(*body.get("body").unwrap(), json!("Body"));
        assert_eq!(*body.get("published").unwrap(), json!(false));
    }

    // delete all
    let response = app.delete_all().await;

    assert!(response.status().is_success());

    // all posts should be deleted
    let response = app.list_all(None, None).await;

    assert!(response.status().is_success());

    let body = response.bytes().await.unwrap();
    assert_eq!(&body[..], b"[]");
}