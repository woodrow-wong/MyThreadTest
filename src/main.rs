use std::{fs, io::{BufRead, BufReader, Write}, net::{TcpListener, TcpStream}, thread, time::Duration};
use hello::ThreadPool;

fn main() {
    // println!("Hello, world!");
    let listener=TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool=ThreadPool::new(4);

    for stream in listener.incoming().take(2){
        let stream=stream.unwrap();

        // println!("Connection establised!");
        // handle_connection(stream);
        pool.execute(||{
            handle_connection(stream);
        })
    }
    println!("Shutting down.");
}

fn handle_connection(mut stream:TcpStream){
    let buf_reader=BufReader::new(&mut stream);
    // let http_request:Vec<_>=buf_reader
    //     .lines()
    //     .map(|result| result.unwrap())
    //     .take_while(|line| !line.is_empty())
    //     .collect();

    // // println!("Request: {:#?}",http_request);
    // // let response="HTTP/1.1 200 OK\r\n\r\n";
    // let status_line="HTTP/1.1 200 OK";
    // let contents=fs::read_to_string("hello.html").unwrap();
    // let length=contents.len();

    // let response=
    //     format!("{status_line}\r\nContent-Length:{length}\r\n\r\n\n{contents}");

    // stream.write_all(response.as_bytes()).unwrap();
    let request_line=buf_reader.lines().next().unwrap().unwrap();

    // if request_line=="GET / HTTP/1.1"{
    //     let status_line="HTTP/1.1 200 OK";
    //     let contents=fs::read_to_string("hello.html").unwrap();
    //     let length=contents.len();

    //     let response=
    //         format!("{status_line}\r\nContent-Length:{length}\r\n\r\n\n{contents}");

    //     stream.write_all(response.as_bytes()).unwrap();
    // }else {
    //     let status_line="HTTP/1.1 404 NOT FOUND";
    //     let contents=fs::read_to_string("404.html").unwrap();
    //     let length = contents.len();

    //     let response=format!(
    //         "{status_line}\r\nContent-Length: {length}\r\n\r\n\n{contents}"
    //     );
    //     stream.write_all(response.as_bytes()).unwrap();
    // }

    // let (status_line, filename) = if request_line == "GET / HTTP/1.1" {
    //     ("HTTP/1.1 200 OK", "hello.html")
    // } else {
    //     ("HTTP/1.1 404 NOT FOUND", "404.html")
    // };

    let (status_line, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "hello.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    };

    let contents: String = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    let response =
        format!("{status_line}\r\nContent-Length: {length}\r\n\r\n\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}