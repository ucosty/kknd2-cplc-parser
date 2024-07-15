use std::error::Error;
use std::fs::File;
use std::io::{BufReader, Read, Seek, SeekFrom};
use byteorder::{LittleEndian, ReadBytesExt};
use crate::units::{CpuPlayerInformation, UnitKind};

#[derive(Debug)]
struct Header {
    file_size: u32,
    list_1: u32,
    list_2: u32,
    list_3: u32,
    list_4: u32,
}

#[derive(Debug)]
struct Unit {
    kind: u8,

    x: u32,
    y: u32,

    list_1: u32,
    list_2: u32,
    list_3: u32,
    list_4: u32,

    team: u16,
    flags: u16,
    unknown: u16,
    activation_timer: u16,
}

#[derive(Debug)]
struct ScrollStart {
    x: u32,
    y: u32,
    list_1: u32,
    list_2: u32,
    list_3: u32,
    list_4: u32,
}

#[derive(Debug)]
struct Ripple {
    x: u32,
    y: u32,
    list_1: u32,
    list_2: u32,
    list_3: u32,
    list_4: u32,
}



#[derive(Debug)]
struct MapConfiguration {
    x: u32,
    y: u32,
    list_1: u32,
    list_2: u32,
    list_3: u32,
    list_4: u32,
}

// 0x00 - Target Area
//

fn parse_header<R: Read>(reader: &mut BufReader<R>) -> Result<Header, Box<dyn Error>> {
    Ok(Header {
        file_size: reader.read_u32::<LittleEndian>()?,
        list_1: reader.read_u32::<LittleEndian>()?,
        list_2: reader.read_u32::<LittleEndian>()?,
        list_3: reader.read_u32::<LittleEndian>()?,
        list_4: reader.read_u32::<LittleEndian>()?,
    })
}

fn file_pointer_to_offset(file_offset: u32, pointer: u32) -> u64 {
    if pointer == 0 {
        return 0;
    }
    (pointer + 8 - file_offset) as u64
}

enum ItemParser {
    CpuPlayerInformation,
    MapConfiguration,
    ScrollStart,
    Ripple,
    Unit(u8),
}

impl From<u8> for ItemParser {
    fn from(value: u8) -> Self {
        match value {
            0x01 => ItemParser::CpuPlayerInformation,
            0x04 => ItemParser::MapConfiguration,
            0x09 => ItemParser::ScrollStart,
            0xa9 => ItemParser::Ripple,
            v => ItemParser::Unit(v),
        }
    }
}

fn parse_unit<R: Read + Seek>(reader: &mut BufReader<R>, kind: u8) -> Result<Unit, Box<dyn Error>> {
    reader.seek_relative(4)?;

    let x = reader.read_u32::<LittleEndian>()?;
    let y = reader.read_u32::<LittleEndian>()?;

    reader.seek_relative(3)?;

    let list_1 = reader.read_u32::<LittleEndian>()?;
    let list_2 = reader.read_u32::<LittleEndian>()?;
    let list_3 = reader.read_u32::<LittleEndian>()?;
    let list_4 = reader.read_u32::<LittleEndian>()?;

    reader.seek_relative(24)?;

    let team = reader.read_u16::<LittleEndian>()?;
    let flags = reader.read_u16::<LittleEndian>()?;
    let unknown = reader.read_u16::<LittleEndian>()?;
    let activation_timer = reader.read_u16::<LittleEndian>()?;

    Ok(Unit {
        kind,
        x,
        y,
        list_1,
        list_2,
        list_3,
        list_4,
        team,
        flags,
        unknown,
        activation_timer,
    })
}

fn parse_ripple<R: Read + Seek>(reader: &mut BufReader<R>) -> Result<Ripple, Box<dyn Error>> {
    reader.seek_relative(4)?;

    let x = reader.read_u32::<LittleEndian>()?;
    let y = reader.read_u32::<LittleEndian>()?;

    reader.seek_relative(3)?;

    let list_1 = reader.read_u32::<LittleEndian>()?;
    let list_2 = reader.read_u32::<LittleEndian>()?;
    let list_3 = reader.read_u32::<LittleEndian>()?;
    let list_4 = reader.read_u32::<LittleEndian>()?;

    Ok(Ripple {
        x,
        y,
        list_1,
        list_2,
        list_3,
        list_4,
    })
}

