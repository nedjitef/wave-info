mod wave_file;

use std::{env, fs};
use std::fs::File;
use std::process::exit;

use gtk4::glib;
use gtk4::prelude::*;

use crate::wave_file::{
    skip_chunk,
    BroadcastAudioExtensionChunk, DataChunk, FactChunk, FormatChunk,
    JunkChunk, RiffHeader,
};

fn main()
{
    let args: Vec<String> = env::args().collect();
    let filename : &String = &args[1];
    let file = File::open(filename).unwrap();
    let chunks = get_file_chunks(file);
    let riff_header = chunks.riff_header;
    let bext_chunk = chunks.broadcast_audio_extension_chunk;
    let data_chunk = chunks.data_chunk;
    let fact_chunk = chunks.fact_chunk;
    let fmt_chunk = chunks.format_chunk;
    let junk_chunk = chunks.junk_chunk;

    println!("RIFF {:?}", riff_header);
    println!("BEXT {:?}", bext_chunk);
    println!("DATA {:?}", data_chunk);
    println!("FACT {:?}", fact_chunk);
    println!("FMT  {:?}", fmt_chunk);
    println!("JUNK {:?}", junk_chunk);
    //println!("Chunks: {:?}", chunks);
}

#[derive(Debug)]
struct AllChunks
{
    broadcast_audio_extension_chunk: Option<BroadcastAudioExtensionChunk>,
    data_chunk: Option<DataChunk>,
    format_chunk: Option<FormatChunk>,
    fact_chunk: Option<FactChunk>,
    riff_header: Option<RiffHeader>,
    junk_chunk: Option<JunkChunk>,
}

fn get_file_chunks(mut file: File) -> AllChunks
{
    /*
    let args: Vec<_> = env::args().collect();
    if args.len() < 2
    {
        println!("Not enough arguments");
        exit(1);
    }
    println!("First Argument {}", args[1]);
    let path =fs::canonicalize(&args[1]).unwrap();
    println!("Path: {:?}", path);
    let mut file = File::open(path).unwrap();
     */
    //println!("File: {:?}", file);

    let riff_chunk_header = wave_file::read_chunk_header(&mut file).unwrap();
    println!("{:?}", riff_chunk_header);

    if riff_chunk_header.ck_id != "RIFF" && riff_chunk_header.ck_id != "RF64"
    {
        println!("No RIFF file");
        exit(2);
    }

    let riff_header = wave_file::read_riff_header(&mut file, riff_chunk_header);
    println!("{:?}", riff_header);

    if riff_header.riff_type != "WAVE"
    {
        println!("Riff Type: {:?}\nno Wave", riff_header.riff_type);
        exit(3);
    }

    let mut bext_chunk: Option<BroadcastAudioExtensionChunk> = None;
    let mut data_chunk: Option<DataChunk> = None;
    let mut fact_chunk: Option<FactChunk> = None;
    let mut fmt_chunk: Option<FormatChunk> = None;
    let mut junk_chunk: Option<JunkChunk> = None;

    loop
    {
        let chunk_header;
        match wave_file::read_chunk_header(&mut file) 
        {
            Some(h) => chunk_header = h,
            None => break,
        }
        //println!("{:?}", chunk_header);
        match chunk_header.ck_id.as_str().to_lowercase().as_str()
        {
            "fmt " =>
                {
                    fmt_chunk =
                        wave_file::read_fmt_chunk(&mut file, chunk_header);
                },
            "junk" =>
                {
                    junk_chunk =
                        wave_file::read_junk_chunk(&mut file, chunk_header);
                },
            "data" =>
                {
                    data_chunk =
                        wave_file::read_data_chunk(&mut file, chunk_header);
                },
            "bext" =>
                {
                    bext_chunk =
                        wave_file::read_bext_chunk(&mut file, chunk_header);
                },
            "fact" =>
                {
                    fact_chunk =
                        wave_file::read_fact_chunk(&mut file, chunk_header);
                }
            &_ => _ = 
                {
                    println!("Unknown chunk: {:?}", chunk_header);
                    skip_chunk(&mut file, chunk_header)
                },
        }
    }
    
    AllChunks
    {
        riff_header: Some(riff_header),

        broadcast_audio_extension_chunk: bext_chunk,
        data_chunk,
        format_chunk: fmt_chunk,
        fact_chunk,
        junk_chunk,
    }
}
