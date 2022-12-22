use std::collections::{HashSet, VecDeque};
use std::convert::TryInto;
use std::fmt::{Debug, Formatter};
use std::io::{BufReader, Write};
use std::path::PathBuf;
use std::sync::{Arc, RwLock};
use std::sync::atomic::AtomicUsize;
use std::time::Instant;
use bimap::{BiBTreeMap, BiMap};
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use elor::Either;
use get_size::GetSize;
use elor::Either::{Left, Right};
use rayon::prelude::{IntoParallelIterator, IntoParallelRefIterator, IntoParallelRefMutIterator, ParallelIterator};
use serde::{Serialize, Serializer};

#[derive(Copy, Clone, Eq, PartialEq, Debug, GetSize, serde::Serialize, serde::Deserialize, PartialOrd, Ord, Hash)]
#[repr(u16)]
pub enum Block {
    Air = 0,
    Stone = 1,
    Grass = 2,
    Dirt = 3,
    Cobblestone = 4,
    Wood = 5,
    Sapling = 6,
    Bedrock = 7,
    WaterSolid = 8,
    Water = 9,
    LavaSolid = 10,
    Lava = 11,
    Sand = 12,
    Gravel = 13,
    GoldOre = 14,
    IronOre = 15,
    CoalOre = 16,
    Log = 17,
    Leaves = 18,
    Sponge = 19,
    Glass = 20,
    LapisOre = 21,
    LapisBlock = 22,
    Dispenser = 23,
    Sandstone = 24,
    NoteBlock = 25,
    BedBlock = 26,
    PoweredRail = 27,
    DetectorRail = 28,
    PistonStickyBase = 29,
    Web = 30,
    LongGrassBottom = 31,
    LongGrassUp = 32,
    PistonBase = 33,
    PistonExtension = 34,
    Wool = 35,
    PistonMovingPiece = 36,
    YellowFlower = 37,
    RedRose = 38,
    BrownMushroom = 39,
    RedMushroom = 40,
    GoldBlock = 41,
    IronBlock = 42,
    StepDown = 43,
    StepUp = 44,
    Brick = 45,
    Tnt = 46,
    Bookshelf = 47,
    MossyCobblestone = 48,
    Obsidian = 49,
    Torch = 50,
    Fire = 51,
    MobSpawner = 52,
    WoodStairs = 53,
    Chest = 54,
    RedstoneWire = 55,
    DiamondOre = 56,
    DiamondBlock = 57,
    Workbench = 58,
    Crops = 59,
    Soil = 60,
    FurnaceOff = 61,
    FurnaceOn = 62,
    SignPost = 63,
    WoodenDoor = 64,
    Ladder = 65,
    Rails = 66,
    CobblestoneStairs = 67,
    WallSign = 68,
    Lever = 69,
    StonePlate = 70,
    IronDoorBlock = 71,
    WoodPlate = 72,
    RedstoneOreOff = 73,
    RedstoneOreOn = 74,
    RedstoneTorchOff = 75,
    RedstoneTorchOn = 76,
    StoneButton = 77,
    Snow = 78,
    Ice = 79,
    SnowBlock = 80,
    Cactus = 81,
    Clay = 82,
    SugarCaneBlock = 83,
    Jukebox = 84,
    Fence = 85,
    Pumpkin = 86,
    Netherrack = 87,
    SoulSand = 88,
    Glowstone = 89,
    Portal = 90,
    JackOLantern = 91,
    CakeBlock = 92,
    DiodeBlockOff = 93,
    DiodeBlockOn = 94,
    StainedGlass = 95,
    TrapDoor = 96,
    MonsterEggs = 97,
    SmoothBrick = 98,
    HugeMushroom1 = 99,
    HugeMushroom2 = 100,
    IronFence = 101,
    ThinGlass = 102,
    MelonBlock = 103,
    PumpkinStem = 104,
    MelonStem = 105,
    Vine = 106,
    FenceGate = 107,
    BrickStairs = 108,
    SmoothStairs = 109,
    Mycel = 110,
    WaterLily = 111,
    NetherBrick = 112,
    NetherFence = 113,
    NetherBrickStairs = 114,
    NetherWarts = 115,
    EnchantmentTable = 116,
    BrewingStand = 117,
    Cauldron = 118,
    EnderPortal = 119,
    EnderPortalFrame = 120,
    EnderStone = 121,
    DragonEgg = 122,
    RedstoneLampOff = 123,
    RedstoneLampOn = 124,
    WoodStepDown = 125,
    WoodStepUp = 126,
    Cocoa = 127,
    SandstoneStairs = 128,
    EmeraldOre = 129,
    EnderChest = 130,
    TripwireHook = 131,
    Tripwire = 132,
    EmeraldBlock = 133,
    SpruceWoodStairs = 134,
    BirchWoodStairs = 135,
    JungleWoodStairs = 136,
    Command = 137,
    Beacon = 138,
    CobbleWall = 139,
    FlowerPot = 140,
    Carrot = 141,
    Potato = 142,
    WoodButton = 143,
    Anvil = 145,
    TrappedChest = 146,
    GoldPlate = 147,
    IronPlate = 148,
    RedstoneComparatorOff = 149,
    RedstoneComparatorOn = 150,
    DaylightDetectorOff = 151,
    RedstoneBlock = 152,
    QuartzOre = 153,
    Hopper = 154,
    QuartzBlock = 155,
    QuartzStairs = 156,
    ActivatorRail = 157,
    Dropper = 158,
    StainedClay = 159,
    StainedGlassPane = 160,
    Leaves2 = 161,
    Log2 = 162,
    AcaciaStairs = 163,
    DarkOakStairs = 164,
    SlimeBlock = 165,
    Barrier = 166,
    IronTrapdoor = 167,
    Prismarine = 168,
    SeaLantern = 169,
    HayBlock = 170,
    Carpet = 171,
    HardClay = 172,
    CoalBlock = 173,
    PackedIce = 174,
    DoublePlant = 175,
    StandingBanner = 176,
    WallBanner = 177,
    DaylightDetectorOn = 178,
    RedSandstone = 179,
    RedSandstoneStairs = 180,
    StoneSlab2 = 181,
    SpruceFenceGate = 183,
    BirchFenceGate = 184,
    JungleFenceGate = 185,
    DarkOakFenceGate = 186,
    AcaciaFenceGate = 187,
    SpruceFence = 188,
    BirchFence = 189,
    JungleFence = 190,
    DarkOakFence = 191,
    AcaciaFence = 192,
    SpruceDoor = 193,
    BirchDoor = 194,
    JungleDoor = 195,
    AcaciaDoor = 196,
    DarkOakDoor = 197,
    EndRod = 198,
    ChorusPlant = 199,
    ChorusFlower = 200,
    PurpurBlock = 201,
    PurpurPillar = 202,
    PurpurStairs = 203,
    PurpurSlabBottom = 204,
    PurpurSlabUp = 205,
    EndBricks = 206,
    BeetrootBlock = 207,
    GrassPath = 208,
    EndGateway = 209,
    CommandRepeating = 210,
    CommandChain = 211,
    FrostedIce = 212,
    Magma = 213,
    NetherWartBlock = 214,
    RedNetherBrick = 215,
    BoneBlock = 216,
    StructureVoid = 217,
    Observer = 218,
    WhiteShulkerBox = 219,
    OrangeShulkerBox = 220,
    MagentaShulkerBox = 221,
    LightBlueShulkerBox = 222,
    YellowShulkerBox = 223,
    LimeShulkerBox = 224,
    PinkShulkerBox = 225,
    GrayShulkerBox = 226,
    SilverShulkerBox = 227,
    CyanShulkerBox = 228,
    PurpleShulkerBox = 229,
    BlueShulkerBox = 230,
    BrownShulkerBox = 231,
    GreenShulkerBox = 232,
    RedShulkerBox = 233,
    BlackShulkerBox = 234,
    WhiteGlazedTerracotta = 235,
    OrangeGlazedTerracotta = 236,
    MagentaGlazedTerracotta = 237,
    LightBlueGlazedTerracotta = 238,
    YellowGlazedTerracotta = 239,
    LimeGlazedTerracotta = 240,
    PinkGlazedTerracotta = 241,
    GrayGlazedTerracotta = 242,
    SilverGlazedTerracotta = 243,
    CyanGlazedTerracotta = 244,
    PurpleGlazedTerracotta = 245,
    BlueGlazedTerracotta = 246,
    BrownGlazedTerracotta = 247,
    GreenGlazedTerracotta = 248,
    RedGlazedTerracotta = 249,
    BlackGlazedTerracotta = 250,
    Concrete = 251,
    ConcretePowder = 252,
    StructureBlock = 255,
}

