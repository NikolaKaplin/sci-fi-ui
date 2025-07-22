use once_cell::sync::Lazy;
use reqwest::{Client, Proxy};
use std::sync::{Arc, RwLock};

// Глобальный клиент с возможностью горячего обновления
pub static HTTP_CLIENT: Lazy<Arc<RwLock<Client>>> = Lazy::new(|| {
    let client = create_client(None); // Инициализация без прокси
    Arc::new(RwLock::new(client))
});

/// Создаёт клиент с опциональным прокси
fn create_client(proxy_url: Option<&str>) -> Client {
    let builder = Client::builder();

    let builder = if let Some(url) = proxy_url {
        builder.proxy(Proxy::all(url).expect("Invalid proxy URL"))
    } else {
        builder
    };

    builder.build().expect("Failed to create HTTP client")
}

/// Обновляет прокси у глобального клиента
pub fn update_proxy(proxy_url: Option<&str>) {
    let new_client = create_client(proxy_url);
    *HTTP_CLIENT.write().unwrap() = new_client;
}
