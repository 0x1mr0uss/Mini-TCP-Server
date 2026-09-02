use std::{env, io::Read ,net::{TcpListener, TcpStream}, thread};
fn claient_tcp(mut stream: TcpListener){
    let perr_addr = stream
        .peer_addr()
        .map_or_else(|_| "unkown".to_string(), |addr| addr.to_string());
    println!("handling connection from {perr_addr}");

    let mut buffer = [0: 1024];

    loop {
        match stream.read(&mut buffer)  {
            Ok(n) => {}
            Err(e) if e.kind() == io
        }
    }
}
fn main(){
    let addr = env::args()
    .nth(1)
    .unwrap_or_else(|| "127.0.0.1:9090".to_string());
    let listener = TcpListener::bind(&addr)
        .expect("Fialed to bind to address");
    println!("server is runing on {}",addr);

    for stream_result in listener.incoming(){
        match stream_result {
            Ok(stream) => {
                thread::spawn(move || {
                  claient_tcp(stream);  
                });
            }
            Err(e) => {
                eprintln!("Fialed to accept the cocnaction {}", e);
            }
        };
    }
}