impl Into<u16> for Block {
    fn into(self) -> u16 {
        match self {
            Self::Air => 0,
            Self::Stone => 1,
            Self::Grass => 2,
            Self::Dirt => 3,
            Self::Cobblestone => 4,
            Self::Wood => 5,
            Self::Sapling => 6,
            Self::Bedrock => 7,
            Self::WaterSolid => 8,
            Self::Water => 9,
            Self::LavaSolid => 10,
            Self::Lava => 11,
            Self::Sand => 12,
            Self::Gravel => 13,
            Self::GoldOre => 14,
            Self::IronOre => 15,
            Self::CoalOre => 16,
            Self::Log => 17,
            Self::Leaves => 18,
            Self::Sponge => 19,
            Self::Glass => 20,
            Self::LapisOre => 21,
            Self::LapisBlock => 22,
            Self::Dispenser => 23,
            Self::Sandstone => 24,
            Self::NoteBlock => 25,
            Self::BedBlock => 26,
            Self::PoweredRail => 27,
            Self::DetectorRail => 28,
            Self::PistonStickyBase => 29,
            Self::Web => 30,
            Self::LongGrassBottom => 31,
            Self::LongGrassUp => 32,
            Self::PistonBase => 33,
            Self::PistonExtension => 34,
            Self::Wool => 35,
            Self::PistonMovingPiece => 36,
            Self::YellowFlower => 37,
            Self::RedRose => 38,
            Self::BrownMushroom => 39,
            Self::RedMushroom => 40,
            Self::GoldBlock => 41,
            Self::IronBlock => 42,
            Self::StepDown => 43,
            Self::StepUp => 44,
            Self::Brick => 45,
            Self::Tnt => 46,
            Self::Bookshelf => 47,
            Self::MossyCobblestone => 48,
            Self::Obsidian => 49,
            Self::Torch => 50,
            Self::Fire => 51,
            Self::MobSpawner => 52,
            Self::WoodStairs => 53,
            Self::Chest => 54,
            Self::RedstoneWire => 55,
            Self::DiamondOre => 56,
            Self::DiamondBlock => 57,
            Self::Workbench => 58,
            Self::Crops => 59,
            Self::Soil => 60,
            Self::FurnaceOff => 61,
            Self::FurnaceOn => 62,
            Self::SignPost => 63,
            Self::WoodenDoor => 64,
            Self::Ladder => 65,
            Self::Rails => 66,
            Self::CobblestoneStairs => 67,
            Self::WallSign => 68,
            Self::Lever => 69,
            Self::StonePlate => 70,
            Self::IronDoorBlock => 71,
            Self::WoodPlate => 72,
            Self::RedstoneOreOff => 73,
            Self::RedstoneOreOn => 74,
            Self::RedstoneTorchOff => 75,
            Self::RedstoneTorchOn => 76,
            Self::StoneButton => 77,
            Self::Snow => 78,
            Self::Ice => 79,
            Self::SnowBlock => 80,
            Self::Cactus => 81,
            Self::Clay => 82,
            Self::SugarCaneBlock => 83,
            Self::Jukebox => 84,
            Self::Fence => 85,
            Self::Pumpkin => 86,
            Self::Netherrack => 87,
            Self::SoulSand => 88,
            Self::Glowstone => 89,
            Self::Portal => 90,
            Self::JackOLantern => 91,
            Self::CakeBlock => 92,
            Self::DiodeBlockOff => 93,
            Self::DiodeBlockOn => 94,
            Self::StainedGlass => 95,
            Self::TrapDoor => 96,
            Self::MonsterEggs => 97,
            Self::SmoothBrick => 98,
            Self::HugeMushroom1 => 99,
            Self::HugeMushroom2 => 100,
            Self::IronFence => 101,
            Self::ThinGlass => 102,
            Self::MelonBlock => 103,
            Self::PumpkinStem => 104,
            Self::MelonStem => 105,
            Self::Vine => 106,
            Self::FenceGate => 107,
            Self::BrickStairs => 108,
            Self::SmoothStairs => 109,
            Self::Mycel => 110,
            Self::WaterLily => 111,
            Self::NetherBrick => 112,
            Self::NetherFence => 113,
            Self::NetherBrickStairs => 114,
            Self::NetherWarts => 115,
            Self::EnchantmentTable => 116,
            Self::BrewingStand => 117,
            Self::Cauldron => 118,
            Self::EnderPortal => 119,
            Self::EnderPortalFrame => 120,
            Self::EnderStone => 121,
            Self::DragonEgg => 122,
            Self::RedstoneLampOff => 123,
            Self::RedstoneLampOn => 124,
            Self::WoodStepDown => 125,
            Self::WoodStepUp => 126,
            Self::Cocoa => 127,
            Self::SandstoneStairs => 128,
            Self::EmeraldOre => 129,
            Self::EnderChest => 130,
            Self::TripwireHook => 131,
            Self::Tripwire => 132,
            Self::EmeraldBlock => 133,
            Self::SpruceWoodStairs => 134,
            Self::BirchWoodStairs => 135,
            Self::JungleWoodStairs => 136,
            Self::Command => 137,
            Self::Beacon => 138,
            Self::CobbleWall => 139,
            Self::FlowerPot => 140,
            Self::Carrot => 141,
            Self::Potato => 142,
            Self::WoodButton => 143,
            Self::Anvil => 145,
            Self::TrappedChest => 146,
            Self::GoldPlate => 147,
            Self::IronPlate => 148,
            Self::RedstoneComparatorOff => 149,
            Self::RedstoneComparatorOn => 150,
            Self::DaylightDetectorOff => 151,
            Self::RedstoneBlock => 152,
            Self::QuartzOre => 153,
            Self::Hopper => 154,
            Self::QuartzBlock => 155,
            Self::QuartzStairs => 156,
            Self::ActivatorRail => 157,
            Self::Dropper => 158,
            Self::StainedClay => 159,
            Self::StainedGlassPane => 160,
            Self::Leaves2 => 161,
            Self::Log2 => 162,
            Self::AcaciaStairs => 163,
            Self::DarkOakStairs => 164,
            Self::SlimeBlock => 165,
            Self::Barrier => 166,
            Self::IronTrapdoor => 167,
            Self::Prismarine => 168,
            Self::SeaLantern => 169,
            Self::HayBlock => 170,
            Self::Carpet => 171,
            Self::HardClay => 172,
            Self::CoalBlock => 173,
            Self::PackedIce => 174,
            Self::DoublePlant => 175,
            Self::StandingBanner => 176,
            Self::WallBanner => 177,
            Self::DaylightDetectorOn => 178,
            Self::RedSandstone => 179,
            Self::RedSandstoneStairs => 180,
            Self::StoneSlab2 => 181,
            Self::SpruceFenceGate => 183,
            Self::BirchFenceGate => 184,
            Self::JungleFenceGate => 185,
            Self::DarkOakFenceGate => 186,
            Self::AcaciaFenceGate => 187,
            Self::SpruceFence => 188,
            Self::BirchFence => 189,
            Self::JungleFence => 190,
            Self::DarkOakFence => 191,
            Self::AcaciaFence => 192,
            Self::SpruceDoor => 193,
            Self::BirchDoor => 194,
            Self::JungleDoor => 195,
            Self::AcaciaDoor => 196,
            Self::DarkOakDoor => 197,
            Self::EndRod => 198,
            Self::ChorusPlant => 199,
            Self::ChorusFlower => 200,
            Self::PurpurBlock => 201,
            Self::PurpurPillar => 202,
            Self::PurpurStairs => 203,
            Self::PurpurSlabBottom => 204,
            Self::PurpurSlabUp => 205,
            Self::EndBricks => 206,
            Self::BeetrootBlock => 207,
            Self::GrassPath => 208,
            Self::EndGateway => 209,
            Self::CommandRepeating => 210,
            Self::CommandChain => 211,
            Self::FrostedIce => 212,
            Self::Magma => 213,
            Self::NetherWartBlock => 214,
            Self::RedNetherBrick => 215,
            Self::BoneBlock => 216,
            Self::StructureVoid => 217,
            Self::Observer => 218,
            Self::WhiteShulkerBox => 219,
            Self::OrangeShulkerBox => 220,
            Self::MagentaShulkerBox => 221,
            Self::LightBlueShulkerBox => 222,
            Self::YellowShulkerBox => 223,
            Self::LimeShulkerBox => 224,
            Self::PinkShulkerBox => 225,
            Self::GrayShulkerBox => 226,
            Self::SilverShulkerBox => 227,
            Self::CyanShulkerBox => 228,
            Self::PurpleShulkerBox => 229,
            Self::BlueShulkerBox => 230,
            Self::BrownShulkerBox => 231,
            Self::GreenShulkerBox => 232,
            Self::RedShulkerBox => 233,
            Self::BlackShulkerBox => 234,
            Self::WhiteGlazedTerracotta => 235,
            Self::OrangeGlazedTerracotta => 236,
            Self::MagentaGlazedTerracotta => 237,
            Self::LightBlueGlazedTerracotta => 238,
            Self::YellowGlazedTerracotta => 239,
            Self::LimeGlazedTerracotta => 240,
            Self::PinkGlazedTerracotta => 241,
            Self::GrayGlazedTerracotta => 242,
            Self::SilverGlazedTerracotta => 243,
            Self::CyanGlazedTerracotta => 244,
            Self::PurpleGlazedTerracotta => 245,
            Self::BlueGlazedTerracotta => 246,
            Self::BrownGlazedTerracotta => 247,
            Self::GreenGlazedTerracotta => 248,
            Self::RedGlazedTerracotta => 249,
            Self::BlackGlazedTerracotta => 250,
            Self::Concrete => 251,
            Self::ConcretePowder => 252,
            Self::StructureBlock => 255,
        }
    }
}

