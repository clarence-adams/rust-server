use server::ThreadPool;
use std::{
    fs,
    // io::{prelude::*, BufReader},
    io::prelude::*,
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming().take(2) {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }

    println!("Shutting down.");
}

fn handle_connection(mut stream: TcpStream) {
    // let buf_reader = BufReader::new(&mut stream);
    // let request_line = buf_reader.lines().next().unwrap().unwrap();
    // let _http_request: Vec<_> = buf_reader
    //     .lines()
    //     .map(|result| result.unwrap())
    //     .take_while(|line| !line.is_empty())
    //     .collect();

    // let (status_line, filename) = match &request_line[..] {
    //     "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "pages/index.html"),
    //     "GET /sleep HTTP/1.1" => {
    //         thread::sleep(Duration::from_secs(5));
    //         ("HTTP/1.1 200 OK", "pages/index.html")
    //     }
    //     _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    // };

    // let contents = fs::read_to_string(filename).unwrap();
    // let length = contents.len();

    // let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    // stream.write_all(response.as_bytes()).unwrap();

    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK", "./pages/index.html")
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK", "./pages/index.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "./pages/404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();

    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );

    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
