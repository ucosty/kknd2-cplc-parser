// kknd2-cplc-parser
// Copyright (c) 2024 Matthew Costa <ucosty@gmail.com>
//
// SPDX-License-Identifier: MIT

use crate::creature_library::Creature;
use byteorder::{LittleEndian, ReadBytesExt};
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{BufReader, Read, Seek, SeekFrom};

pub struct BasicInformation {
    pub x: u32,
    pub y: u32,
    pub list_1: u32,
    pub list_2: u32,
    pub list_3: u32,
    pub list_4: u32,
}

#[derive(Debug)]
pub struct CpuPlayerInformation {
    pub ally_mode: u32,
    pub cost_modifier: u16,
    pub time_modifier: u16,
    pub confidence: u16,
    pub default_units: [u16; 3],
}

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
    team: u16,
    flags: u16,
    unknown: u16,
    activation_timer: u16,
}

#[derive(Debug)]
struct ScrollStart {}

#[derive(Debug)]
struct Ripple {}

#[derive(Debug)]
struct MapConfiguration {
    ally_funds: u16,
    enemy_funds: u16,
    player_funds: u16,
    build_restrictions: [u16; 4],
    par_time: u16,
    matrix_set: u16,
    counter: u32,
    counter_function: u16,
    mission_win_condition: u16,
    mission_lose_condition: u16,
    max_tech_level: u16,
    local_team: u16,
    local_race: u16,
    team_colours: [u16; 9],
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
    Unit,
}

impl From<u8> for ItemParser {
    fn from(value: u8) -> Self {
        match value {
            0x01 => ItemParser::CpuPlayerInformation,
            0x04 => ItemParser::MapConfiguration,
            0x09 => ItemParser::ScrollStart,
            0xa9 => ItemParser::Ripple,
            _ => ItemParser::Unit,
        }
    }
}

fn parse_unit<R: Read + Seek>(
    reader: &mut BufReader<R>,
) -> Result<(BasicInformation, Unit), Box<dyn Error>> {
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

    Ok((
        BasicInformation {
            x,
            y,
            list_1,
            list_2,
            list_3,
            list_4,
        },
        Unit {
            team,
            flags,
            unknown,
            activation_timer,
        },
    ))
}

fn parse_ripple<R: Read + Seek>(
    reader: &mut BufReader<R>,
) -> Result<(BasicInformation, Ripple), Box<dyn Error>> {
    reader.seek_relative(4)?;

    let x = reader.read_u32::<LittleEndian>()?;
    let y = reader.read_u32::<LittleEndian>()?;

    reader.seek_relative(3)?;

    let list_1 = reader.read_u32::<LittleEndian>()?;
    let list_2 = reader.read_u32::<LittleEndian>()?;
    let list_3 = reader.read_u32::<LittleEndian>()?;
    let list_4 = reader.read_u32::<LittleEndian>()?;

    Ok((
        BasicInformation {
            x,
            y,
            list_1,
            list_2,
            list_3,
            list_4,
        },
        Ripple {},
    ))
}

fn parse_scroll_start<R: Read + Seek>(
    reader: &mut BufReader<R>,
) -> Result<(BasicInformation, ScrollStart), Box<dyn Error>> {
    reader.seek_relative(4)?;

    let x = reader.read_u32::<LittleEndian>()?;
    let y = reader.read_u32::<LittleEndian>()?;

    reader.seek_relative(3)?;

    let list_1 = reader.read_u32::<LittleEndian>()?;
    let list_2 = reader.read_u32::<LittleEndian>()?;
    let list_3 = reader.read_u32::<LittleEndian>()?;
    let list_4 = reader.read_u32::<LittleEndian>()?;

    Ok((
        BasicInformation {
            x,
            y,
            list_1,
            list_2,
            list_3,
            list_4,
        },
        ScrollStart {},
    ))
}

