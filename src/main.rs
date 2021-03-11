use std::{sync::{mpsc::channel, Arc, Condvar, Mutex}, thread::{self, JoinHandle}, time::Duration};

use clap::Clap;

use decrypter::Decrypter;
use encrypter::Encrypter;
use infra::generic_error::GenericError;

mod common;
mod decrypter;
mod encrypter;
mod infra;

#[derive(Clap)]
struct CommandLineArguments {
    #[clap(short, long)]
    string_length: usize,
    #[clap(short, long)]
    decrypters_count: u32,
    #[clap(short, long, default_value="18446744073709551615")]
    timeout_sec: u64
}

fn main() {
    let args = CommandLineArguments::parse();
    let (channel_sender, channel_reciever) = channel();
    let arc = Arc::new((Mutex::new(Option::<Vec<u8>>::None), Condvar::new()));

    let mut encrypter = Encrypter::new(args.string_length, channel_reciever, arc.clone(), Duration::from_secs(args.timeout_sec)).unwrap();
    let mut threads: Vec<JoinHandle<Result<(), GenericError>>> = (0..args.decrypters_count)
        .map(|i| {
            Decrypter::new(i, args.string_length, channel_sender.clone(), arc.clone()).unwrap()
        })
        .map(|decrypter| thread::spawn(move || decrypter.run()))
        .collect();

    threads.push(thread::spawn(move || encrypter.run()));

    for thread in threads {
        thread.join().unwrap().unwrap();
    }
}
