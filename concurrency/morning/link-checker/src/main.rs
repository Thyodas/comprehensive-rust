use reqwest::blocking::{get, Response};
use reqwest::Url;
use scraper::{Html, Selector};
use thiserror::Error;
use std::sync::{mpsc, Arc, Mutex};
use std::sync::mpsc::{Receiver, SyncSender};
use std::thread;


#[derive(Error, Debug)]
enum Error {
    #[error("request error: {0}")]
    ReqwestError(#[from] reqwest::Error),
}

fn extract_links(response: Response, sender: &mpsc::SyncSender<Url>) {
    let base_url = response.url().to_owned();
    let document = response.text().unwrap();
    let html = Html::parse_document(&document);
    let selector = Selector::parse("a").unwrap();

    for element in html.select(&selector) {
        if let Some(href) = element.value().attr("href") {
            let path = href.to_string() + "/";
            match base_url.join(path.as_str()) {
                Ok(url) => {
                    match sender.send(url) {
                        Err(e) => {
                            println!("Error {e}");
                            return
                        },
                        _ => {}
                    }
                },
                Err(err) => {
                    println!("On {base_url}: could not parse {href:?}: {err} (ignored)",);
                }
            }
        }
    }
}

fn thread_logic(nb_requests: Arc<Mutex<u64>>, sender: SyncSender<Url>,
                receiver: Arc<Mutex<Receiver<Url>>>) {
    loop {
        let url = receiver.lock().unwrap().recv().unwrap();
        if *nb_requests.lock().unwrap() >= 100 {
            break;
        }
        let response = match get(url) {
            Err(u) => {
                println!("Invalid url: {u:?}");
                continue;
            }
            Ok(value) => value,
        };
        println!("Valid url: {response:?}");
        *nb_requests.lock().unwrap() += 1;
        extract_links(response, &sender);
    }
}

fn main() {
    let nb_requests = Arc::new(Mutex::new(0));
    let (sender, receiver) = mpsc::sync_channel(20);
    let safe_receiver = Arc::new(Mutex::new(receiver));

    let start_url = Url::parse("https://guillaume-hein.fr").unwrap();
    sender.send(start_url).unwrap();

    let nb_threads = thread::available_parallelism().unwrap().get();
    let mut handles = Vec::with_capacity(nb_threads);
    for _i in 0..nb_threads {
        let nb_requests_clone = nb_requests.clone();
        let sender_clone = sender.clone();
        let safe_receiver_clone = safe_receiver.clone();
        handles.push(thread::spawn(move || {
            thread_logic(nb_requests_clone, sender_clone,
                         safe_receiver_clone);
        }));
    }
    for handle in handles {
        handle.join().unwrap();
    }
}
