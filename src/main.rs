//
// MIT License
//
// Copyright (c) 2016, 2018 Yago Mouriño Mendaña <contacto@ylabs.es>
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.
//

extern crate getopts;
extern crate rand;

use getopts::Options;
use rand::Rng;
use std::env;
use std::fs;
use std::process::Command;

fn usage(opts: Options) {
    // Get the current executable name, without the path.
    let prog = env::current_exe().expect("ERROR: can't get current exe.");
    let prog = prog.as_path()
        .file_name()
        .expect("ERROR: can't get current exe.")
        .to_str()
        .unwrap();

    // Show the usage message.
    let usage = format!("Usage: {} [options]", prog);
    println!("{}", opts.usage(&usage));
}

fn main() {
    // Prepare program options...
    let mut opts = Options::new();

    opts.optflag("h", "help", "prints this help screen");
    opts.optopt("i", "input", "defines the input directory", "DIR");
    opts.optopt("c", "count", "how many files should we get?", "COUNT");
    opts.optflag("p", "play", "should we play the selected file?");

    // ...and parse them.
    let parsed = opts.parse(env::args())
        .expect("ERROR: can't parse arguments.");

    // Is the usage message requested?
    if parsed.opt_present("h") {
        usage(opts);
        return;
    }

    // Get the input directory.
    let directory = parsed.opt_str("i");
    let directory = directory.as_ref().map(String::as_ref).unwrap_or(".");

    // Get the contents of that directory...
    let paths = fs::read_dir(directory).expect("ERROR: can't read directory.");

    // ...and filter them to get only multimedia files.
    let mut entries: Vec<_> = paths
        .filter(|entry| {
            entry.as_ref().unwrap().path().is_file()
                && (entry.as_ref().unwrap().path().extension().unwrap() == "mp4"
                    || entry.as_ref().unwrap().path().extension().unwrap() == "avi"
                    || entry.as_ref().unwrap().path().extension().unwrap() == "mov"
                    || entry.as_ref().unwrap().path().extension().unwrap() == "mkv"
                    || entry.as_ref().unwrap().path().extension().unwrap() == "flv"
                    || entry.as_ref().unwrap().path().extension().unwrap() == "wmv"
                    || entry.as_ref().unwrap().path().extension().unwrap() == "mpg"
                    || entry.as_ref().unwrap().path().extension().unwrap() == "mpeg"
                    || entry.as_ref().unwrap().path().extension().unwrap() == "flac"
                    || entry.as_ref().unwrap().path().extension().unwrap() == "mp3"
                    || entry.as_ref().unwrap().path().extension().unwrap() == "wav"
                    || entry.as_ref().unwrap().path().extension().unwrap() == "ogg"
                    || entry.as_ref().unwrap().path().extension().unwrap() == "jpg")
        })
        .collect();

    // How many files are in the collection?
    let entries_count = entries.len();

    // Avoid errors if there are 0 files.
    if entries_count > 0 {
        // Get how many files the user wants.
        let count = parsed.opt_str("c");
        let count = count.as_ref().map(String::as_ref).unwrap_or("1");
        let count = count.parse::<i32>().expect("ERROR: can't count files.");

        if count == 1 {
            // Get a random file.
            let selected = rand::thread_rng().gen_range(0, entries_count);
            let selected = entries[selected].as_ref().unwrap().path();

            // Should we play it?
            if parsed.opt_present("p") {
                Command::new("vlc")
                    .arg(selected.to_str().unwrap())
                    .spawn()
                    .expect("Command Not Found");
            } else {
                // If not, show its name.
                println!("{}", selected.display());
            }
        } else if count > 1 && count < 6 {
            for i in 0..count {
                let selected = rand::thread_rng().gen_range(0, entries_count - i as usize);
                let selected = entries.remove(selected).unwrap().path();
                println!("{}", selected.display());
            }
        } else {
            println!("ERROR: 5 files max.");
        }
    } else {
        println!("ERROR: files not found.");
    }
}
