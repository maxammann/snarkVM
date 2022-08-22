// Copyright (C) 2019-2022 Aleo Systems Inc.
// This file is part of the snarkVM library.

// The snarkVM library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The snarkVM library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with the snarkVM library. If not, see <https://www.gnu.org/licenses/>.


use std::panic;
use std::process::abort;
use once_cell::sync::OnceCell;
use snarkvm::circuit::Parser;
use snarkvm::compiler::{Program};
use snarkvm::prelude::{circuit, Environment, Input, PrivateKey, test_crypto_rng, Testnet3, VM};
use std::str::FromStr;
use arbitrary::Arbitrary;

pub fn harness(buf: &[u8]) {
    if let Ok(s) = std::str::from_utf8(buf) {
        let result = panic::catch_unwind(|| {
            if let Ok(program) = Program::<FuzzNetwork>::from_str(&s) {
                fuzz_program(program);
            }
        });

        if let Err(err) = result {
            if let Some(str) = err.downcast_ref::<&str>() {
                if *str != "HaltedABC" {
                    abort();
                }
            } else {
                abort();
            }
        }
    }

    /*            let mut unstructured = Unstructured::new(buf);

                if let Ok(program) = Program::<CurrentNetwork>::arbitrary(&mut unstructured) {

                } else {

                }*/
}

static INSTANCE: OnceCell<VM<FuzzNetwork>> = OnceCell::new();

pub type FuzzNetwork = Testnet3;

pub fn init_vm() -> &'static VM<FuzzNetwork> {
    INSTANCE.get_or_init(|| VM::<FuzzNetwork>::new().unwrap())
}

pub fn deploy(program: &Program<FuzzNetwork>) {
    let vm = init_vm();
    let rng = &mut test_crypto_rng();

    if let Ok(deployment) = vm.deploy(&program, rng) {
        vm.verify_deployment(&deployment);
    }
}

pub fn execute(program: &Program<FuzzNetwork>) {
    let vm = init_vm();
    let rng = &mut test_crypto_rng();

    if let Some(f) = program.functions().first() {
        let pkey = PrivateKey::new(rng).unwrap();

        //let inputs = f.1.inputs().iter().map(|_| Input::arbitrary()).collect::<Vec<_>>();

        let inputs = [];

        if let Ok(auth) = vm.authorize(&pkey, program.id(), f.0.clone(), &inputs, rng) {
            vm.execute(auth, rng); // ignore unwrap
        } else {
            // ignore
        }
    }
}
pub fn fuzz_program(program: Program<FuzzNetwork>) {
    deploy(&program);
    execute(&program);
}