type Section<T> = [T; 16 * 16 * 16];
type RawSection = [Block; 16 * 16 * 16];

const MINIMAL_NODE_SIZE: usize = 4;

trait PaletteIndex: Into<usize> + Copy + PartialEq + Eq + Ord + Debug + Send + Sync {}

impl PaletteIndex for u16 {}

impl PaletteIndex for u8 {}


#[derive(Clone, Debug)]
enum Node<T> where T: PaletteIndex {
    Block(T),
    Blocks(Box<[T; MINIMAL_NODE_SIZE * MINIMAL_NODE_SIZE * MINIMAL_NODE_SIZE]>),
    Nodes(Box<[Node<T>; 8]>),
}


#[derive(Debug)]
enum Nodes {
    Large(Box<[Node<u16>; 24]>, BiBTreeMap<u16, Block>),
    Small(Box<[Node<u8>; 24]>, BiBTreeMap<u8, Block>),
}

impl<T: PaletteIndex> Node<T> {
    pub fn new(section: Section<T>) -> Self {
        Self::new_internal(section, 0, 0, 0, 16)
    }

    fn get(&self, pos: LocalBlockPosition) -> T {
        let (x, y, z) = ((pos.c >> 8) & 0x0f, pos.c >> 16, pos.c & 0x0f);
        self.get_block_internal(x as u8, y as u8, z as u8, 16)
    }

