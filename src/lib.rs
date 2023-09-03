use tokio::{net::{TcpStream, TcpSocket}, io::{Interest, Ready}};



struct Topio<'a> {
    pool: Connection<'a>
}

struct Connection<'a> {
    target_addr: &'a str,
    tcp_ready: Ready,
    tcp_stream: TcpStream
}


impl Topio<'_> {

}

async fn connect(target_addr: &str) -> Connection {
    let tcp_stream: TcpStream = match TcpStream::connect(target_addr).await {
        Ok(ok) => { ok },
        Err(_) => { panic!("connect fail") },
    };
    let tcp_ready: Ready = match tcp_stream.ready(Interest::READABLE | Interest::WRITABLE).await {
        Ok(ok) => { ok },
        Err(_) => { panic!("connection ready fail")},
    };
    Connection {target_addr, tcp_ready, tcp_stream}
}

fn run_connection(connection: Connection) {

}

#[cfg(test)]
mod tests {
    use tokio::net::TcpListener;

    use super::*;

    #[tokio::test]
    async fn test_tcp_connect() {
        struct TestCase<'a> {
            input_addr: &'a str
        }
        let test_cases = vec![
            TestCase { input_addr: "127.0.0.1:32122" }
        ];
        for case in test_cases {
            tokio::task::spawn(run_test_listender(case.input_addr));
            tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;

            let connection: Connection<'_> = connect(case.input_addr).await;
            let result_addr: String = format!("{}", connection.tcp_stream.peer_addr().unwrap());
            println!("{:?}", connection.tcp_ready);
            assert_eq!(case.input_addr, result_addr);
            assert_eq!(false, connection.tcp_ready.is_read_closed());
            assert_eq!(false, connection.tcp_ready.is_error());
            assert_eq!(true, connection.tcp_ready.is_writable());
        }
    }

    async fn run_test_listender(addr: &str) {
        let listender = TcpListener::bind(addr).await.unwrap();
        loop {
            listender.accept().await.unwrap();
        }
    }
}
