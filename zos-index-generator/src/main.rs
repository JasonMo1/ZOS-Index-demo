/**
 * SPDX-FileCopyrightText: 2023 Jason Mo <jasonmo2009@hotmail.com>
 *
 * SPDX-License-Identifier: Apache-2.0
 */

use std::io;
use std::fs;
use std::mem;
use std::env;
use url::Url;
use tracing::info;
use anyhow::Result;
use std::io::{Write};
use std::path::PathBuf;
use sha256::try_digest;
use std::time::Duration;
use std::fs::{OpenOptions};
use std::num::{NonZeroU8, NonZeroUsize};
use http_downloader::{HttpDownloaderBuilder,
    speed_tracker::DownloadSpeedTrackerExtension,
    status_tracker::DownloadStatusTrackerExtension,
    breakpoint_resume::DownloadBreakpointResumeExtension,
    bson_file_archiver::{ArchiveFilePath, BsonFileArchiverBuilder}};
use git2::{Repository, Signature, Direction, PushOptions, RemoteCallbacks, Cred};

fn main() {
    let helpmsg = "usage: zos-index-generator [new][help]";
    let args: Vec<String> = env::args().collect();
    println!("zos-index-generator v0.1.0\n");

    if args.len() > 1 {
        if args[1] == "new" {
            match clone_repo() { // 使用 match 来处理结果
                Ok(_clone) => println!("\nClone successfully!"),
                Err(e) => println!("\nClone error: {}", e), // 打印错误信息
            }
            let name = read_name();
            let urls = read_urls();
            let inptimg = read_img(urls.clone());
            let version = read_ver(&inptimg);
            let hash = read_hash(&inptimg);

            let mut newindex = format!("        {{\n            \"name\":\"{0}\",\n            \"urls\":\n            [", name);
            let urlindex = 0;
            for url in urls {
                let comma: String;
                if urlindex == 0 {
                    comma = "".to_string();
                }
                else {
                    comma = ",".to_string();
                }
                newindex += format!("\n                \"{0}\"{1}", url, comma).as_str();
            }
            newindex += format!("\n            ],\n            \"version\":\"{0}\",\n            \"hash\":\"{1}\"\n        }}", version, hash).as_str();
            
            match write_index(&newindex.to_string()) { // 使用 match 来处理结果
                Ok(()) => println!("\nWritten into the index successfully"),
                Err(e) => println!("\nError while writing into the index: {}", e), // 打印错误信息
            }

            match commit_repo() { // 使用 match 来处理结果
                Ok(()) => println!("\nCommit successfully!"),
                Err(e) => println!("\nCommit error: {}", e), // 打印错误信息
            }

            match push_repo() { // 使用 match 来处理结果
                Ok(()) => 
                {
                    println!("\nPushed successfully, please wait for merge!");
                    fs::remove_dir_all("cache");
                },
                Err(e) => {
                    println!("\nPush error: {}\nPlease remove \"cache\" directory by yourself", e);
                },
            }
        }
        else if args[1] == "help" {
            println!("{}", helpmsg);
        }
    }
    else{
        println!("{}", helpmsg);
    }
}

fn read_name() -> String {
    println!("The name of your image [less than 25 half-width characters]\nformat [os-${{git tag}}-publisher+program inside]:");
    let mut name :String;
    loop {
        name = read_line();
        if name.len() > 25 {
            println!("\nError: Input a name over 25 characters");
            println!("Please input again:");
            continue;
        }
        else if name.len() <= 25 {
            break
        }
    }
    return name;
    
}

fn read_urls() -> Vec<String> {
    let mut index = 0;
    let mut urllist :Vec<String> = vec![];
    loop {
        println!("\nThe value of \"urls\"({index}):");
        let url = read_line();
        if url != "" {
            urllist.push(url);
        }
        else {
            println!("\nError: Url shouldn't be an empty string");
            println!("Please input again:");
            let urlagain = read_line();
            urllist.push(urlagain);
        }

        println!("\nAre there more urls? (Y/n) [default n]:");
        let moreurls = read_line();
        let murlres = ynprompt(&moreurls);
        if murlres == false {
            break;
        }
        index += 1;
    }
    return urllist;
}

fn read_img(urls: Vec<String>) -> String {

    let url = &urls[0];
    let down_res = download_img(&url, "cache/imgs");
    let imgsplit: Vec<_> = url.split("/").collect();
    let imgnames1 = imgsplit[imgsplit.len() - 1];
    let imgname = "cache/imgs/".to_owned()+&imgnames1;
    mem::drop(down_res);
    return imgname;
}