    fn new_internal(section: Section<T>, x: usize, y: usize, z: usize, size: usize) -> Self {
        let s = section.as_ref();
        let blocks: Vec<T> = (y..y + size).flat_map(|y| (z..z + size)
            .flat_map(move |z| &s[x + (z << 4) + (y << 8)..(size + x) + (z << 4) + (y << 8)])).copied().collect();

        let first = blocks[0];
        if blocks.iter().all(|b| &first == b) {
            return Self::Block(first);
        }

        if size == MINIMAL_NODE_SIZE {
            return Self::Blocks(blocks.try_into().unwrap());
        }

        let size = size / 2;
        let children: [Node<T>; 8] = std::array::from_fn(|i|
            Self::new_internal(section, x + (i & 1) * size, y + (i >> 2) * size, z + ((i >> 1) & 1) * size, size)
        );

        Self::Nodes(children.into())
    }

    fn get_block_internal(&self, x: u8, y: u8, z: u8, size: u8) -> T {
        match self {
            Self::Block(id) => *id,
            Self::Blocks(blocks) => blocks[x as usize + z as usize * MINIMAL_NODE_SIZE + y as usize * MINIMAL_NODE_SIZE * MINIMAL_NODE_SIZE],
            Self::Nodes(children) => {
                let child_size = size / 2;
                let child_index = (x / child_size) + ((y / child_size) << 2) + ((z / child_size) << 1);
                children[child_index as usize].get_block_internal(x % child_size, y % child_size, z % child_size, child_size)
            }
        }
    }
}


