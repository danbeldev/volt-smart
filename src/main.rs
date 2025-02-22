use reqwest::{Client, header::{AUTHORIZATION, CONTENT_TYPE}};
use serde_json::json;
use std::fs;
use std::env;
use dotenvy::dotenv;
use tokio::time::{sleep, Duration};

const API_URL: &str = "https://api.iot.yandex.net/v1.0/devices/actions";
const BATTERY_PATH: &str = "/sys/class/power_supply/BAT0/capacity";

#[tokio::main]
async fn main() {
    dotenv().ok();

    let token = env::var("YANDEX_IOT_TOKEN").expect("Не найден YANDEX_IOT_TOKEN");
    let device_id = env::var("YANDEX_IOT_DEVICE_ID").expect("Не найден YANDEX_IOT_DEVICE_ID");

    let client = Client::new();

    loop {
        if let Some(level) = get_battery_level() {
            println!("🔋 Текущий уровень заряда: {}%", level);

            if level <= 25 {
                println!("🔴 Заряд низкий! Включаем розетку...");
                send_request(&client, &token, &device_id, true).await;
            } else if level == 100 {
                println!("🟢 Заряд полный! Выключаем розетку...");
                send_request(&client, &token, &device_id, false).await;
            }
        } else {
            println!("⚠️ Не удалось получить уровень заряда.");
        }

        sleep(Duration::from_secs(60)).await;
    }
}

fn get_battery_level() -> Option<u32> {
    fs::read_to_string(BATTERY_PATH)
        .ok()
        .and_then(|s| s.trim().parse::<u32>().ok())
}

async fn send_request(client: &Client, token: &str, device_id: &str, power_on: bool) {
    let payload = json!({
        "devices": [{
            "id": device_id,
            "actions": [{
                "type": "devices.capabilities.on_off",
                "state": {
                    "instance": "on",
                    "value": power_on
                }
            }]
        }]
    });

    let response = client.post(API_URL)
        .header(AUTHORIZATION, format!("Bearer {}", token))
        .header(CONTENT_TYPE, "application/json")
        .json(&payload)
        .send()
        .await;

    match response {
        Ok(res) if res.status().is_success() => {
            println!("✅ Успешно отправлен запрос на {}", if power_on { "включение" } else { "выключение" });
        }
        Ok(res) => {
            println!("❌ Ошибка {}: {:?}", res.status(), res.text().await);
        }
        Err(err) => {
            println!("⚠️ Ошибка подключения к API Яндекс IoT: {}", err);
        }
    }
}