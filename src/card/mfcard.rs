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
use serde_json;
use std::collections::HashMap;
use std::fmt::Write;

use super::Card;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[allow(non_snake_case)]
pub struct MifareClassicFile {
    pub Card: MifareClassicCard,
    pub blocks: HashMap<String, String>,
    pub SectorKeys: HashMap<String, MifareClassicSectorKeys>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[allow(non_snake_case)]
pub struct MifareClassicCard {
    pub UID: String,
    pub ATQA: String,
    pub SAK: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[allow(non_snake_case)]
pub struct MifareClassicSectorKeys {
    pub KeyA: String,
    pub KeyB: String,
    pub AccessConditions: String,
    // AccessConditionsText omitted
}

fn prettify_hex(hex: &str) -> String {
    let mut out = String::new();
    for (i, c) in hex.chars().enumerate() {
        if i % 2 == 0 {
            out.push(' ');
        }
        out.push(c);
    }
    out.trim().to_owned()
}

impl MifareClassicFile {
    fn block_to_fblock(&self) -> String {
        let mut fblock = String::new();
        let mut block_vec: Vec<_> = self.blocks.iter().collect();
        block_vec.sort_by(|a, b| {
            let v_a: i32 = a.0.parse().unwrap_or(-1);
            let v_b: i32 = b.0.parse().unwrap_or(-1);
            v_a.cmp(&v_b)
        });

        for blk in block_vec {
            writeln!(fblock, "Block {}: {}", blk.0, prettify_hex(blk.1)).unwrap();
        }
        fblock.trim().to_owned()
    }

    fn get_cardtype(&self) -> String {
        match self.blocks.len() * 16 {
            1024 => "1K".to_string(),
            2048 => "2K".to_string(),
            4096 => "4K".to_string(),
            _ => "UNKNOWN".to_string(),
        }
    }
}

impl Card for MifareClassicFile {
    fn from_json(json: &str) -> Result<Self, crate::error::CardError>
    where
        Self: Sized,
    {
        let file: MifareClassicFile = serde_json::from_str(json)?;
        Ok(file)
    }

    fn to_json(&self) -> Result<String, crate::error::CardError> {
        let json = serde_json::to_string_pretty(self)?;
        Ok(json)
    }

    fn from_flipper(nfc: &str) -> Result<Self, crate::error::CardError>
    where
        Self: Sized,
    {
        todo!()
    }

    fn to_flipper(&self) -> Result<String, crate::error::CardError> {
        let flipper_file = format!(
            r##"Filetype: Flipper NFC device
Version: 2
# Nfc device type can be UID, Mifare Ultralight, Mifare Classic, Bank card
Device type: Mifare Classic
# UID, ATQA and SAK are common for all formats
UID: {}
ATQA: {}
SAK: {}
# Mifare Classic specific data
Mifare Classic type: {}
Data format version: 2
# Mifare Classic blocks
{}
"##,
            prettify_hex(&self.Card.UID),
            prettify_hex(&self.Card.ATQA),
            prettify_hex(&self.Card.SAK),
            self.get_cardtype(),
            self.block_to_fblock()
        );

        Ok(flipper_file)
    }
}