#[derive(Debug)]
struct ChunkData {
    // down -> top
    pub nodes: Nodes,
}

impl From<&[[Block; 16 * 16 * 16]; 24]> for ChunkData {
    fn from(raw: &[[Block; 16 * 16 * 16]; 24]) -> Self {
        let mut blocks = vec![];
        raw.iter().flatten().for_each(|b| if !blocks.contains(b) { blocks.push(*b) });

        fn convert<T: PaletteIndex>(raw: &[[Block; 16 * 16 * 16]; 24], blocks: &BiBTreeMap<T, Block>) -> Box<[Node<T>; 24]> {
            let sections: [Section<T>; 24] = raw.into_par_iter().map(|f|
                f.into_iter().map(|f| *blocks.get_by_right(&f).unwrap()).collect::<Vec<T>>().try_into().unwrap()
            ).collect::<Vec<[T; 4096]>>().try_into().unwrap();

            sections.into_par_iter().map(Node::new).collect::<Vec<Node<T>>>().try_into().unwrap()
        }

        let nodes = if blocks.len() > u8::MAX as usize {
            let blocks = blocks.into_iter().enumerate().map(|f| (f.0 as u16, f.1)).collect::<BiBTreeMap<u16, Block>>();
            Nodes::Large(convert(raw, &blocks), blocks)
        } else {
            let blocks = blocks.into_iter().enumerate().map(|f| (f.0 as u8, f.1)).collect::<BiBTreeMap<u8, Block>>();
            Nodes::Small(convert(raw, &blocks), blocks)
        };

        ChunkData { nodes }
    }
}


