use std::fs;
use bitreader::BitReader;
use std::str::from_utf8;

#[derive(PartialEq,Debug)]
struct PacketHeader{
    version : u8,
    type_id : u8,
}

impl PacketHeader {
    fn parse(i:&mut BitReader)->Self{
        Self{version:i.read_u8(3).unwrap(),type_id:i.read_u8(3).unwrap()}
    }
}
#[derive(PartialEq,Debug)]
struct Packet{
    header :PacketHeader,
    data :PacketType,
}


#[derive(PartialEq,Debug)]
enum PacketType {
    Value(u64),
    Operator(Vec<Packet>)
}

impl Packet {
    fn parse(i:&mut BitReader)->Self{
        let header= PacketHeader::parse(i);
        let data:PacketType;
        if header.type_id==4 {
            let mut repeat=true;
            let mut value_byts=vec![];
            while repeat {
                repeat= i.read_bool().unwrap();
                value_byts.push(i.read_u8(4).unwrap());
            }
            let value=value_byts.iter().rev().enumerate().fold(0u64, |mut v,(i,x)| {v|=(*x as u64)<<4*i;v} );
            data=PacketType::Value(value);
        }
        else{
            if i.read_bool().unwrap() {
                let num = i.read_u16(11).unwrap();
                let mut r: Vec<Packet>=vec![];
                for _ in 0..num {
                    r.push(Packet::parse(i));
                }
                data=PacketType::Operator(r)
            }
            else {
                let len = i.read_u16(15).unwrap();
                let start = i.position();
                let mut r: Vec<Packet>=vec![];
                while (i.position()-start)<len as u64 {
                    r.push(Packet::parse(i));
                }
                data=PacketType::Operator(r)
            }
        }
        Self{header,data}
    }

    fn version_sum(&self)->u32{
        match &self.data {
            PacketType::Value(_)=>self.header.version as u32,
            PacketType::Operator(x)=>x.iter().map(|a|a.version_sum()).sum::<u32>()+self.header.version as u32
        }
    }

    fn eval(&self) -> u64{
        match  &self.data {
            PacketType::Operator(x)=>{
                match self.header.type_id {
                    0=>x.iter().map(|a|a.eval()).sum(),
                    1=>x.iter().map(|a|a.eval()).product(),
                    2=>x.iter().map(|a|a.eval()).min().unwrap(),
                    3=>x.iter().map(|a|a.eval()).max().unwrap(),
                    5=> if x[0].eval()>x[1].eval() {1} else{0},
                    6=> if x[0].eval()<x[1].eval() {1} else{0},
                    7=> if x[0].eval()==x[1].eval() {1} else{0},
                    _=>panic!("unknown type")
                }
            },
            PacketType::Value(x)=>*x,
        }
        
    }
}


pub fn solve(){
    let input = fs::read_to_string("data/day16.dat").expect("file day16 fehlt");
    let bytes: Vec<u8> = input.lines().next().unwrap().as_bytes().chunks(2).map(|x|from_utf8(x).unwrap()).collect::<Vec<&str>>().iter().map(|x|u8::from_str_radix(x, 16).unwrap()).collect();
    let mut reader = BitReader::new(&bytes);
    let packets= Packet::parse(&mut reader);
    println!("version sum {}",packets.version_sum());

    println!("eval {}",packets.eval());
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parse_packet_header() {
        let header =  PacketHeader::parse(&mut BitReader::new(&[0xD2u8]));
        assert_eq!(header.version,6);
        assert_eq!(header.type_id,4);
    }
    #[test]
    fn parse_value() {
        let packet =  Packet::parse(&mut BitReader::new(&[0xD2u8,0xFE,0x28]));
        assert_eq!(packet.data,PacketType::Value(2021));
    }
    #[test]
    fn test1(){
        let packet =  Packet::parse(&mut BitReader::new(&[0x8A,0x00,0x4A,0x80,0x1A,0x80,0x02,0xF4,0x78]));
        assert_eq!(packet.version_sum(),16);
    }
    #[test]
    fn test2(){
        let packet =  Packet::parse(&mut BitReader::new(&[0x62,0x00,0x80,0x00,0x16,0x11,0x56,0x2C,0x88,0x02,0x11,0x8E,0x34]));
        assert_eq!(packet.version_sum(),12);
    }
    #[test]
    fn test3(){
        let packet =  Packet::parse(&mut BitReader::new(&[0xC0,0x01,0x50,0x00,0x01,0x61,0x15,0xA2,0xE0,0x80,0x2F,0x18,0x23,0x40]));
        assert_eq!(packet.version_sum(),23);
    }
    #[test]
    fn test4(){
        let packet =  Packet::parse(&mut BitReader::new(&[0xA0,0x01,0x6C,0x88,0x01,0x62,0x01,0x7C,0x36,0x86,0xB1,0x8A,0x3D,0x47,0x80]));
        assert_eq!(packet.version_sum(),31);
    }
}