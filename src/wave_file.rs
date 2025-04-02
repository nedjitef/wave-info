use std::fs::File;
use std::io::{Read, Seek};
use std::io::SeekFrom::Current;

#[derive(Debug)]
pub struct ChunkHeader
{
    pub ck_id: String,
    pub ck_size: u32,
    pub data_pointer: u64,
}

#[derive(Debug)]
pub struct RiffHeader
{
    pub chunk_header: ChunkHeader,
    pub riff_type: String,
}

#[derive(Debug)]
pub struct FormatChunk
{
    pub chunk_header: ChunkHeader,
    pub format_tag: u16,
    pub channel_count: u16,
    pub sample_rate: u32,
    pub bytes_per_second: u32,
    pub block_alignment: u16,
    pub bits_per_sample: Option<u16>,
    //pub cbSize: u16,
    //pub extraData: String,
}

#[derive(Debug)]
pub struct BroadcastAudioExtensionChunk
{
    pub chunk_header: ChunkHeader,
    //ckData: String,
    pub description: String,
    pub originator: String,
    pub originator_reference: String,
    pub origination_date: String,
    pub origination_time: String,
    pub time_reference: u64,
    pub version: u16,
    pub umid: [u8; 64],
    pub loudness_value: u16,
    pub loudness_range: u16,
    pub max_true_peak_level: u16,
    pub max_momentary_loudness: u16,
    pub max_short_term_loudness: u16,
    pub coding_history: Option<String>,
}

#[derive(Debug)]
pub struct JunkChunk
{
    chunk_header: ChunkHeader,
    junk: Option<Vec<u8>>,
}

#[derive(Debug)]
pub struct DataChunk
{
    pub chunk_header: ChunkHeader,
}

#[derive(Debug)]
pub struct FactChunk
{
    pub chunk_header: ChunkHeader,
    pub sample_length: u32,
}

#[derive(Debug)]
pub struct WaveFormatExtendedTag
{
    pub chunk_header: ChunkHeader,
    pub format_tag: u16,
    pub channel_count: u16,
    pub sample_rate: u32,
    pub bytes_per_second: u32,
    pub block_alignment: u16,
    pub bits_per_sample: u16,
    pub cb_size: u16,
}

pub fn read_chunk_header(file: &mut File) -> Option<ChunkHeader>
{
    let mut ck_id: Vec<u8> = vec![0u8; 4];
    let mut ck_size: [u8; 4] = [0u8; 4];
    
    let s_ch_id;
    
    match file.read_exact(&mut ck_id)
    {
        Ok(_) => {}
        Err(_) => {return None}
    }
    match file.read_exact(&mut ck_size)
    {
        Ok(_) => {}
        Err(_) => {return None}
    }

    let data_pointer = file.stream_position().unwrap();

    match String::from_utf8(ck_id)
    {
        Ok(v) => { s_ch_id = v }
        Err(_) => {return None}
    }
    
    Some(
        ChunkHeader
        {
            ck_id: s_ch_id,
            ck_size: u32::from_le_bytes(ck_size),
            data_pointer,
        }
    )
}

pub fn read_riff_header(file: &mut File, chunk_header: ChunkHeader) -> RiffHeader
{
    let mut riff_type: Vec<u8> = vec![0u8; 4];
    
    file.read_exact(&mut riff_type).unwrap();

    RiffHeader
    {
        chunk_header,
        riff_type: String::from_utf8(riff_type).unwrap(),
    }
}

pub fn read_fmt_chunk(file: &mut File, chunk_header: ChunkHeader) -> Option<FormatChunk>
{
    let mut format_tag: [u8; 2] = [0u8; 2];
    let mut channel_count: [u8; 2] = [0u8; 2];
    let mut sample_rate: [u8; 4] = [0u8; 4];
    let mut bytes_per_second: [u8; 4] = [0u8; 4];
    let mut block_alignment: [u8; 2] = [0u8; 2];
    file.read_exact(&mut format_tag).unwrap();
    file.read_exact(&mut channel_count).unwrap();
    file.read_exact(&mut sample_rate).unwrap();
    file.read_exact(&mut bytes_per_second).unwrap();
    file.read_exact(&mut block_alignment).unwrap();
        
    let ui_format_tag = u16::from_le_bytes(format_tag);
    let ui_channel_count = u16::from_le_bytes(channel_count);
    let ui_sample_rate = u32::from_le_bytes(sample_rate);
    let ui_bytes_per_second = u32::from_le_bytes(bytes_per_second);
    let ui_block_alignment = u16::from_le_bytes(block_alignment);
    let mut ui_bits_per_sample: Option<u16> = None;
    
    if ui_format_tag == 1
    {
        let mut bits_per_sample: [u8; 2] = [0u8; 2];
        file.read_exact(&mut bits_per_sample).unwrap();
        ui_bits_per_sample = Some(u16::from_le_bytes(bits_per_sample));
    }

    Some(
        FormatChunk
        {
            chunk_header,
            format_tag: ui_format_tag,
            channel_count: ui_channel_count,
            sample_rate: ui_sample_rate,
            bytes_per_second: ui_bytes_per_second,
            block_alignment: ui_block_alignment,
            bits_per_sample: ui_bits_per_sample,
        }
    )
}