fn parse_cpu_player_information<R: Read + Seek>(
    reader: &mut BufReader<R>,
) -> Result<(BasicInformation, CpuPlayerInformation), Box<dyn Error>> {
    reader.seek_relative(4)?;

    let x = reader.read_u32::<LittleEndian>()?;
    let y = reader.read_u32::<LittleEndian>()?;

    reader.seek_relative(3)?;

    let list_1 = reader.read_u32::<LittleEndian>()?;
    let list_2 = reader.read_u32::<LittleEndian>()?;
    let list_3 = reader.read_u32::<LittleEndian>()?;
    let list_4 = reader.read_u32::<LittleEndian>()?;

    reader.seek_relative(20)?;

    let ally_mode = reader.read_u32::<LittleEndian>()?;
    let cost_modifier = reader.read_u16::<LittleEndian>()?;
    let time_modifier = reader.read_u16::<LittleEndian>()?;
    let confidence = reader.read_u16::<LittleEndian>()?;
    let default_unit_1 = reader.read_u16::<LittleEndian>()?;
    let default_unit_2 = reader.read_u16::<LittleEndian>()?;
    let default_unit_3 = reader.read_u16::<LittleEndian>()?;

    Ok((
        BasicInformation {
            x,
            y,
            list_1,
            list_2,
            list_3,
            list_4,
        },
        CpuPlayerInformation {
            ally_mode,
            cost_modifier,
            time_modifier,
            confidence,
            default_units: [default_unit_1, default_unit_2, default_unit_3],
        },
    ))
}

fn parse_team_colours<R: Read>(reader: &mut BufReader<R>) -> Result<[u16; 9], Box<dyn Error>> {
    let mut team_colours : Vec<u16> = Vec::new();
    team_colours.resize(9, 0);
    for i in 0..9 {
        team_colours[i] = reader.read_u16::<LittleEndian>()?;
    }

    let result = <[u16; 9]>::try_from(team_colours.as_slice())?;

    Ok(result)
}

fn parse_build_restrictions<R: Read>(reader: &mut BufReader<R>) -> Result<[u16; 4], Box<dyn Error>> {
    let mut build_restrictions : Vec<u16> = Vec::new();
    build_restrictions.resize(4, 0);
    for i in 0..4 {
        build_restrictions[i] = reader.read_u16::<LittleEndian>()?;
    }

    Ok(<[u16; 4]>::try_from(build_restrictions.as_slice())?)
}


fn parse_map_configuration<R: Read + Seek>(
    reader: &mut BufReader<R>,
) -> Result<(BasicInformation, MapConfiguration), Box<dyn Error>> {
    reader.seek_relative(4)?;

    let x = reader.read_u32::<LittleEndian>()?;
    let y = reader.read_u32::<LittleEndian>()?;

    reader.seek_relative(3)?;

    let list_1 = reader.read_u32::<LittleEndian>()?;
    let list_2 = reader.read_u32::<LittleEndian>()?;
    let list_3 = reader.read_u32::<LittleEndian>()?;
    let list_4 = reader.read_u32::<LittleEndian>()?;

    reader.seek_relative(20)?;

    let team_colours = parse_team_colours(&mut *reader)?;

    let local_team = reader.read_u16::<LittleEndian>()?;
    let local_race = reader.read_u16::<LittleEndian>()?;
    let counter = reader.read_u32::<LittleEndian>()?;
    let build_restrictions = parse_build_restrictions(&mut *reader)?;
    let ally_funds = reader.read_u16::<LittleEndian>()?;
    let enemy_funds = reader.read_u16::<LittleEndian>()?;
    let player_funds = reader.read_u16::<LittleEndian>()?;
    let mission_lose_condition = reader.read_u16::<LittleEndian>()?;
    let mission_win_condition = reader.read_u16::<LittleEndian>()?;
    let max_tech_level = reader.read_u16::<LittleEndian>()?;
    reader.seek_relative(4)?;
    let counter_function = reader.read_u16::<LittleEndian>()?;
    let matrix_set = reader.read_u16::<LittleEndian>()?;
    let par_time = reader.read_u16::<LittleEndian>()?;

    Ok((
        BasicInformation {
            x,
            y,
            list_1,
            list_2,
            list_3,
            list_4,
        },
        MapConfiguration {
            ally_funds,
            enemy_funds,
            player_funds,
            build_restrictions,
            par_time,
            matrix_set,
            counter,
            counter_function,
            mission_win_condition,
            mission_lose_condition,
            max_tech_level,
            local_team,
            local_race,
            team_colours,
        },
    ))
}

