use actix_files::Files as ActixFiles;
use actix_web::{
    get,
    web::{Data as WebData, Path as WebPath},
    App, HttpRequest, HttpResponse, HttpServer, Responder,
};
use serde::Deserialize;
use std::io::Result;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::{collections::HashMap, fs::File};
use tokio::time::{interval, Duration};
use walkdir::WalkDir;

#[actix_web::main]
async fn main() -> Result<()> {
    let config = load_config();
    let media_library = load_media_library(config.media_directory.clone());
    let state = WebData::new(AppState { media_library });

    HttpServer::new(move || {
        App::new()
            // 注册全局状态
            .app_data(state.clone())
            // 列出视频文件
            .service(list_videos)
            // 播放视频文件
            .service(play_video)
            // 提供视频文件的流式传输
            .service(ActixFiles::new("/videos", "./videos").show_files_listing())
    })
    .bind("0.0.0.0:3000")? // 0.0.0.0 allow inet access
    .run()
    .await
}

/// 列出指定文件夹下所有视频文件
#[get("/videos")]
async fn list_videos(data: WebData<AppState>) -> impl Responder {
    let mtx = data.media_library.list.lock();
    match mtx {
        Ok(medias) => HttpResponse::Ok().json(medias.clone()),
        Err(err) => HttpResponse::InternalServerError().json(err.to_string()),
    }
}

/// 播放指定视频文件
#[get("/play/{filename:.*}")]
async fn play_video(
    data: WebData<AppState>,
    req: HttpRequest,
    filename: WebPath<String>,
) -> impl Responder {
    let mtx = data.media_library.map.lock();
    match mtx {
        Ok(map) => {
            let path = map.get(filename.as_str()).unwrap();
            let file = actix_files::NamedFile::open(path).unwrap();
            file.into_response(&req)
        }
        Err(err) => HttpResponse::InternalServerError().json(err.to_string()),
    }
}

fn load_config() -> Config {
    let file = File::open("config.yml").expect("打开配置文件失败！");
    let config: Config = serde_yaml::from_reader(file).expect("配置文件结构转换失败");
    println!("{:?}", config);

    config
}

/// 加载媒体库数据
fn load_media_library(dir: String) -> MediaLibrary {
    let media_library: MediaLibrary = MediaLibrary::new();
    tokio::spawn(refresh_media_library(media_library.clone(), dir));

    media_library
}

/// 异步方法用于定时更新媒体库
async fn refresh_media_library(media_library: MediaLibrary, dir: String) {
    println!("{}", dir);
    let mut interval = interval(Duration::from_secs(60)); // 每60秒检查一次
    loop {
        interval.tick().await; // 等待下一个间隔

        let mut list = media_library.list.lock().unwrap();
        let mut map = media_library.map.lock().unwrap();
        let lib_map = WalkDir::new(&dir)
            .into_iter()
            .filter_map(|item| match item {
                Ok(it) => {
                    if !it.file_type().is_file() {
                        return None;
                    }

                    let extension = it.path().extension().and_then(|ext| ext.to_str());
                    match extension {
                        Some(ext) => {
                            if ["mp4", "mkv", "avi"].contains(&ext) {
                                let file_name = it.file_name().to_string_lossy().to_string();
                                let path = it.path().to_path_buf();

                                Some((file_name, path))
                            } else {
                                None
                            }
                        }
                        None => None,
                    }
                }
                Err(err) => {
                    println!("{:?}", err);
                    None
                }
            });

        // 清除当前数据
        list.clear();
        map.clear();
        for item in lib_map {
            list.append(&mut vec![item.0.clone()]);
            map.insert(item.0, item.1);
        }
    }
}

#[derive(Clone)]
struct AppState {
    media_library: MediaLibrary,
}

#[derive(Debug, Deserialize, Clone)]
struct Config {
    media_directory: String,
}

struct MediaLibrary {
    list: Arc<Mutex<Vec<String>>>,
    map: Arc<Mutex<HashMap<String, PathBuf>>>,
}

impl MediaLibrary {
    pub fn new() -> Self {
        Self {
            list: Arc::new(Mutex::new(vec![])),
            map: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

impl Clone for MediaLibrary {
    fn clone(&self) -> Self {
        let list = self.list.clone();
        let map = self.map.clone();

        Self { list, map }
    }
}
