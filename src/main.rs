use structopt::StructOpt;
use std::thread;
use std::net::{TcpListener, TcpStream, Shutdown};
use std::fmt;
use std::io::{Read, Write};
use std::fmt::Display;

struct Msg {
    content: String,
    size: usize,
}

impl Display for Msg {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Received \n size : {} \n content : {}", self.size, self.content)
    }
}

fn handle_client(mut stream: TcpStream) {
    let mut data = [0 as u8; 50];
    while match stream.read(&mut data){
        Ok(size) => {
            let tmp =  String::from_utf8_lossy(&data[0..size]);
            let msg = Msg{
                content: tmp.to_owned().parse().unwrap(),
                size:size,
            };
            println!("{}", msg);
            if &*tmp == "exit\r\n"{
                false
            } else {
                stream.write(&data[0..size]).unwrap();
                true
            }
        },
        Err(_) => {
            println!("An error occurred, terminating connection with {}", stream.peer_addr().unwrap());
            stream.shutdown(Shutdown::Both).unwrap();
            false
        }
    }{}
}


fn echo_server_run(port:String){
    let listener = TcpListener::bind("0.0.0.0:".to_owned() + &port).unwrap();
    println!("Server listening on port {}", port);
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                thread::spawn(move|| {
                    handle_client(stream)
                });
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
    // 关闭 tcp 服务
    drop(listener);
}

#[derive(StructOpt, Debug)]
#[structopt(name = "lvtoo")]
#[structopt(version = "0.2.0")]
enum Lvtoo {
    #[structopt(name = "sort", help = "sort N numbers",)]
    Sort {
        numbers: Vec<i64>
    },
    #[structopt(name = "echo", help = "echo server",)]
    Echo {
        port:String
    },
}

fn main() {
    // let port = "8081";
    // echo_server_run(port.to_owned());
    let matches = Lvtoo::clap().get_matches();
    if let Some(sort) = matches.subcommand_matches("sort") {
        if let Some(numbers) = sort.values_of("numbers") {
            let mut vec:Vec<i64> = vec![];
            for i in numbers {
                vec.push(i.parse::<i64>().unwrap());
            }
            vec.sort();
            println!("{:#?}", vec)
        }
    }else if let Some(sort) = matches.subcommand_matches("echo")  {
        if let Some(port) = sort.values_of("port") {
            for i in port {
                echo_server_run(i.to_owned());
                break;
            }
        }
    }
}