fn read_ver(inptimg :&String) -> String {
    let mut version :String;
    let vers1 = fs::read(inptimg).unwrap();
    let vers2 = &vers1[769..787];

    println!("\nWhat's your name?");
    let usrname = read_line();
    let program :String;
    println!("\nAre there any program in this image? (Y/n):");
    let rdynprogram = read_line();
    let ynprogram = ynprompt(&rdynprogram);
    if ynprogram == true {
        println!("\nPlease input the name of your program:");
        let programname = read_line();
        program = "+".to_string()+&programname;
    }
    else {
        program = "".to_string()
    }
    let vers3 = String::from_utf8_lossy(&vers2).to_string();
    version = "zos-".to_owned()+&vers3.trim()+"-main-"+&usrname+&program;


    println!("\nIs this your image version? (Y/n)\n[{}]", version);
    let ynvergetln = read_line();
    let ynverget = ynprompt(&ynvergetln);
    if ynverget == false {
        println!("\nPlease input your version in this format [os-${{git describe --tags}}-branch-publisher]:");
        version = read_line();
    }
    return version;
}

fn read_hash(inptimg :&String) -> String {
    let computed_hash = try_digest(std::path::Path::new(&inptimg)).unwrap();
    return computed_hash;
}

fn write_index(index: &String) -> std::io::Result<()> {
    // 打开文件
    let text = fs::read_to_string("cache/repo/index.json").unwrap();
    let mut lines: Vec<_> = text.split("\n").collect();

    let indexlines: Vec<_> = index.split("\n").collect();
    let mut insindex = lines.len() - 2;
    lines[insindex - 1] = "        },";
    for indexline in indexlines {
        lines.insert(insindex, indexline);
        insindex += 1;
    }

    let mut file = OpenOptions::new()
        .write(true).open("cache/repo/index.json")?;

    let mut index2 = 0;
    let mut n = "\n";
    let lastline = lines.len() - 1;
    for line in lines {
        if index2 == lastline {
            n = "";
        }
        else {
            n = n;
        }
        let linew = line.clone().to_owned() + n;
        file.write(linew.as_bytes())?;
        index2 += 1;
    }
    Ok(())
}

fn clone_repo() -> Result<git2::Repository, git2::Error> {
    println!("Cloning git repo...");
    let url = "git@github.com:JasonMo1/ZOS-Index-demo.git";
    let callbacks = create_callbacks();
    let mut fetch_options = git2::FetchOptions::new();
    fetch_options.remote_callbacks(callbacks);
    let mut builder = git2::build::RepoBuilder::new();
    builder.fetch_options(fetch_options);
    let clone = builder.clone(url, std::path::Path::new("cache/repo"))?; // 使用 clone 函数来克隆远程仓库
    Ok(clone) // 返回成功结果
}

fn commit_repo() -> Result<(), git2::Error> {
    let repo = Repository::open("cache/repo")?; // 打开一个仓库
    let mut index = repo.index()?; // 获取索引
    let tree_id = index.write_tree()?; // 写入树对象
    let tree = repo.find_tree(tree_id)?; // 查找树对象
    let parent = repo.head()?.peel_to_commit()?; // 查找父提交
    let sig = Signature::now("name", "email")?; // 创建签名
    let message = "Index: Update index list"; // 创建消息
    let commit_id = repo.commit(Some("HEAD"), &sig, &sig, message, &tree, &[&parent])?; // 创建提交对象
    repo.set_head_detached(commit_id)
}

fn push_repo_() -> Result<(), git2::Error> {
    let repo = Repository::open("cache/repo")?; // 打开一个仓库

    let mut remote = repo.find_remote("origin")?;

    remote.connect_auth(Direction::Push, Some(create_callbacks()), None).unwrap();
    repo.remote_add_push("origin", "refs/heads/temp:refs/heads/temp").unwrap();
    let mut push_options = PushOptions::default();
    let callbacks = create_callbacks();
    push_options.remote_callbacks(callbacks);
    // 推送本地更改到远程仓库，并返回结果
    remote.push(&["refs/heads/temp:refs/heads/temp"], Some(&mut push_options)).unwrap();

    std::mem::drop(remote);

    Ok(())
}

fn push_repo() -> Result<(), git2::Error> {
    let repo = Repository::open("cache/repo")?;
    let url = "git@github.com:JasonMo1/ZOS-Index-demo.git";

    let mut remote = match repo.find_remote("origin") {
        Ok(r) => r,
        Err(_) => repo.remote("origin", url)?,
    };
    // remote.connect(Direction::Push)?;
    remote.connect_auth(Direction::Push, Some(create_callbacks()), None)?;
    remote.push(&["refs/heads/temp:refs/heads/temp"], None)
}

///////////
// Utils //
///////////

