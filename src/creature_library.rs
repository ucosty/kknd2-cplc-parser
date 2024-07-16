// kknd2-cplc-parser
// Copyright (c) 2024 Matthew Costa <ucosty@gmail.com>
//
// SPDX-License-Identifier: MIT

use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{BufReader, Read, Seek};

use byteorder::{LittleEndian, ReadBytesExt};

const FILE_MAGIC: u32 = 0x4c43324b;
const ENTRY_MAGIC: u32 = 0x5243324b;
const BITMAP_MAGIC: u16 = 0x4D42;
const BITMAP_HEADER_SIZE: i64 = 14;

struct BitmapHeader {
    magic: u16,
    size_in_bytes: u32,
    reserved_1: u16,
    reserved_2: u16,
    pixel_data_offset: u32
}

pub struct Creature {
    pub id: u16,
    pub name: String,
    pub size: usize,
}

fn parse_bitmap_header<R: Read + Seek>(reader: &mut BufReader<R>) -> Result<BitmapHeader, Box<dyn Error>> {
    let magic = reader.read_u16::<LittleEndian>()?;
    if magic != BITMAP_MAGIC {
        return Err(format!("Invalid magic. Expected {:#x}, found {:#x} at {:#x}", BITMAP_MAGIC, magic, reader.stream_position()? - 2).into());
    }

    let size_in_bytes = reader.read_u32::<LittleEndian>()?;
    let reserved_1 = reader.read_u16::<LittleEndian>()?;
    let reserved_2 = reader.read_u16::<LittleEndian>()?;
    let pixel_data_offset = reader.read_u32::<LittleEndian>()?;

    Ok(BitmapHeader{
        magic,
        size_in_bytes,
        reserved_1,
        reserved_2,
        pixel_data_offset,
    })
}

fn parse_bitmap<R: Read + Seek>(reader: &mut BufReader<R>) -> Result<(), Box<dyn Error>> {
    let header = parse_bitmap_header(&mut *reader)?;
    reader.seek_relative(header.size_in_bytes as i64 - BITMAP_HEADER_SIZE)?;

    Ok(())
}

fn parse_pascal_string<R: Read>(reader: &mut BufReader<R>) -> Result<String, Box<dyn Error>> {
    let length = reader.read_u8()?;

    let mut buffer: Vec<u8> = Vec::new();
    buffer.resize(length as usize, 0);
    reader.read_exact(buffer.as_mut_slice())?;

    Ok(String::from_utf8(buffer)?)
}

fn parse_array_item<R: Read + Seek>(reader: &mut BufReader<R>) -> Result<(String, u32), Box<dyn Error>> {
    let name = parse_pascal_string(&mut *reader)?;
    let value = reader.read_u32::<LittleEndian>()?;
    Ok((name, value))
}

fn parse_array_list<R: Read + Seek>(reader: &mut BufReader<R>) -> Result<Vec<(String, u32)>, Box<dyn Error>> {
    reader.seek_relative(12)?;
    let array_length = reader.read_u16::<LittleEndian>()?;

    let mut result: Vec<(String, u32)> = Vec::new();

    for _i in 0..array_length {
        result.push(parse_array_item(&mut *reader)?);
    }

    Ok(result)
}

fn parse_property<R: Read + Seek>(reader: &mut BufReader<R>) -> Result<(), Box<dyn Error>> {
    let _name = parse_pascal_string(&mut *reader)?;
    let kind = reader.read_u8()?;

    if kind == 1 || kind == 2 || kind == 3 {
        parse_array_list(&mut *reader)?;
    } else {
        reader.seek_relative(14)?;
    }

    Ok(())
}

fn parse_entry<R: Read + Seek>(reader: &mut BufReader<R>) -> Result<Creature, Box<dyn Error>> {
    let magic = reader.read_u32::<LittleEndian>()?;
    if magic != ENTRY_MAGIC {
        return Err(format!("Invalid magic. Expected {:#x}, found {:#x} at {:#x}", ENTRY_MAGIC, magic, reader.stream_position()? - 4).into());
    }

    let id = reader.read_u16::<LittleEndian>()?;
    let name = parse_pascal_string(&mut *reader)?;

    let metadata_length = reader.read_u16::<LittleEndian>()?;
    reader.seek_relative((metadata_length + 6) as i64)?;

    let property_count = reader.read_u16::<LittleEndian>()?;
    for _ in 0..property_count {
        parse_property(&mut *reader)?;
    }

    let _marker = reader.read_u8()?;
    parse_bitmap(&mut *reader)?;

    Ok(Creature{
        id,
        name,
        size: (32 + metadata_length) as usize,
    })
}

pub fn parse(filename: &str) -> Result<HashMap<u16, Creature>, Box<dyn Error>> {
    let file = File::open(filename).map_err(|e| format!("Could not open creature library ({}): {}", filename, e))?;

    let mut reader = BufReader::new(file);

    let magic = reader.read_u32::<LittleEndian>()?;
    if magic != FILE_MAGIC {
        return Err(format!("Invalid magic. Expected {:#x}, found {:#x}", FILE_MAGIC, magic).into());
    }

    let total_entries = reader.read_u16::<LittleEndian>()?;

    let mut library: HashMap<u16, Creature> = HashMap::new();

    while library.len() < total_entries as usize {
        let creature = parse_entry(&mut reader)?;
        library.insert(creature.id, creature);
    }

    Ok(library)
}
