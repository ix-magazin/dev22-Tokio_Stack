////////////////////////////////////////////////////////
// Listing 1: Das Tokio-„Hello World” //
////////////////////////////////////////////////////////

use tokio::{
    io::{self, AsyncWriteExt},
    net::TcpListener,
};

#[tokio::main] // 1
async fn main() -> io::Result<()> {
    let listener = TcpListener::bind("localhost:8001").await?; // 2

    loop {
        let (mut socket, addr) = listener.accept().await?;
        println!("Listening to {}", addr);

        tokio::spawn(async move { socket.write_all("Hello World!\n\r".as_bytes()).await }); // 3
    }
}


////////////////////////////////////////////////////////////////////////////////////
// Listing 2: Der Aufbau der asynchronen Tokio-Runtime //
////////////////////////////////////////////////////////////////////////////////////

fn main() {
    let mut rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        // Inhalt der main-Methode
    })
}


////////////////////////////////////////////////////////////////////////////////////////////////////////
// Listing 3: Strukturen ohne Send-bound können Fehler verursachen //
///////////////////////////////////////////////////////////////////////////////////////////////////////

use tokio::task::yield_now;
use std::rc::Rc;

#[tokio::main]
async fn main() {
    tokio::spawn(async {
        let rc = Rc::new("hello");

        yield_now().await;

        println!("{}", rc); // ERROR
    });
}

///////////////////////////////////////////////////////////////////////////////////
// Listing 4: Der richtige Umgang mit Rc in Tokio Tasks //
///////////////////////////////////////////////////////////////////////////////////

use tokio::task::yield_now;
use std::rc::Rc;

#[tokio::main]
async fn main() {
    tokio::spawn(async {
        {
            let rc = Rc::new("hello");
            println!("{}", rc);
        }

        yield_now().await;
    });
}


//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Listing 5: Multiple Producer, Single Consumer (MPSC) Channels aus der Tokio Runtime //
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

async fn main() -> io::Result<()> {
    let mut socket = TcpStream::connect("www.example.com:1234").await?;
    let (tx, mut rx) = mpsc::channel(100); // 1

    for _ in 0..10 {
       let tx = tx.clone(); // 2

        tokio::spawn(async move { // 3
            tx.send(&b"data to write"[..]).await.unwrap();
        });
    }

    drop(tx); // 4

    while let Some(res) = rx.recv().await { // 5
        socket.write_all(res).await?;
    }

    Ok(())
}


///////////////////////////////////////////////////////////////////
// Listing 6: Tokio select! mit einem Timeout //
//////////////////////////////////////////////////////////////////

let mut stream = stream::iter(vec![1, 2, 3]);
let mut delay = time::delay_for(Duration::from_secs(1));

loop {
    tokio::select! {
        maybe_v = stream.next() => {
            if let Some(v) = maybe_v {
                println!("got = {}", v);
            } else {
                break;
            }
        }
        _ = &mut delay => {
            println!("timeout");
            break;
        }
    }
}