fn create_callbacks<'a>() -> RemoteCallbacks<'a>{
    let mut callbacks = RemoteCallbacks::new();
    // callbacks.credentials(|_str, _str_opt, _cred_type| {
    //     Cred::ssh_key(
    //         "push",
    //         Some(std::path::Path::new("ssh_key/pushzig_rsa.pub")),
    //         std::path::Path::new("ssh_key/pushzig_rsa"),
    //         None,
    //     )
    // });
    callbacks.credentials(|_url, username_from_url, _allowed_types| {
        // Use the username from the url or a default one
        let username = username_from_url.unwrap_or("git");
        Cred::ssh_key(
            username,
            Some(&std::path::Path::new("C:/Users/AdminJason/Desktop/ZOW/Zeal-8-bit/ZOS-Index-demo/zos-index-generator/src/ssh_key/pushzig_rsa.pub")),
            std::path::Path::new("ssh_key/pushzig_rsa"),
            None,
        )
    });
    // callbacks.credentials(|_url, username_from_url, _allowed_types| {
    //     tracing::info!("Allowed types: {:?}", _allowed_types);

    //     Cred::ssh_key(
    //         username_from_url.unwrap(),
    //         None,
    //         std::path::Path::new("ssh_key/pushzig_rsa"),
    //         None,
    //     )
    // });
    callbacks
}

#[tokio::main]
async fn download_img(url: &str, save: &str) -> Result<()> {
    {
        tracing_subscriber::fmt::init();
    }

    let save_dir = PathBuf::from(save);
    let test_url = Url::parse(url)?;
    let (mut downloader, (status_state, speed_state, ..)) =
        HttpDownloaderBuilder::new(test_url, save_dir)
            .chunk_size(NonZeroUsize::new(1024 * 1024 * 10).unwrap()) // 块大小
            .download_connection_count(NonZeroU8::new(3).unwrap())
            .build((
                // 下载状态追踪扩展
                // by cargo feature "status-tracker" enable
                DownloadStatusTrackerExtension { log: true },
                // 下载速度追踪扩展
                // by cargo feature "speed-tracker" enable
                DownloadSpeedTrackerExtension { log: true },
                // 断点续传扩展，
                // by cargo feature "breakpoint-resume" enable
                DownloadBreakpointResumeExtension {
                    // BsonFileArchiver by cargo feature "bson-file-archiver" enable
                    download_archiver_builder: BsonFileArchiverBuilder::new(ArchiveFilePath::Suffix("bson".to_string()))
                }
            ));

    println!("");
    info!("Preparing download");
    let download_future = downloader.prepare_download()?;

    let _status = status_state.status(); // get download status， 获取状态
    let _status_receiver = status_state.status_receiver; //status watcher，状态监听器
    let _byte_per_second = speed_state.download_speed(); // get download speed，Byte per second，获取速度，字节每秒
    let _speed_receiver = speed_state.receiver; // get download speed watcher，速度监听器

    // downloader.cancel().await; // 取消下载

    // 打印下载进度
    // Print download Progress
    tokio::spawn({
        let mut downloaded_len_receiver = downloader.downloaded_len_receiver().clone();
        let total_size_future = downloader.total_size_future();
        async move {
            let total_len = total_size_future.await;
            if let Some(total_len) = total_len {
                info!("Total size: {:.2} Mb",total_len.get() as f64 / 1024_f64/ 1024_f64);
            }
            while downloaded_len_receiver.changed().await.is_ok() {
                let progress = *downloaded_len_receiver.borrow();
                if let Some(total_len) = total_len {
                    info!("Download Progress: {} %,{}/{}",progress*100/total_len,progress,total_len);
                }

                tokio::time::sleep(Duration::from_millis(1000)).await;
            }
        }
    });

    info!("Start downloading");
    let dec = download_future.await?;
    info!("Downloading end cause: {:?}", dec);
    Ok(())
}

fn read_line() -> String {
    let mut inptsrc: String = String::new();

    io::stdin()
        .read_line(&mut inptsrc)
        .expect("Error: Failed to read input");

    inptsrc.pop();
    inptsrc.pop();

    return inptsrc;
}

fn ynprompt(inputstr:&String) -> bool {
    let res: bool;
    if inputstr == "Y" {
        res = true;
    }
    else if inputstr == "Yes" {
        res = true;
    }
    else if inputstr == "y" {
        res = true;
    }
    else if inputstr == "yes" {
        res = true;
    }
    else if inputstr == "N" {
        res = false;
    }
    else if inputstr == "No" {
        res = false;
    }
    else if inputstr == "n" {
        res = false;
    }
    else if inputstr == "no" {
        res = false;
    }
    else if inputstr == "" {
        res = false;
    }
    else {
        println!("\nError: Please input \"Y\" or \"n\"");
        println!("Please input again:");
        let readagain = read_line();
        res = ynprompt(&readagain);
    }

    return res;
}