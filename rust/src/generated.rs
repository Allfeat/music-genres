// This file is part of Allfeat.

// Copyright (C) 2022-2025 Allfeat.
// SPDX-License-Identifier: GPL-3.0-or-later

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.
// AUTO-GENERATED FILE. DO NOT EDIT MANUALLY.

use parity_scale_codec::{Decode, DecodeWithMemTracking, Encode, MaxEncodedLen};
use scale_info::TypeInfo;
use sp_runtime::RuntimeDebug;

#[cfg(feature = "js")]
use serde::{Deserialize, Serialize};
#[cfg(feature = "js")]
use wasm_bindgen::prelude::*;

/// Flat enum containing all main genres and subgenres.
/// Subgenres are grouped under genre-level comments.
/// This enum is used directly in the blockchain to identify any genre type.
#[derive(
    Clone,
    Copy,
    RuntimeDebug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Encode,
    Decode,
    DecodeWithMemTracking,
    TypeInfo,
    MaxEncodedLen,
)]
#[cfg_attr(feature = "js", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "js", wasm_bindgen)]
pub enum GenreId {
    // ===== Genre: Rock =====
    Rock,
    HardRock,
    ClassicRock,
    PunkRock,
    AlternativeRock,
    IndieRock,
    ProgressiveRock,
    GarageRock,
    PostRock,
    MathRock,
    Grunge,

    // ===== Genre: Pop =====
    Pop,
    DancePop,
    Electropop,
    Synthpop,
    TeenPop,
    KPop,
    BaroquePop,
    ArtPop,

    // ===== Genre: Hip Hop / Rap =====
    HipHop,
    Trap,
    BoomBap,
    GangstaRap,
    ConsciousRap,
    LofiHipHop,
    Drill,
    CloudRap,
    ExperimentalHipHop,

    // ===== Genre: Electronic =====
    Electronic,
    House,
    Techno,
    Trance,
    Dubstep,
    DrumAndBass,
    ElectronicAmbient,
    Idm,
    Electro,
    Downtempo,
    Breakbeat,
    Hardstyle,
    Triphop,

    // ===== Genre: R&B / Soul =====
    RAndB,
    ContemporaryRAndB,
    NeoSoul,
    Motown,
    Funk,
    QuietStorm,
    BlueEyedSoul,

    // ===== Genre: Jazz =====
    Jazz,
    Bebop,
    Swing,
    CoolJazz,
    Fusion,
    AcidJazz,
    LatinJazz,
    VocalJazz,

    // ===== Genre: Classical =====
    Classical,
    Baroque,
    ClassicalPeriod,
    Romantic,
    ContemporaryClassical,
    Opera,
    ChamberMusic,
    Electroacoustic,

    // ===== Genre: Country =====
    Country,
    Bluegrass,
    OutlawCountry,
    AltCountry,
    ContemporaryCountry,
    CountryPop,
    HonkyTonk,

    // ===== Genre: Latin =====
    Latin,
    Reggaeton,
    Salsa,
    Bachata,
    LatinRock,
    Cumbia,
    Merengue,
    Tango,

    // ===== Genre: Reggae =====
    Reggae,
    RootsReggae,
    Dancehall,
    Dub,
    Ska,
    Rocksteady,

    // ===== Genre: Metal =====
    Metal,
    HeavyMetal,
    ThrashMetal,
    DeathMetal,
    BlackMetal,
    DoomMetal,
    PowerMetal,
    Metalcore,
    SymphonicMetal,
    ProgressiveMetal,

    // ===== Genre: Folk =====
    Folk,
    FolkRock,
    TraditionalFolk,
    IndieFolk,
    ProgressiveFolk,

    // ===== Genre: World =====
    World,
    Afrobeat,
    Highlife,
    Brazilian,
    Flamenco,
    Celtic,
    Bharatnatyam,
    Gamelan,
    Fado,

    // ===== Genre: Soundtrack / Score =====
    Soundtrack,
    FilmScore,
    VideoGameMusic,
    MusicalSoundtrack,
    TelevisionScore,

    // ===== Genre: Experimental / Avant‑Garde =====
    Experimental,
    Noise,
    MusiqueConcrete,
    Glitch,
    Minimalism,
    ElectroacousticExperimental,

    // ===== Genre: Punk =====
    Punk,
    HardcorePunk,
    PostPunk,
    SkaPunk,
    CrustPunk,

    // ===== Genre: Gospel / Christian =====
    GospelChristian,
    Gospel,
    ContemporaryChristian,
    ChristianRock,

    // ===== Genre: Blues =====
    Blues,
    DeltaBlues,
    ElectricBlues,
    UrbanBlues,
    BluesRock,

    // ===== Genre: Ambient =====
    Ambient,
    DarkAmbient,
    SpaceAmbient,
    AmbientNewAge,

    // ===== Genre: New Age =====
    NewAge,
    Meditation,
    Relaxation,
    Healing,
}