fn parse_scroll_start<R: Read + Seek>(
    reader: &mut BufReader<R>,
) -> Result<ScrollStart, Box<dyn Error>> {
    reader.seek_relative(4)?;

    let x = reader.read_u32::<LittleEndian>()?;
    let y = reader.read_u32::<LittleEndian>()?;

    reader.seek_relative(3)?;

    let list_1 = reader.read_u32::<LittleEndian>()?;
    let list_2 = reader.read_u32::<LittleEndian>()?;
    let list_3 = reader.read_u32::<LittleEndian>()?;
    let list_4 = reader.read_u32::<LittleEndian>()?;

    Ok(ScrollStart {
        x,
        y,
        list_1,
        list_2,
        list_3,
        list_4,
    })
}

fn parse_cpu_player_information<R: Read + Seek>(
    reader: &mut BufReader<R>,
) -> Result<CpuPlayerInformation, Box<dyn Error>> {
    reader.seek_relative(4)?;

    let x = reader.read_u32::<LittleEndian>()?;
    let y = reader.read_u32::<LittleEndian>()?;

    reader.seek_relative(3)?;

    let list_1 = reader.read_u32::<LittleEndian>()?;
    let list_2 = reader.read_u32::<LittleEndian>()?;
    let list_3 = reader.read_u32::<LittleEndian>()?;
    let list_4 = reader.read_u32::<LittleEndian>()?;

    reader.seek_relative(30)?;

    let default_unit_1 = reader.read_u16::<LittleEndian>()?;
    let default_unit_2 = reader.read_u16::<LittleEndian>()?;
    let default_unit_3 = reader.read_u16::<LittleEndian>()?;

    Ok(CpuPlayerInformation {
        x,
        y,
        list_1,
        list_2,
        list_3,
        list_4,
        default_units: [default_unit_1, default_unit_2, default_unit_3],
    })
}

fn parse_map_configuration<R: Read + Seek>(
    reader: &mut BufReader<R>,
) -> Result<MapConfiguration, Box<dyn Error>> {
    reader.seek_relative(4)?;

    let x = reader.read_u32::<LittleEndian>()?;
    let y = reader.read_u32::<LittleEndian>()?;

    reader.seek_relative(3)?;

    let list_1 = reader.read_u32::<LittleEndian>()?;
    let list_2 = reader.read_u32::<LittleEndian>()?;
    let list_3 = reader.read_u32::<LittleEndian>()?;
    let list_4 = reader.read_u32::<LittleEndian>()?;

    Ok(MapConfiguration {
        x,
        y,
        list_1,
        list_2,
        list_3,
        list_4,
    })
}

pub fn parse_cplc(filename: &str) -> Result<(), Box<dyn Error>> {
    let file = File::open(filename).map_err(|e| format!("Failed to open file: {}", e))?;

    let mut reader = BufReader::new(file);

    let magic = reader.read_u32::<LittleEndian>()?;
    if magic != 0xdeadc0de {
        return Err(format!("Invalid magic. Expected 0xdeadc0de, found {:#x}", magic).into());
    }

    let file_offset = reader.read_u32::<LittleEndian>()?;
    let header = parse_header(&mut reader)?;

    let mut next_offset = file_pointer_to_offset(file_offset, header.list_1);
    let mut units: Vec<Unit> = Vec::new();

    let mut items: Vec<UnitKind> = Vec::new();

    while next_offset != 0 {
        reader.seek(SeekFrom::Start(next_offset))?;

        let value = reader.read_u8()?;
        let kind = ItemParser::try_from(value)?;

        let pointer = match kind {
            ItemParser::CpuPlayerInformation => {
                let cpu_player_information = parse_cpu_player_information(&mut reader)?;
                items.push(UnitKind::CpuPlayerInformation);
                cpu_player_information.list_1
            }
            ItemParser::MapConfiguration => {
                let map_configuration = parse_map_configuration(&mut reader)?;
                items.push(UnitKind::MapConfiguration);
                map_configuration.list_1
            }
            ItemParser::ScrollStart => {
                let scroll_start = parse_scroll_start(&mut reader)?;
                items.push(UnitKind::ScrollStart);
                scroll_start.list_1
            }
            ItemParser::Ripple => {
                let ripple = parse_ripple(&mut reader)?;
                items.push(UnitKind::ScrollStart);
                ripple.list_1
            }
            ItemParser::Unit(id) => {
                let unit = parse_unit(&mut reader, id)?;
                let item = UnitKind::from_number(unit.kind);
                items.push(item.unwrap_or(UnitKind::UnknownUnit(unit.kind)));
                unit.list_1
            }
        };

        next_offset = file_pointer_to_offset(file_offset, pointer);
    }

    for item in &items {
        println!("Item: {}", item);
        // let name = UnitKind::from_number(unit.kind).unwrap_or(UnitKind::UnknownUnit(unit.kind));
    }

    Ok(())
}
