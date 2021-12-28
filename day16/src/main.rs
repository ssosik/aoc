use anyhow::Result;
use phf::phf_map;
use std::io::{BufRead, BufReader};

static HEX: phf::Map<char, [u8; 4]> = phf_map! {
    '0' => [0,0,0,0],
    '1' => [0,0,0,1],
    '2' => [0,0,1,0],
    '3' => [0,0,1,1],
    '4' => [0,1,0,0],
    '5' => [0,1,0,1],
    '6' => [0,1,1,0],
    '7' => [0,1,1,1],
    '8' => [1,0,0,0],
    '9' => [1,0,0,1],
    'A' => [1,0,1,0],
    'B' => [1,0,1,1],
    'C' => [1,1,0,0],
    'D' => [1,1,0,1],
    'E' => [1,1,1,0],
    'F' => [1,1,1,1],
};

#[allow(dead_code)]
#[derive(Debug)]
struct Packet {
    version: u8,
    type_id: u8,
    payload: Payload,
}

#[derive(Debug)]
enum Payload {
    Literal(usize),
    Operator(Vec<Packet>),
}

impl Packet {
    fn from(bits: Vec<u8>) -> Result<(Packet, Vec<u8>)> {
        let version = u8::from_str_radix(
            &bits
                .iter()
                .take(3)
                .map(|b| b.to_string())
                .collect::<Vec<_>>()
                .join(""),
            2,
        )?;
        let type_id = u8::from_str_radix(
            &bits
                .iter()
                .skip(3)
                .take(3)
                .map(|b| b.to_string())
                .collect::<Vec<_>>()
                .join(""),
            2,
        )?;
        let rest: Vec<_> = bits.iter().skip(6).copied().collect();
        let (payload, rest) = match type_id {
            // Literal type packet
            4 => Packet::literal_payload(rest)?,
            // Operator type packet
            _ => Packet::operator_payload(rest)?,
        };

        // Return the packet and unparsed bits
        Ok((
            Packet {
                version,
                type_id,
                payload,
            },
            rest,
        ))
    }

    fn literal_payload(bits: Vec<u8>) -> Result<(Payload, Vec<u8>)> {
        let mut acc: Vec<&u8> = Vec::new();
        let mut idx = 0;
        for sl in bits.chunks(5) {
            idx += 5;
            match sl {
                [0, bits @ ..] => {
                    acc.extend(bits);
                    break;
                }
                [1, bits @ ..] => acc.extend(bits),
                _ => unreachable!(),
            }
        }
        let payload = Payload::Literal(usize::from_str_radix(
            &acc.iter()
                .map(|b| b.to_string())
                .collect::<Vec<String>>()
                .join(""),
            2,
        )?);
        // Return the payload and unparsed bits
        Ok((payload, bits.iter().skip(idx).copied().collect::<Vec<_>>()))
    }

    fn split_bits(bits: &[u8], idx: usize) -> Result<(Vec<u8>, usize)> {
        let mut bits = bits.to_vec();
        Ok((
            bits.split_off(idx),
            usize::from_str_radix(
                &bits
                    .iter()
                    .map(|b| b.to_string())
                    .collect::<Vec<_>>()
                    .join(""),
                2,
            )?,
        ))
    }

    fn operator_payload(bits: Vec<u8>) -> Result<(Payload, Vec<u8>)> {
        let mut packets: Vec<Packet> = Vec::new();
        let rem = match &bits[..] {
            // length type ID is 0, then the next 15 bits are a number that
            // represents the total length in bits of the sub-packets contained
            // by this packet.
            [0, bits @ ..] => {
                let (mut pkt_bits, len) = Packet::split_bits(bits, 15)?;
                let leftover = pkt_bits.split_off(len);
                while let Ok((pkt, rem)) = Packet::from(pkt_bits) {
                    packets.push(pkt);
                    pkt_bits = rem.to_vec();
                }
                leftover
            }
            // length type ID is 1, then the next 11 bits are a number that
            // represents the number of sub-packets immediately contained by this
            // packet.
            [1, bits @ ..] => {
                let (mut pkt_bits, pkt_cnt) = Packet::split_bits(bits, 11)?;
                for _ in 1..=pkt_cnt {
                    if let Ok((pkt, leftover)) = Packet::from(pkt_bits) {
                        packets.push(pkt);
                        pkt_bits = leftover.to_vec();
                    } else {
                        unreachable!("Not OK Packet");
                    }
                }
                pkt_bits
            }
            _ => unreachable!(),
        };
        // Return the payload and unparsed bits
        Ok((Payload::Operator(packets), rem))
    }

    fn sum_versions(&self) -> usize {
        match &self.payload {
            Payload::Literal(_) => self.version as usize,
            Payload::Operator(pkts) => {
                pkts.iter().map(|p| p.sum_versions()).sum::<usize>() + self.version as usize
            }
        }
    }

    fn value(&self) -> usize {
        match &self.payload {
            Payload::Literal(val) => *val,
            Payload::Operator(pkts) => match self.type_id {
                0 => pkts.iter().map(|p| p.value()).sum::<usize>(),
                1 => pkts.iter().map(|p| p.value()).product::<usize>(),
                2 => pkts.iter().map(|p| p.value()).min().unwrap(),
                3 => pkts.iter().map(|p| p.value()).max().unwrap(),
                5 => match pkts[0].value() > pkts[1].value() {
                    true => 1,
                    _ => 0,
                },
                6 => match pkts[0].value() < pkts[1].value() {
                    true => 1,
                    _ => 0,
                },
                7 => match pkts[0].value() == pkts[1].value() {
                    true => 1,
                    _ => 0,
                },
                x => unreachable!("Unsupported type id: {:?}", x),
            },
        }
    }
}

fn main() {
    // Read from Stdin
    let binary = BufReader::new(std::io::stdin())
        .lines()
        // Take only the one line
        .take(1)
        .inspect(|line| println!("Hex Input list {:?}", line))
        // Convert the line into a Vec of chars
        .map(|l| l.unwrap().chars().collect::<_>())
        // Convert each char into its binary representation of 4 bits, flattening
        // into one continuous list of bits
        .flat_map(|l: Vec<char>| {
            l.iter()
                // Lookup the char in the Hex map, returning the slice of bits
                .flat_map(|c| *HEX.get(c).unwrap())
                .collect::<Vec<u8>>()
        })
        .collect::<Vec<_>>();

    //println!("Binary {:?}", binary);
    let (pkt, _rest) = Packet::from(binary).unwrap();
    println!("{:?}", pkt);
    let sum = pkt.sum_versions();
    println!("Sum {}", sum);
    let val = pkt.value();
    println!("Val {}", val);
}
