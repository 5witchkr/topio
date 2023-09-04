use tokio::{net::{TcpStream, tcp::{OwnedReadHalf, OwnedWriteHalf}}, io::{Interest, Ready}};



struct Topio {
    pool: dyn Connection
}

impl Topio {

}

#[async_trait::async_trait]
trait Connection {
    
}

struct AsyncConnection<'a> {
    target_addr: &'a str,
    tcp_ready: Ready,
    tcp_reader: OwnedReadHalf,
    tcp_writer: OwnedWriteHalf
}

struct BlockingConnection {

}

fn blocking_connect() -> BlockingConnection {

    BlockingConnection {  }
}

async fn connect(target_addr: &str) -> AsyncConnection {
    let mut tcp_stream: TcpStream = match TcpStream::connect(target_addr).await {
        Ok(ok) => { ok },
        Err(_) => { panic!("connect fail") },
    };
    let tcp_ready: Ready = match tcp_stream.ready(Interest::READABLE | Interest::WRITABLE).await {
        Ok(ok) => { ok },
        Err(_) => { panic!("connection ready fail")},
    };
    let (tcp_reader, tcp_writer) = tcp_stream.into_split();

    AsyncConnection {target_addr, tcp_ready, tcp_reader, tcp_writer}
}

fn run_connection(connection: AsyncConnection) {

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
            let result_addr1: String = format!("{}", connection.tcp_reader.peer_addr().unwrap());
            let result_addr2: String = format!("{}", connection.tcp_writer.peer_addr().unwrap());
            assert_eq!(case.input_addr, result_addr1);
            assert_eq!(case.input_addr, result_addr2);
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