#[derive(Default)]
struct BlockPosition {
    x: u32,
    y: u16,
    z: u32,
}

#[derive(Default)]
struct LocalBlockPosition {
    c: u32,
}

impl Debug for LocalBlockPosition {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("LocalBlockPosition {{ x: {}, y: {}, z: {} }}", (self.c >> 8) & 0x0f, self.c >> 16, self.c & 0x0f))
    }
}

impl LocalBlockPosition {
    fn new(x: u32, y: u32, z: u32) -> Self {
        Self { c: y << 16 | x << 8 | z }
    }
}

impl BlockPosition {
    pub fn new(x: u32, y: u32, z: u32) -> Self {
        Self { x, y: y as u16, z }
    }
}

impl From<BlockPosition> for LocalBlockPosition {
    fn from(p: BlockPosition) -> Self {
        Self { c: (p.y as u32) << 16 | p.x << 8 | p.z }
    }
}

fn main() {
    // for file in std::fs::read_dir("./data").unwrap().filter_map(|f| f.ok()) {
    //     if !file.file_name().to_str().unwrap().ends_with(".dat") { continue; }
    //     let mut original = std::fs::read(file.path()).unwrap();
    //     original.reverse();
    //
    //     let name = file.file_name().to_str().unwrap().to_string();
    //     let name = name[..name.len() - 4].to_string() + "_r.dat";
    //     let mut path = file.path().iter().filter(|f| !f.to_str().unwrap().ends_with(".dat")).collect::<PathBuf>();
    //     path.push(name);
    //     let mut new = std::fs::OpenOptions::new().write(true).create(true).truncate(true).open(path).unwrap();
    //
    //     while let Some(a) = original.pop() {
    //         new.write_all(&[0x00, a]).unwrap();
    //     }
    // }
    // let mut a = std::fs::read("./data/0_0.dat").unwrap();
    // a.reverse();
    // let mut b = std::fs::OpenOptions::new().write(true).truncate(true).open("./data/0_0_1.dat").unwrap();
    // while let Some(a) = a.pop() {
    //     b.write_all(&[0x00, a]).unwrap();
    // }

    // let mut s = 0f64;
    // for f in std::fs::read_dir("./data/").unwrap().filter_map(|f| f.ok()) {
    //     let mut a = BufReader::new(std::fs::File::open(f.path()).unwrap());
    //
    //     let mut b = vec![];
    //     while let Ok(v) = a.read_u16::<BigEndian>() {
    //         b.push(v);
    //     }
    //
    //     // let b = std::fs::read(f.path()).unwrap();
    //
    //     let a: Vec<BlockId> = unsafe { std::mem::transmute(b) };
    //
    //     let chunk: [BlockId; 16 * 16 * 16 * 24] = <[BlockId; 16 * 16 * 16 * 24]>::try_from(a).unwrap();
    //     let mut sections = vec![];
    //     for s in chunk.chunks(16 * 16 * 16) {
    //         sections.push(Section::try_from(s).unwrap())
    //     }
    //     let sections: [Section; 24] = <[Section; 24]>::try_from(sections).unwrap();
    //
    //     // let mut a = std::fs::OpenOptions::new().create(true).write(true).truncate(true).open("./test.txt").unwrap();
    //     // a.write_all(serde_json::to_vec(&ChunkData::from(&sections).nodes[0]).unwrap().as_slice()).unwrap();
    //     // println!("{:?}", std::mem::size_of::<Node>());
    //     let size = ChunkData::from(&sections).get_size();
    //     s += f64::from(ChunkData::from(&sections).get_size() as u32);
    // }
    // println!("{:?}", s / 1024.0 / 1024.0);
    //
    let mut a = BufReader::new(std::fs::File::open("./data/7_12.dat").unwrap());

    let mut b = vec![];
    while let Ok(v) = a.read_u16::<BigEndian>() {
        b.push(v);
    }
    let a: Vec<Block> = unsafe { std::mem::transmute(b) };

    let chunk: [Block; 16 * 16 * 16 * 24] = <[Block; 16 * 16 * 16 * 24]>::try_from(a).unwrap();
    let mut sections = vec![];
    for s in chunk.chunks(16 * 16 * 16) {
        sections.push(Section::try_from(s).unwrap())
    }
    let sections: [RawSection; 24] = <[RawSection; 24]>::try_from(sections).unwrap();

    // for x in (0..16).rev() {
    //     println!("{:?}", (0..16).map(|z| sections[0].get(x + z * 16 + 1 * 256).unwrap()).collect::<Vec<_>>());
    // }
    //
    let r = (0..512).into_par_iter().map(|_| {
        let s = Instant::now();
        let chunk = &ChunkData::from(&sections);
        s.elapsed()
    }).collect::<Vec<_>>();
    println!("{:?}", r.iter().map(|f| f.as_nanos()).sum::<u128>() / r.len() as u128);

    // let block = chunk.nodes[0].get(LocalBlockPosition::new(1, 1, 1));
    // println!("{:?}", block);
    // // println!("{:?}", chunk.nodes[0].get(LocalBlockPosition::new(1, 1, 0)));
    // // let node = &chunk.nodes[0];

    // println!("{:?}", node.get(LocalBlockPosition::new(15, 0, 14)));
    // for y in 0..16 {
    //     for x in 0..16 {
    //
    //         println!("{:?}", (0..16).map(|z| node.get(LocalBlockPosition::new(x, y, z))).collect::<Vec<_>>());
    //     }
    //     println!("---\n");
    // }

    // let mut file = std::fs::OpenOptions::new().create(true).write(true).truncate(true).open("./hi.dat").unwrap();
    // let blocks = chunk.nodes.par_iter().flat_map(|n|
    //     (0..16).flat_map(|y| (0..16).flat_map(move |z| (0..16).map(move |x| n.get(LocalBlockPosition::new(x, y, z)))))
    //         .flat_map(|f| (f as u16).to_be_bytes()).collect::<Vec<_>>()
    // ).collect::<Vec<_>>();
    // file.write_all(blocks.as_slice()).unwrap();

    println!("{:?}", std::fs::read("./hi.dat").unwrap() == std::fs::read("./data/7_12.dat").unwrap());
}
