use clap::Parser;
use reqwest;
use std::fs::File;
use std::io::Write;
// use zip::read::ZipArchive;

/// 对指定目录生成模板文件
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// 输入目录地址, . 将会删除当前目录创建模板
    #[arg(short, long)]
    path: String,
}
// async fn start() -> Result<(), Box<dyn std::error::Error>> {}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    // 用户想要存放模板的目录
    let path = args.path;

    println!("想要生成模板的路径是:{}", path);

    // 下载 github 代码的地址
    let archive_url = "https://github.com/xuxchao/common-vue-template/archive/main.zip";

    // Create a reqwest client
    // Send a GET request to download the archive
    let response = reqwest::get(archive_url).await?;

    if response.status().is_success() {
        // 获取响应体的字节
        let bytes = response.bytes().await?;

        // 创建一个用于保存 ZIP 文件的 File 实例
        let mut file = File::create("common-vue-template.zip")?;

        // 将字节写入文件
        file.write_all(&bytes)?;
    } else {
        eprintln!("Download error: {}", response.status());
    }

    println!("File extracted successfully.");

    Ok(())
}
