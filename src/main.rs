extern crate reqwest;
extern crate hyper;

use std::env;
use std::thread;
use std::io::Read;
use std::sync::{Mutex};
use std::sync::mpsc::{channel, Sender, Receiver};

use hyper::server::{Handler, Server, Request, Response};

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
    Server::http("0.0.0.0:12345").unwrap().handle(Inbox {
        inbox: Mutex::new(tx)
    }).unwrap();

}

struct Chat {
    id: u32,
    token: String,
}

impl Chat {
    pub fn new(id: u32, token: String) -> Chat {
        Chat {
            id,
            token
        }
    }

    pub fn send_message(&self, text: String) {
        let params = [("chat_id", &format!("{}", self.id)), ("text", &text)];
        let client = reqwest::Client::new();
        let res = client.post(&*format!("https://api.telegram.org/bot{}/sendMessage", self.token))
            .form(&params)
            .send();
    }
}

fn bot(rx: Receiver<String>) {
    loop {
        let msg = rx.recv().unwrap();
        let token = env::var("TELEGRAM_BOT_TOKEN").unwrap();
        let chat = Chat::new(142664361, token);
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
    println!("started bot");

    child1.join().unwrap();
    child2.join().unwrap();
}
