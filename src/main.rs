use bitbuffer::{BitRead, BitWrite, BitWriteStream, LittleEndian};
use std::env::args;
use std::fs;
use tf_demo_parser::demo::header::Header;
use tf_demo_parser::demo::packet::consolecmd::ConsoleCmdPacket;
use tf_demo_parser::demo::packet::{Packet, PacketType};
use tf_demo_parser::demo::parser::{DemoHandler, Encode, RawPacketStream};
use tf_demo_parser::Demo;

fn main() {
    let mut args = args();
    let bin = args.next().unwrap();
    let path = match args.next() {
        Some(file) => file,
        None => {
            println!("usage: {} <demo> <tick>", bin);
            return;
        }
    };
    let tick = match args.next() {
        Some(tick) => tick.parse().unwrap(),
        None => {
            println!("usage: {} <demo> <tick>", bin);
            return;
        }
    };
    let file = fs::read(&path).unwrap();
    let out_path = format!("{}_bobstop.dem", path.trim_end_matches(".dem"));

    let stripped = mutate(&file, tick);
    fs::write(out_path, stripped).unwrap();
}

fn bob_packet(tick: u32, cycle: f32) -> Packet<'static> {
    Packet::ConsoleCmd(ConsoleCmdPacket {
        tick,
        command: format!("cl_bobcycle {}", cycle),
    })
}

#[derive(PartialEq, Debug)]
enum State {
    SignOn,
    Stopped,
    Started,
}

fn mutate(input: &[u8], bob_start: u32) -> Vec<u8> {
    let mut out_buffer = Vec::with_capacity(input.len());
    {
        let mut state = State::SignOn;
        let mut out_stream = BitWriteStream::new(&mut out_buffer, LittleEndian);

        let demo = Demo::new(&input);
        let mut stream = demo.get_stream();
        let header = Header::read(&mut stream).unwrap();
        header.write(&mut out_stream).unwrap();

        let mut packets = RawPacketStream::new(stream.clone());
        let mut handler = DemoHandler::default();
        handler.handle_header(&header);

        while let Some(packet) = packets.next(&handler.state_handler).unwrap() {
            if state == State::SignOn && packet.packet_type() == PacketType::ConsoleCmd {
                state = State::Stopped;
                bob_packet(packet.tick(), 99999999.0)
                    .encode(&mut out_stream, &handler.state_handler)
                    .unwrap();
            }

            if state == State::Stopped && packet.tick() > bob_start {
                state = State::Started;
                bob_packet(packet.tick(), 0.8)
                    .encode(&mut out_stream, &handler.state_handler)
                    .unwrap();
            }

            if packet.packet_type() != PacketType::ConsoleCmd {
                packet
                    .encode(&mut out_stream, &handler.state_handler)
                    .unwrap();
            }
            handler.handle_packet(packet).unwrap();
        }
    }
    out_buffer
}
