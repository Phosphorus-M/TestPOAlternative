use salvo::{Request, Response, handler};

use crate::{entities::test::Tests, RB};



#[handler]
pub async fn get_test(req: &mut Request, res: &mut Response) {
    let uid = req.query::<i64>("id").unwrap();
    println!("{:?}", uid);
    let data = Tests::select_by_id(&mut RB.clone(), uid).await.unwrap();
    println!("{:?}", data);
    res.render(serde_json::to_string(&data.unwrap()).unwrap());
}

#[handler]
pub async fn hello_world() -> &'static str {
    "Hello World"
}