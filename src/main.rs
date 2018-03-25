extern crate reqwest;
extern crate hyper;

use std::env;
use std::thread;
use std::io::Read;
use std::sync::{Mutex};
use std::sync::mpsc::{channel, Sender, Receiver};

use hyper::server::{Handler, Server, Request, Response};

mod echo;

struct Inbox{
    inbox: Mutex<Sender<String>>
}

impl Handler for Inbox {
    fn handle(&self, mut req: Request, res: Response) {
        let mut msg = String::new();
        req.read_to_string(&mut msg).unwrap();
        println!("{}: {}", req.remote_addr, msg);
        self.inbox.lock().unwrap().send(format!("{} \n\n{}", msg, req.remote_addr)).unwrap();
        res.send(b"OK\n").unwrap();
    }
}

fn inbox(tx: Sender<String>) {
    let port = env::var("PORT").unwrap_or("12345".to_string());
    Server::http(format!("0.0.0.0:{}", port)).unwrap().handle(Inbox {
        inbox: Mutex::new(tx)
    }).unwrap();

}

struct Chat {
    id: String,
    token: String,
}

impl Chat {
    pub fn new(id: String, token: String) -> Chat {
        Chat {
            id,
            token
        }
    }

    pub fn send_message(&self, text: String) {
        let params = [("chat_id", &self.id), ("text", &text)];
        let client = reqwest::Client::new();
        let _res = client.post(&*format!("https://api.telegram.org/bot{}/sendMessage", self.token))
            .form(&params)
            .send();
    }
}

fn bot(rx: Receiver<String>) {
    let token = env::var("TELEGRAM_BOT_TOKEN").expect("missing TELEGRAM_BOT_TOKEN");
    let myid = env::var("TELEGRAM_CHAT_ID").expect("missing TELEGRAM_CHAT_ID");
    let chat = Chat::new(myid, token);
    loop {
        let msg = rx.recv().unwrap();
        chat.send_message(msg);
    }
}

fn main() {
    let (tx, rx) = channel();

    let child1 = thread::spawn(move || {
        inbox(tx);
    });
    println!("started inbox");

    let child2 = thread::spawn(move || {
        bot(rx);
    });
    println!("started redirect bot");

    let child3 = thread::spawn(move || {
        echo::robot();
    });
    println!("started telegram bot");

    child1.join().unwrap();
    child2.join().unwrap();
    child3.join().unwrap();
}
