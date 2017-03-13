extern crate clap;
use clap::App;

mod modes;
use modes::*;

mod config;
use config::*;

enum TransportProtocol {
    Tcp,
    Udp,
}

enum NcMode {
    Connect,
    Listen,
}

fn main() {

    let matches = App::new("nc")
                          .version("0.1")
                          .author("nobody <nobody@gmail.com>")
                          .about("Concatenate and redirect sockets")
                          .args_from_usage(
                              "-l, --listen 'Turn on listening mode'
                              -u, --udp     'Use UDP on transport layer'
                              <hostname>    'target hostname or ip address'
                              <port>        'specify port'")
                          .get_matches();

    let hostname = matches.value_of("hostname").unwrap();
    let port = matches.value_of("port").unwrap();
    let proto = if matches.is_present("udp") {
        TransportProtocol::Udp
    } else {
        TransportProtocol::Tcp
    };
    let mode = if matches.is_present("listen") {
        NcMode::Listen
    } else {
        NcMode::Connect
    };

    let hostname = format!("{}:{}", hostname, port);

    match (mode, proto) {
        (NcMode::Connect, TransportProtocol::Tcp) => {
            connect_tcp(&hostname).unwrap_or_else(|e| {
                println!("nc error: {}", e);
            });
        }
        (NcMode::Listen, TransportProtocol::Tcp) => {
            listen_tcp(&hostname).unwrap_or_else(|e| {
                println!("nc error: {}", e);
            });
        }
        (NcMode::Connect, TransportProtocol::Udp) => {
            connect_udp(&hostname).unwrap_or_else(|e| {
                println!("nc error: {}", e);
            });
        }
        (NcMode::Listen, TransportProtocol::Udp) => {
            listen_udp(&hostname).unwrap_or_else(|e| {
                println!("nc error: {}", e);
            });
        }
    }

}
