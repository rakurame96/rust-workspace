use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

fn main() {
    // The bind function in this scenario works like the new function in that it will return a new TcpListener instance.
    // The function is called bind because, in networking, connecting to a port to listen to is known as “binding to a port".
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    //  The incoming method on TcpListener returns an iterator that gives us a sequence of streams
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

// Before introducing thread
// fn handle_connection(mut stream: TcpStream) {
//     let buf_reader = BufReader::new(&mut stream);

//     /* http request message */
//     // Method Request-URI HTTP-Version CRLF
//     // headers CRLF
//     // message-body
//     // let http_request: Vec<_> = buf_reader
//     //         .lines()
//     //         .map(|result| result.unwrap())
//     //         .take_while(|line|!line.is_empty())
//     //         .collect();
//     // println!("Request: {:#?}", http_request);

//     let request_line = buf_reader
//         .lines()
//         .next()
//         .unwrap()
//         .unwrap();

//     let (status_line, filename) =
//         if request_line == "GET / HTTP/1.1" {
//             ("HTTP/1.1 200 OK", "hello.html")
//         } else {
//             ("HTTP/1.1 404 NOT FOUND", "404.html")
//         };

//     let contents = fs::read_to_string(filename).unwrap();
//     let length = contents.len();

//     /* http response message */
//     // HTTP-Version Status-Code Reason-Phrase CRLF
//     // headers CRLF
//     // message-body
//     let response = format!(
//         "{status_line}\r\n\
//         Content-Length: {length}\r\n\r\n\
//         {contents}"
//     );

//     stream.write_all(response.as_bytes()).unwrap();
// }

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);

    /* http request message */
    // Method Request-URI HTTP-Version CRLF
    // headers CRLF
    // message-body

    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let (status_line, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "hello.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    };

    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    /* http response message */
    // HTTP-Version Status-Code Reason-Phrase CRLF
    // headers CRLF
    // message-body
    let response = format!(
        "{status_line}\r\n\
        Content-Length: {length}\r\n\r\n\
        {contents}"
    );

    stream.write_all(response.as_bytes()).unwrap();
}
