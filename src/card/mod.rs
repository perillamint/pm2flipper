/*
 * SPDX-FileCopyrightText: 2022 perillamint
 *
 * SPDX-License-Identifier: GPL-3.0-or-later
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful, but
 * WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
 * General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program. If not, see <http://www.gnu.org/licenses/>.
 */

use serde::{Deserialize, Serialize};

pub mod mfcard;
pub mod mfu;

/// Proxmark3 JSON header
#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct CardJson {
    Created: String,
    FileType: String,
}

/// Card type enum
pub enum CardType {
    MifareClassic(mfcard::MifareClassicFile),
    MifareUltralight(mfu::MFUFile),
    Default(),
}

pub trait Card {
    fn from_json(json: &str) -> Result<Self, crate::error::CardError>
    where
        Self: Sized;
    fn to_json(&self) -> Result<String, crate::error::CardError>;
    fn from_flipper(nfc: &str) -> Result<Self, crate::error::CardError>
    where
        Self: Sized;
    fn to_flipper(&self) -> Result<String, crate::error::CardError>;
}
