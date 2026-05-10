use serde::Deserialize;

#[derive(Deserialize)]
struct IpInfo {
    ip: String,
    country: String,
    city: String,
    connection: Connection,
}

#[derive(Deserialize)]
struct Connection {
    isp: String,
}

#[tokio::main]
async fn main() {
    println!("Проверяю ваше соединение...");

    let client = reqwest::Client::new();

    let info = client
        .get("https://ipwho.is/")
        .send()
        .await
        .unwrap()
        .json::<IpInfo>()
        .await
        .unwrap();

    println!("IP: {}", info.ip);
    println!("Страна: {}", info.country);
    println!("Город: {}", info.city);
    println!("Провайдер: {}", info.connection.isp);
}
