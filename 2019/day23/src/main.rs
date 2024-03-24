use std::io::{self, Read};

use intcode::IntcodeComputer;

#[derive(Clone, Copy)]
struct Packet {
    x: i64,
    y: i64,
}

impl Packet {
    fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }
}

// Create each computer and configures its address.
fn build_network(computer: &IntcodeComputer) -> Vec<IntcodeComputer> {
    (0..50)
        .map(|a| {
            let mut computer = computer.clone();
            computer.io.add_input(a);
            computer
        })
        .collect()
}

// Executes the computer code and returns its output, if any.
fn exec(computer: &mut IntcodeComputer) -> Option<(usize, Packet)> {
    computer.exec();
    if let Some(dest_addr) = computer.io.get_output() {
        let x = computer.io.get_output().unwrap();
        let y = computer.io.get_output().unwrap();
        Some((usize::try_from(dest_addr).unwrap(), Packet::new(x, y)))
    } else {
        None
    }
}

fn send_packet(network: &mut [IntcodeComputer], dest_addr: usize, packet: &Packet) {
    let dest = &mut network[dest_addr];
    dest.io.add_input(packet.x);
    dest.io.add_input(packet.y);
}

// Runs the network.
fn run<const AS_NAT: bool>(computer: &IntcodeComputer) -> i64 {
    let mut network = build_network(computer);

    let mut last_delivered_y = i64::MAX;
    let mut last_nat_received_packet = Packet::new(0, 0);

    loop {
        let mut idle = true;
        for a in 0..50 {
            let c = &mut network[a];

            // This simulates the computer having received no packet.
            c.io.add_input(-1);

            if let Some((dest_addr, packet)) = exec(c) {
                if dest_addr == 255 {
                    if !AS_NAT {
                        // If not running as NAT, return first packet delivered to 255.
                        return packet.y;
                    }
                    last_nat_received_packet = packet;
                } else {
                    send_packet(&mut network, dest_addr, &packet);
                }

                idle = false;
            }
        }

        if idle {
            if last_delivered_y == last_nat_received_packet.y {
                assert!(AS_NAT);
                // When running as NAT, return the first y delivered by the NAT twice in a row.
                return last_delivered_y;
            }

            send_packet(&mut network, 0, &last_nat_received_packet);

            last_delivered_y = last_nat_received_packet.y;
        }
    }
}

fn first_packet_to_255_y_val(computer: &IntcodeComputer) -> i64 {
    run::<false>(computer)
}

fn monitor_nat(computer: &IntcodeComputer) -> i64 {
    run::<true>(computer)
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let computer = IntcodeComputer::build(&input);

    println!("Part 1: {}", first_packet_to_255_y_val(&computer));
    println!("Part 2: {}", monitor_nat(&computer));
}