#[derive(Debug)]
enum Entity {
    MapConfiguration(MapConfiguration),
    CpuPlayerInformation(CpuPlayerInformation),
    ScrollStart(ScrollStart),
    Ripple(Ripple),
    Unit(Unit),
}

struct MapEntity {
    kind: u8,
    x: u32,
    y: u32,
    entity: Entity,
}

pub fn parse(filename: &str, library: &HashMap<u16, Creature>) -> Result<(), Box<dyn Error>> {
    let file = File::open(filename).map_err(|e| format!("Failed to open file: {}", e))?;

    let mut reader = BufReader::new(file);

    let magic = reader.read_u32::<LittleEndian>()?;
    if magic != 0xdeadc0de {
        return Err(format!("Invalid magic. Expected 0xdeadc0de, found {:#x}", magic).into());
    }

    let file_offset = reader.read_u32::<LittleEndian>()?;
    let header = parse_header(&mut reader)?;

    let mut next_offset = file_pointer_to_offset(file_offset, header.list_1);
    let mut entities: Vec<MapEntity> = Vec::new();

    while next_offset != 0 {
        reader.seek(SeekFrom::Start(next_offset))?;

        let value = reader.read_u8()?;
        let kind = ItemParser::try_from(value)?;

        let pointer = match kind {
            ItemParser::CpuPlayerInformation => {
                let (basic_info, cpu_player_info) = parse_cpu_player_information(&mut reader)?;
                entities.push(MapEntity {
                    kind: value,
                    x: basic_info.x,
                    y: basic_info.y,
                    entity: Entity::CpuPlayerInformation(cpu_player_info),
                });
                basic_info.list_1
            }
            ItemParser::MapConfiguration => {
                let (basic_info, map_config) = parse_map_configuration(&mut reader)?;
                entities.push(MapEntity {
                    kind: value,
                    x: basic_info.x,
                    y: basic_info.y,
                    entity: Entity::MapConfiguration(map_config),
                });
                basic_info.list_1
            }
            ItemParser::ScrollStart => {
                let (basic_info, scroll_start) = parse_scroll_start(&mut reader)?;
                entities.push(MapEntity {
                    kind: value,
                    x: basic_info.x,
                    y: basic_info.y,
                    entity: Entity::ScrollStart(scroll_start),
                });
                basic_info.list_1
            }
            ItemParser::Ripple => {
                let (basic_info, ripple) = parse_ripple(&mut reader)?;
                entities.push(MapEntity {
                    kind: value,
                    x: basic_info.x,
                    y: basic_info.y,
                    entity: Entity::Ripple(ripple),
                });
                basic_info.list_1
            }
            ItemParser::Unit => {
                let (basic_info, unit) = parse_unit(&mut reader)?;
                entities.push(MapEntity {
                    kind: value,
                    x: basic_info.x,
                    y: basic_info.y,
                    entity: Entity::Unit(unit),
                });
                basic_info.list_1
            }
        };

        next_offset = file_pointer_to_offset(file_offset, pointer);
    }

    let mut count = 0;
    for entity in entities {
        let fallback_entry = Creature {
            id: entity.kind as u16,
            name: "Unknown entry".to_string(),
            size: 0,
        };
        let library_entry = library
            .get(&(entity.kind as u16))
            .unwrap_or(&fallback_entry);

        println!("Entity {}", count);
        println!("  ├─ kind = {:#x}", entity.kind);
        println!("  ├─ name = {}", library_entry.name);
        println!("  ├─    x = {}", entity.x);
        println!("  ├─    y = {}", entity.y);
        println!("  └─ data = {:?}", entity.entity);

        count = count + 1;
    }

    Ok(())
}