pub fn read_bext_chunk(file: &mut File, chunk_header: ChunkHeader)
    -> Option<BroadcastAudioExtensionChunk>
{
    let mut description: Vec<u8> = vec![0u8; 256];
    let mut originator: Vec<u8> = vec![0u8; 32];
    let mut originator_reference: Vec<u8> = vec![0u8; 32];
    let mut origination_date: Vec<u8> = vec![0u8; 10];
    let mut origination_time: Vec<u8> = vec![0u8; 8];
    let mut time_reference: [u8; 8] = [0u8; 8];
    let mut version: [u8; 2] = [0u8; 2];
    let mut umid: [u8; 64] = [0u8; 64];
    let mut loudness_value: [u8; 2] = [0u8; 2];
    let mut loudness_range: [u8; 2] = [0u8; 2];
    let mut max_true_peak_level: [u8; 2] = [0u8; 2];
    let mut max_momentary_loudness: [u8; 2] = [0u8; 2];
    let mut max_short_term_loudness: [u8; 2] = [0u8; 2];
    let mut coding_history: Vec<u8>;

    file.read_exact(&mut description).unwrap();
    file.read_exact(&mut originator).unwrap();
    file.read_exact(&mut originator_reference).unwrap();
    file.read_exact(&mut origination_date).unwrap();
    file.read_exact(&mut origination_time).unwrap();
    file.read_exact(&mut time_reference).unwrap();
    file.read_exact(&mut version).unwrap();
    file.read_exact(&mut umid).unwrap();
    file.read_exact(&mut loudness_value).unwrap();
    file.read_exact(&mut loudness_range).unwrap();
    file.read_exact(&mut max_true_peak_level).unwrap();
    file.read_exact(&mut max_momentary_loudness).unwrap();
    file.read_exact(&mut max_short_term_loudness).unwrap();
    file.seek(Current(180)).unwrap();

    let mut s_coding_history: Option<String> = None;
    let stream_position : u64  = file.stream_position().unwrap();
    let read_until : u64 = chunk_header.data_pointer + (chunk_header.ck_size as u64);
    if read_until - stream_position == 0
    {
        println!("BEXT junk contains no coding history");
    }
    else if read_until - stream_position > 0
    {
        coding_history = vec![0u8; (read_until - stream_position) as usize];
        file.read_exact(&mut coding_history).unwrap();
        s_coding_history = Some(String::from_utf8(coding_history).unwrap())
    }
    else if read_until - stream_position < 0
    {
        println!("BEXT junk read too much data");
    }

    let s_description: String = String::from_utf8(description).unwrap()
        .trim_end_matches("\0").to_string();
    let s_originator: String = String::from_utf8(originator).unwrap()
        .trim_end_matches("\0").to_string();
    let s_originator_reference: String = String::from_utf8(originator_reference)
        .unwrap().trim_end_matches("\0").to_string();
    let s_origination_date: String = String::from_utf8(origination_date)
        .unwrap().trim_end_matches("\0").to_string();
    let s_origination_time: String = String::from_utf8(origination_time)
        .unwrap().trim_end_matches("\0").to_string();
    let ui_time_reference: u64 = u64::from_le_bytes(time_reference);
    let ui_version: u16 = u16::from_le_bytes(version);
    let ui_umid: [u8; 64] = umid;
    let ui_loudness_value: u16 = u16::from_le_bytes(loudness_value);
    let ui_loudness_range: u16 = u16::from_le_bytes(loudness_range);
    let ui_max_true_peak_level: u16 = u16::from_le_bytes(max_true_peak_level);
    let ui_max_momentary_loudness: u16 = u16::from_le_bytes(max_momentary_loudness);
    let ui_max_short_term_loudness: u16 = u16::from_le_bytes(max_short_term_loudness);

    Some(
        BroadcastAudioExtensionChunk
        {
            chunk_header,
            description: s_description,
            originator: s_originator,
            originator_reference: s_originator_reference,
            origination_date: s_origination_date,
            origination_time: s_origination_time,
            time_reference: ui_time_reference,
            version: ui_version,
            umid: ui_umid,
            loudness_value: ui_loudness_value,
            loudness_range: ui_loudness_range,
            max_true_peak_level: ui_max_true_peak_level,
            max_momentary_loudness: ui_max_momentary_loudness,
            max_short_term_loudness: ui_max_short_term_loudness,
            coding_history: s_coding_history,
        }
    )
}

pub fn read_junk_chunk(file: &mut File, chunk_header: ChunkHeader) -> Option<JunkChunk>
{
    file.seek(Current(i64::from(chunk_header.ck_size))).unwrap();

    let mut junk = Vec::with_capacity(chunk_header.ck_size as usize);

    file.read_exact(&mut junk).unwrap();

    Some
    (
        JunkChunk
        {
            chunk_header,
            junk: Some(junk),
        }
    )
}

pub fn read_data_chunk(file: &mut File, chunk_header: ChunkHeader) -> Option<DataChunk> 
{
    Some(
        DataChunk
        {
            chunk_header,
        }
    )
}

pub fn skip_chunk(file: &mut File, chunk_header: ChunkHeader) 
    -> std::io::Result<u64> 
{
    file.seek(Current(i64::from(chunk_header.ck_size)))
}

pub fn read_fact_chunk(file: &mut File, chunk_header: ChunkHeader) -> Option<FactChunk>
{
    let mut slen : [u8; 4] = [0u8; 4];
    file.read_exact(&mut slen).unwrap();

    let sample_length : u32 = u32::from_le_bytes(slen);

    Some
    (
        FactChunk
        {
            chunk_header,
            sample_length,
        }
    )
}
