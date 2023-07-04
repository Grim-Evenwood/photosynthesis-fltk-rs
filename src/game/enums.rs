use std::fmt::Display;

#[allow(dead_code)]
#[derive(Debug, PartialEq, Eq, Copy, Clone, Ord, PartialOrd, Hash)]
#[doc = r"# PieceType
This enum represents the type of piece that might be placed on a board,
such that each type would need to be handled differently."]
pub enum PieceType {
    Empty,
    Tree,
    Animal,
    Dam,
    Moonstone,
    GreatElderTree,
}//end enum PieceType

impl Default for PieceType {
    fn default() -> Self {
        PieceType::Empty
    }//end default()
}//end impl Default for PieceType

impl Display for PieceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PieceType::Empty => write!(f, "Empty"),
            PieceType::Tree => write!(f, "Tree"),
            PieceType::Animal => write!(f, "Animal"),
            PieceType::Dam => write!(f, "Beaver Dam"),
            PieceType::Moonstone => write!(f, "Moonstone"),
            PieceType::GreatElderTree => write!(f, "Great Elder Tree"),
        }//end matching self
    }//end fmt(&self, f)
}//end impl Display for PieceType

#[allow(dead_code)]
#[derive(Debug, PartialEq, Eq, Copy, Clone, Ord, PartialOrd, Hash)]
#[doc = r"# Animal
This enum represents the type of forest animal.
Each forest animal has its own special ability to be used by the owning player."]
pub enum Animal {
    Boar,
    Hedgehog,
    Squirrel,
    Fox,
    Owl,
    Badger,
    Beaver,
    Turtle,
}//end enum Animal

impl Display for Animal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Animal::Boar => write!(f, "Boar"),
            Animal::Hedgehog => write!(f, "Hedgehog"),
            Animal::Squirrel => write!(f, "Squirrel"),
            Animal::Fox => write!(f, "Fox"),
            Animal::Owl => write!(f, "Owl"),
            Animal::Badger => write!(f, "Badger"),
            Animal::Beaver => write!(f, "Beaver"),
            Animal::Turtle => write!(f, "Turtle"),
        }//end matching self
    }//end fmt(&self, f)
}//end impl Display for Animal

#[allow(dead_code)]
#[derive(Debug, PartialEq, Eq, Copy, Clone, Ord, PartialOrd, Hash)]
#[doc = r"# TreeSize
This enum represents the size of a tree.
Note that for game purposes, a seed counts as a tree, and is the smallest size."]
pub enum TreeSize {
    Seed,
    Small,
    Medium,
    Large,
}//end enum TreeSize

#[allow(dead_code)]
impl TreeSize {
    pub fn size(&self) -> usize {
        match self {
            TreeSize::Seed => 0,
            TreeSize::Small => 1,
            TreeSize::Medium => 2,
            TreeSize::Large => 3,
        }//end matching self
    }//end size()
}//end impl for TreeSize

impl Default for TreeSize {
    fn default() -> Self {
        TreeSize::Seed
    }//end default()
}//end impl Default for TreeSize

impl Display for TreeSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TreeSize::Seed => write!(f, "Seed"),
            TreeSize::Small => write!(f, "Small"),
            TreeSize::Medium => write!(f, "Medium"),
            TreeSize::Large => write!(f, "Large"),
        }//end matching self
    }//end fmt(&self, f)
}//end impl Display for TreeSize

#[allow(dead_code)]
#[derive(Debug, PartialEq, Eq, Copy, Clone, Ord, PartialOrd, Hash)]
#[doc = r"# Fertility
This enum represents the fertility of a spot on the board.
This is mostly only relevant for determining harvesting points,
and is only kept as an enum to make things connect better for those purposes."]
pub enum Fertility {
    OneLeaf,
    TwoLeaf,
    ThreeLeaf,
    FourLeaf,
}//end enum Fertility

#[allow(dead_code)]
impl Fertility {
    pub fn as_int(&self) -> i32 {
        match self {
            Fertility::OneLeaf => 1,
            Fertility::TwoLeaf => 2,
            Fertility::ThreeLeaf => 3,
            Fertility::FourLeaf => 4,
        }//end matching self
    }//end as_int(&self)
    pub fn as_usize(&self) -> usize {
        match self {
            Fertility::OneLeaf => 1,
            Fertility::TwoLeaf => 2,
            Fertility::ThreeLeaf => 3,
            Fertility::FourLeaf => 4,
        }//end matching self
    }//end as_usize(&self)
}//end impl Fertility

impl Display for Fertility {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Fertility::OneLeaf => write!(f, "One-Leaf"),
            Fertility::TwoLeaf => write!(f, "Two-Leaf"),
            Fertility::ThreeLeaf => write!(f, "Three-Leaf"),
            Fertility::FourLeaf => write!(f, "Four-Leaf"),
        }//end matching self
    }//end fmt(&self, f)
}//end impl Display for Fertility