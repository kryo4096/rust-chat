extern crate ws;

use ws::{Handler, Sender, Message, Result, Handshake, CloseCode, Request, Response};
use std::fs::File;
use std::io::Read;
use std::env::args;

struct Server {
    out: Sender, 
    remote_addr: String,
    local_addr: String,
}

impl Handler for Server {

    fn on_request(&mut self, req: &Request) -> Result<Response> {
        match req.resource() {
            "/" => {

                let mut buf = String::new();

                File::open("site/client.html").unwrap().read_to_string(&mut buf).unwrap();

                buf = buf.replace("<IP>",self.local_addr.as_str());
                
                Ok(Response::new(200, "Ok", buf.as_bytes().into()))
            },
            "/ws" => Response::from_request(req),
            _ => Ok(Response::new(404, "Not Found", "404 - Not Found".into())),
        }
    }

    fn on_message(&mut self, msg: Message) -> Result<()>{
        self.out.broadcast(format!("[{}] {}<br>",self.remote_addr, msg))
    }

    fn on_open(&mut self, shake: Handshake) -> Result<()>{
        self.remote_addr = shake.remote_addr()?.unwrap();
        self.out.broadcast(format!("<span style=\"color: green\">{} has connected</span><br>",self.remote_addr))
    }
    
    fn on_close(&mut self, _: CloseCode, _: &str){
        self.out.broadcast(format!("<span style=\"color: red\">{} has disconnected</span><br>",self.remote_addr));
    }
}

fn main() {

    let ip = args().nth(1).expect("No IP specified");

    
    println!("Listening on {}", &ip);
    ws::listen(&ip,|out| Server {out, remote_addr: "".into(), local_addr: ip.clone()}).unwrap();

    

}
