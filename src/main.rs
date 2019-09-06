mod lib;

use std::env;
use std::fs::remove_dir_all;
use std::panic;
use rayon::ThreadPoolBuilder;

use lib::downloader::parallel::ParallelDownloader;
use lib::downloader::single::SingleDownloader;
use lib::utils::{Download, get_content_length};

fn main() {
    ThreadPoolBuilder::new()
        .num_threads(num_cpus::get())
        .build_global()
        .unwrap();
    panic::set_hook(Box::new(|_| {
        eprintln!("download failed...");
        remove_dir_all("ruget_tmp_dir").unwrap();
    }));

    let args: Vec<String> = env::args().collect();
    let url = &args[1];
    let content_length = get_content_length(url);

    if content_length < 1000000 {
        let downloader = SingleDownloader {
            url: url.to_owned(),
        };
        downloader.download();
    } else {
        let downloader = ParallelDownloader {
            url: url.to_owned(),
        };
        downloader.download();
    }
}
