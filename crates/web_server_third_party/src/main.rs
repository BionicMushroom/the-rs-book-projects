use rayon::{ThreadBuilder, ThreadPool, ThreadPoolBuilder};

use std::{
    fs,
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
    process::ExitCode,
    thread,
    time::Duration,
};

fn main() -> ExitCode {
    if let Err(e) = execute() {
        eprintln!("{e}");
        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    }
}

fn execute() -> Result<(), String> {
    let listener = TcpListener::bind("127.0.0.1:7878")
        .map_err(|e| format!("Could not bind the socket: {e}."))?;

    ThreadPoolBuilder::new()
        .num_threads(4)
        .build_scoped(ThreadBuilder::run, |pool| {
            listen_for_connections(&listener, pool);
            println!("Shutting down.");
        })
        .map_err(|e| format!("Could not build the thread pool: {e}."))?;

    Ok(())
}

fn listen_for_connections(listener: &TcpListener, pool: &ThreadPool) {
    for stream in listener.incoming().take(2) {
        match stream {
            Ok(stream) => {
                pool.spawn(|| {
                    if let Err(e) = handle_connection(stream) {
                        eprintln!("{e}");
                    }
                });
            }
            Err(e) => {
                eprintln!("Could not listen for connection: {e}.");
            }
        }
    }
}

fn handle_connection(mut stream: TcpStream) -> Result<(), String> {
    let timeout = Some(Duration::from_secs(5));

    stream
        .set_read_timeout(timeout)
        .map_err(|e| format!("Could not set the read timeout: {e}."))?;

    stream
        .set_write_timeout(timeout)
        .map_err(|e| format!("Could not set the write timeout: {e}."))?;

    let buf_reader = BufReader::new(&mut stream);

    let request_line = buf_reader
        .lines()
        .next()
        .ok_or("Request does not have a first line.")?
        .map_err(|e| format!("Could not read the first line of the request: {e}."))?;

    let (status_line, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "res/hello.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "res/hello.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "res/404.html"),
    };

    let contents = fs::read_to_string(filename)
        .map_err(|e| format!("Could not read file {filename}: {e}."))?;

    let length = contents.len();
    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream
        .write_all(response.as_bytes())
        .map_err(|e| format!("Could not write response: {e}."))?;

    Ok(())
}
