use clap::Parser;
use reqwest;
use std::fs::File;
use std::io;
use std::io::Write;
use std::path::PathBuf;
use zip::read::ZipArchive;

/// 弹出一层路径
/// 原路径:　/a/b/c
/// 处理后: /b/c
fn unshift_path(path: &PathBuf) -> PathBuf {
    let mut components = path.components();

    // 跳过第一个组件（顶级路径）
    components.next();

    // 从剩余的组件中构建新的 PathBuf
    components.as_path().to_path_buf()
}

/// 在最前面插入一层路径一层路径
///
/// # example
/// ```
/// shift_path("/c", "/a/b") // /c/a/b
/// ```
fn shift_path(path: &str, path_but: &PathBuf) -> PathBuf {
    let mut n = PathBuf::new();
    n = n.join(path);
    n.join(path_but)
}

/// 解压文件夹到目标文件夹
///
/// path 路径, 从这个路径读取压缩文件
/// to_path 目标路径，将文件解压的路径
/// unshift 取消一级路径
///
/// # example
/// test.zip => test/xxx/实际内容
/// ```
/// unzip("test.zip", "toTest", true)
/// test.zip => test/实际内容
///
/// unzip("test.zip", "toTest", false)
/// test.zip => test/xxx/实际内容
/// ```
fn unzip(path: &str, to_path: &str, unshift: bool) -> zip::result::ZipResult<()> {
    let zip_file = File::open(path)?;
    let mut zip = ZipArchive::new(zip_file)?;
    for i in 0..zip.len() {
        let mut file = zip.by_index(i)?;
        let mut outpath = file.mangled_name();
        if unshift {
            outpath = unshift_path(&outpath);
        }
        outpath = shift_path(to_path, &outpath);
        println!("{:?}", outpath);
        if file.is_dir() {
            std::fs::create_dir_all(&outpath)?;
        } else {
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    std::fs::create_dir_all(&p)?;
                }
            }
            let mut outfile = File::create(&outpath)?;
            io::copy(&mut file, &mut outfile)?;
        }
    }

    Ok(())
}

/// 下载 github 仓库的 zip 包
///
/// archive_url github 仓库直接下载 main.zip 包的地址
/// name 下载的压缩包的名字
async fn download_github_zip(
    archive_url: &str,
    name: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // Create a reqwest client
    // Send a GET request to download the archive
    let response = reqwest::get(archive_url).await?.error_for_status()?;

    // 获取响应体的字节
    let bytes = response.bytes().await?;

    // 创建一个用于保存 ZIP 文件的 File 实例
    let mut file = File::create(name)?;
    // 将字节写入文件
    file.write_all(&bytes)?;

    Ok(())
}

/// 对指定目录生成模板文件
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// 输入目录地址, . 将会删除当前目录创建模板
    #[arg(short, long)]
    path: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    // 用户想要存放模板的目录
    let path = args.path;

    println!("想要生成模板的路径是:{}", path);

    let zip_path = &format!("{}.zip", path);

    // 下载 github 代码的地址
    let archive_url = "https://github.com/xuxchao/common-vue-template/archive/main.zip";
    download_github_zip(archive_url, zip_path).await?;

    unzip(&zip_path, &path, true)?;

    Ok(())
}
