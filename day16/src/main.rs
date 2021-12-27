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
    Literal(u32),
    Operator(Vec<Packet>),
}

impl Packet {
    fn from(bits: Vec<u8>) -> Result<(Packet, Vec<u8>)> {
        //println!("FROM {:?}", bits);
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
        let payload = Payload::Literal(u32::from_str_radix(
            &acc.iter()
                .map(|b| b.to_string())
                .collect::<Vec<String>>()
                .join(""),
            2,
        )?);
        // Return the payload and unparsed bits
        Ok((payload, bits.iter().skip(idx).copied().collect::<Vec<_>>()))
    }

    fn operator_payload(bits: Vec<u8>) -> Result<(Payload, Vec<u8>)> {
        let mut packets: Vec<Packet> = Vec::new();
        let rem = match &bits[..] {
            // length type ID is 0, then the next 15 bits are a number that
            // represents the total length in bits of the sub-packets contained
            // by this packet.
            [0, rest @ ..] => {
                let len = usize::from_str_radix(
                    &rest
                        .iter()
                        .take(15)
                        .map(|b| b.to_string())
                        .collect::<Vec<_>>()
                        .join(""),
                    2,
                )?;
                let rest = rest.iter().skip(15).copied().collect::<Vec<_>>();
                let to_parse = rest.iter().take(len).collect::<Vec<_>>();
                let rest = rest.iter().skip(len).copied().collect::<Vec<_>>();

                //println!("To Parse {:?}", to_parse);
                let mut looper = to_parse.iter().map(|x| *(*x)).collect::<Vec<_>>();
                while let Ok((pkt, leftover)) = Packet::from(looper) {
                    //println!("Inner pkt {:?}", pkt);
                    packets.push(pkt);
                    //println!("Inner leftover {:?}", leftover);
                    looper = leftover.to_vec();
                }

                println!("Rest {:?}", rest);
                rest
            }
            // length type ID is 1, then the next 11 bits are a number that
            // represents the number of sub-packets immediately contained by this
            // packet.
            [1, rest @ ..] => {
                let len = u32::from_str_radix(
                    &rest
                        .iter()
                        .take(11)
                        .map(|b| b.to_string())
                        .collect::<Vec<_>>()
                        .join(""),
                    2,
                )?;
                let rest = rest.iter().skip(11).copied().collect::<Vec<_>>();
                let mut looper = rest.clone();
                for _ in 0..len {
                    //println!("TYPE1 Loop {}", i);
                    if let Ok((pkt, leftover)) = Packet::from(looper.clone()) {
                        //println!("Inner pkt {:?}", pkt);
                        packets.push(pkt);
                        //println!("Inner leftover {:?}", leftover);
                        looper = leftover.to_vec();
                    } else {
                        unreachable!("Not OK");
                    }
                }

                looper
            }
            _ => unreachable!(),
        };
        // Return the payload and unparsed bits
        Ok((Payload::Operator(packets), rem))
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

    println!("Binary {:?}", binary);
    let pkt = Packet::from(binary);
    println!("{:?}", pkt);
}
