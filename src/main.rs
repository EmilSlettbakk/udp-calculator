mod client;

use std::net::UdpSocket;
use std::str;

fn main() -> std::io::Result<()> {
    let socket = UdpSocket::bind("127.0.0.1:34254")?;
    println!("Server kjører på 127.0.0.1:34254");

    loop {
        let mut buf = [0; 1024];
        let (amt, src) = socket.recv_from(&mut buf)?;

        let buf = &mut buf[..amt];
        let received = str::from_utf8(buf).expect("Kunne ikke dekode meldingen");
        let parts: Vec<&str> = received.split_whitespace().collect();

        if parts.len() != 3 {
            println!("Ugyldig forespørsel: {}", received);
            continue;
        }

        let num1: f32 = parts[0].parse().expect("Feil ved parsing av første tall");
        let num2: f32 = parts[1].parse().expect("Feil ved parsing av andre tall");
        let operation = parts[2];

        let result = match operation {
            "+" => num1 + num2,
            "-" => num1 - num2,
            "*" => num1 * num2,
            "/" => num1 / num2,
            _ => {
                println!("Ukjent operasjon: {}", operation);
                continue;
            }
        };

        let response = format!("Resultat: {}", result);
        socket.send_to(response.as_bytes(), &src)?;
    }
}
