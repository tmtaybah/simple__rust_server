extern crate askama;
extern crate simple_webserver;

use std::io::prelude::*;
use std::thread;
use std::fs::File;
use std::net::TcpStream;
use std::net::TcpListener;
use std::time::Duration;
use askama::Template;
use simple_webserver::ThreadPool;


// TODO:
// add configuration
// add multithreading
// push to github

#[derive(Template)]
#[template(path = "hello.html")]

struct HelloTemplate<'a>{
    name: &'a str,
}


fn main() {

    let addr = "127.0.0.1:9998";
    let listener = TcpListener::bind(addr).unwrap();
    let pool = ThreadPool::new(4);

    let mut counter = 0;

    println!("Listening on [{}] ...", addr);

    // accept connections on a tcp listener
    for stream in listener.incoming(){
        counter = counter + 1;

        let stream = stream.unwrap();

        pool.execute(||{
            handle_connection(stream, counter);
        });
    }
}

fn handle_connection(mut stream: TcpStream, counter: i32){

    //  rendering template using askama
    // let hello = HelloTemplate {name: "world"};
    // let response2 = format!("{}", hello.render().unwrap());


    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let test = b"GET /test HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";


    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else if buffer.starts_with(test) {
        ("HTTP/1.1 200 OK\r\n\r\n", "test.html")
    } else if buffer.starts_with(sleep){
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };

    let mut file = File::open(filename).unwrap();

    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    // let visitor_para = "<p>{} visitros have been to this page so far .. </p>", counter;

    let response = format!("{}{}{}", status_line, contents, counter);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();

}
