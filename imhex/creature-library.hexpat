// kknd2-cplc-parser
// Copyright (c) 2024 Matthew Costa <ucosty@gmail.com>
//
// SPDX-License-Identifier: MIT

#pragma pattern_limit 1048576

using PascalString;
using Creature;

fn display_pascal_string(PascalString value) {
    return value.string;
};

fn display_creature(Creature creature) {
    return creature.name.string;
};

struct PascalString {
    u8 len;
    char string[len];
} [[format_read("display_pascal_string")]];

struct ArrayPropertyItem {
    PascalString;
    u32 value;
};

struct ArrayProperty {
    u8 stuff[11];
    u8 array_length;
    u8 unknown;
    ArrayPropertyItem items[array_length];
};

struct Property {
    PascalString;
    u16 kind;
    
    if (kind == 1 || kind == 2 || kind == 3) {
        ArrayProperty;
    } else {
        u8 stuff[13];
    }
};

struct Bitmap {
    u16 magic;
    u16 length;
    u8 content[length - 4];
    u8 data[length] @ addressof(magic);
} [[hex::visualize("image", data)]];

struct Creature {
    char magic[4];
    u16 id;
    PascalString name;
    u16 meta_length;
    u8 meta[meta_length];
    u8 unknown[6];
    
    u16 property_count;
    Property properties[property_count];
    
    u8 marker;
    
    Bitmap bitmap;
} [[format_read("display_creature")]];


char magic[4] @0x0;
u16 count @0x04;

Creature creature[count] @ $;
