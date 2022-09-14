mod tree_gen;
// use actix_web::{get, post, App, HttpResponse, HttpServer, Responder};

fn main() -> Result<(), std::io::Error> {
    let path = "./data/active_users_audience.csv";
    let tree = tree_gen::generate_tree_from_csv(path).unwrap();

    println!("Root: {} ", tree.root_hex().unwrap());
    println!("Done");

    Ok(())
}

// #[actix_rt::main]
// async fn main() -> std::io::Result<()> {
//     let port = 9000;

//     HttpServer::new(|| App::new().service(index).service(create_tree))
//         .bind(format!("127.0.0.1:{}", port))?
//         .run()
//         .await
// }

// #[get("/")]
// async fn index() -> impl Responder {
//     HttpResponse::Ok().body("gm")
// }

// #[post("/create_tree")]
// async fn create_tree() -> impl Responder {
//     let path = "./data/tide.csv";
//     let tree = tree_gen::generate_tree_from_csv(path).unwrap();

//     HttpResponse::Ok().body(format!("{:?}", tree.root_hex()))
// }
