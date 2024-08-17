use reqwest::blocking::Client;
use std::io;
use serde_json::json;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 从用户获取输入
    let mut input = String::new();
    println!("Username:");
    io::stdin().read_line(&mut input)?;
    let input = input.trim(); // 去除输入中的换行符

    // 将输入转换为所需的JSON格式
    let body = json!([input]);

    // 创建HTTP客户端并发送POST请求
    let client = Client::new();
    let res = client.post("https://littleskin.cn/api/yggdrasil/api/profiles/minecraft")
        .json(&body)
        .send()?;

    // 打印响应内容
    println!("Response: {:?}", res.text()?);
    Ok(())
}
