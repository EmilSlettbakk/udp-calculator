use std::net::UdpSocket;
use std::io;

fn main() -> io::Result<()> {
    let socket = UdpSocket::bind("0.0.0.0:0")?;
    socket.connect("127.0.0.1:34254")?;

    loop {
        println!("Koble til tjener på 127.0.0.1:34254");
        println!("Skriv inn beregning (format: <tall> <operasjon> <tall>), f.eks. '3 + 4', eller 'exit' for å avslutte:");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Feil ved lesing av input");

        if input.trim().eq_ignore_ascii_case("exit") {
            println!("Avslutter klient...");
            break;
        }

        socket.send(input.as_bytes())?;

        let mut buf = [0; 1024];
        let (amt, _) = socket.recv_from(&mut buf)?;
        let buf = &buf[..amt];
        let received = std::str::from_utf8(buf).expect("Kunne ikke dekode meldingen");

        println!("Mottok: {}", received);
    }

    Ok(())
}
