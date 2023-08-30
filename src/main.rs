use actix_web::http::{header::ContentType, StatusCode};
use actix_web::{get, post, web::Data, App, HttpResponse, HttpServer, Responder};
use mycoin::blockchain::blockchain::BlockChain;
use mycoin::settings::Settings;
use mycoin::transaction::Transaction;
use serde_json::json;
use std::env;
use std::sync::Mutex;

#[get("/mine")]
async fn mine(
    blockchain_store: Data<Mutex<BlockChain>>,
    settings_store: Data<Settings>,
) -> impl Responder {
    match blockchain_store.lock() {
        Ok(mut blockchain) => {
            blockchain.add_transaction(
                settings_store.address.clone(),
                settings_store.name.clone(),
                1.0,
            );
            let new_block = blockchain.mine_block();
            let data = json!({
                "message": "Block is mined",
                "index": new_block.index,
                "proof": new_block.proof,
                "previous_hash": new_block.previous_hash,
                "timestamp": new_block.timestamp,
                "transactions": new_block.transactions
            })
            .to_string();

            HttpResponse::build(StatusCode::OK)
                .content_type(ContentType::json())
                .body(data)
        }
        _ => HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR).into(),
    }
}

#[get("/chain")]
async fn chain(blockchain_store: Data<Mutex<BlockChain>>) -> impl Responder {
    match blockchain_store.lock() {
        Ok(blockchain) => {
            let data = json!({
                "length": blockchain.chain.len(),
                "chain": blockchain.chain
            })
            .to_string();

            HttpResponse::build(StatusCode::OK)
                .content_type(ContentType::json())
                .body(data)
        }
        _ => HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR).into(),
    }
}

#[get("/is_valid")]
async fn is_valid(blockchain_store: Data<Mutex<BlockChain>>) -> impl Responder {
    match blockchain_store.lock() {
        Ok(blockchain) => {
            let data = json!({
                "is_valid": blockchain.is_valid(&blockchain.chain),
            })
            .to_string();

            HttpResponse::build(StatusCode::OK)
                .content_type(ContentType::json())
                .body(data)
        }
        _ => HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR).into(),
    }
}

#[post("/add_transaction")]
async fn add_transaction(
    blockchain_store: Data<Mutex<BlockChain>>,
    transaction_data: actix_web::web::Json<Transaction>,
) -> impl Responder {
    match blockchain_store.lock() {
        Ok(mut blockchain) => {
            let index = blockchain.add_transaction(
                transaction_data.sender.clone(),
                transaction_data.receiver.clone(),
                transaction_data.amount,
            );
            let data = json!({
                "message": "Transaction added",
                "block_index": index
            })
            .to_string();

            HttpResponse::build(StatusCode::CREATED)
                .content_type(ContentType::json())
                .body(data)
        }
        _ => HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR).into(),
    }
}

#[derive(serde::Deserialize)]
struct NodesData {
    addresses: Vec<String>,
}

#[post("/connect_node")]
async fn connect_node(
    blockchain_store: Data<Mutex<BlockChain>>,
    nodes_data: actix_web::web::Json<NodesData>,
) -> impl Responder {
    match blockchain_store.lock() {
        Ok(mut blockchain) => {
            for address in nodes_data.addresses.iter() {
                blockchain.add_node(address.clone());
            }

            let data = json!({
                "message": "All the nodes are now connected",
                "total_nodes": blockchain.nodes
            })
            .to_string();

            HttpResponse::build(StatusCode::CREATED)
                .content_type(ContentType::json())
                .body(data)
        }
        _ => HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR).into(),
    }
}

#[get("/replace_chain")]
async fn replace_chain(blockchain_store: Data<Mutex<BlockChain>>) -> impl Responder {
    match blockchain_store.lock() {
        Ok(mut blockchain) => {
            let message = if blockchain.replace_chain().await {
                "Chain is replaced by the longest one"
            } else {
                "All good. Chain is the longest one"
            };

            let data = json!({
                "message": message,
                "length": blockchain.chain.len(),
                "chain": blockchain.chain
            })
            .to_string();

            HttpResponse::build(StatusCode::OK)
                .content_type(ContentType::json())
                .body(data)
        }
        _ => HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR).into(),
    }
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let host = &args[1];
    let port = &args[2];
    let name = &args[3];

    #[allow(clippy::mutex_atomic)] // it's intentional.
    let blockchain_store = Data::new(Mutex::new(BlockChain::new()));
    let settings = Settings {
        name: name.clone(),
        address: format!("{}:{}", host, port),
    };
    let settings_store = Data::new(settings);

    HttpServer::new(move || {
        App::new()
            .app_data(blockchain_store.clone())
            .app_data(settings_store.clone())
            .service(mine)
            .service(chain)
            .service(is_valid)
            .service(add_transaction)
            .service(connect_node)
            .service(replace_chain)
    })
    .bind((host.clone(), port.parse::<u16>().unwrap()))?
    .run()
    .await
}
