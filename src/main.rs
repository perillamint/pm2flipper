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

use crate::card::Card;
use clap::Parser;
use std::{fs::File, io::Read};

mod card;
mod error;

#[derive(clap::Parser)]
#[clap(about, version, author)]
struct Args {
    #[clap(value_parser, value_name = "FILE")]
    infile: String,
}

fn main() {
    let args: Args = Args::parse();

    let mut cardfile = File::open(args.infile).unwrap();
    let mut buf = "".to_owned();
    cardfile.read_to_string(&mut buf).unwrap();

    let card = card::mfcard::MifareClassicFile::from_json(&buf).unwrap();
    print!("{}", card.to_flipper().unwrap());
}
