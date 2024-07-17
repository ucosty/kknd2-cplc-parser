# kknd2-cplc-parser

`kknd2-cplc-parser` is a small utility which parses the contents of a KKnD 2 map CPLC file, which contains the units and map metadata for a KKnD 2 map.

## Building

Build with the Rust package manager Cargo.

```shell
cargo build
```

## Usage

This requires the `Creatures.klb` file to be present in the working directory. Alternatively you can use the `-c` option to pass the path to this file.

```text
Usage: kknd2-cplc-parser [OPTIONS] <FILENAME>

Arguments:
  <FILENAME>  

Options:
  -c, --creature-library <CREATURE_LIBRARY>  [default: Creature.klb]
  -h, --help                                 Print help
  -V, --version                              Print version
```

## Example Output

```text
kknd2-cplc-parser test.CPLC          
Entity 0
  ├─ kind = 0x7e
  ├─ name = Survivor Laser Rifleman
  ├─    x = 192
  ├─    y = 896
  └─ data = Unit(Unit { team: 6, flags: 0, unknown: 18, activation_timer: 0 })
Entity 1
  ├─ kind = 0x9
  ├─ name = Scroll start
  ├─    x = 352
  ├─    y = 320
  └─ data = ScrollStart(ScrollStart)
Entity 2
  ├─ kind = 0x43
  ├─ name = Series 9 Windmill
  ├─    x = 416
  ├─    y = 448
  └─ data = Unit(Unit { team: 2, flags: 524, unknown: 95, activation_timer: 32 })
Entity 3
  ├─ kind = 0x7e
  ├─ name = Survivor Laser Rifleman
  ├─    x = 416
  ├─    y = 896
  └─ data = Unit(Unit { team: 6, flags: 0, unknown: 18, activation_timer: 0 })
Entity 4
  ├─ kind = 0x1
  ├─ name = CPU player information
  ├─    x = 832
  ├─    y = 1312
  └─ data = CpuPlayerInformation(CpuPlayerInformation { ally_mode: 0, cost_modifier: 100, time_modifier: 100, confidence: 50, default_units: [6, 6, 6] })
Entity 5
  ├─ kind = 0x25
  ├─ name = Evolved Martyr
  ├─    x = 928
  ├─    y = 1248
  └─ data = Unit(Unit { team: 3, flags: 0, unknown: 25, activation_timer: 0 })
Entity 6
  ├─ kind = 0x1
  ├─ name = CPU player information
  ├─    x = 1152
  ├─    y = 256
  └─ data = CpuPlayerInformation(CpuPlayerInformation { ally_mode: 1, cost_modifier: 100, time_modifier: 100, confidence: 50, default_units: [69, 71, 70] })
Entity 7
  ├─ kind = 0x4
  ├─ name = Level information
  ├─    x = 1440
  ├─    y = 768
  └─ data = MapConfiguration(MapConfiguration { ally_funds: 4660, enemy_funds: 51966, player_funds: 1337, build_restrictions: [65535, 65535, 8055, 2046], par_time: 57005, matrix_set: 32776, counter: 49374, counter_function: 1, mission_win_condition: 6, mission_lose_condition: 4, max_tech_level: 3, local_team: 6, local_race: 2, team_colours: [7, 1, 3, 2, 4, 5, 6, 1, 0] })
```

## License

The project is licensed under the MIT License.
