//use std::f32::consts::PI;
use std::i16;
use hound;
use std::fs::File;
use std::io::{Write, BufWriter};
//use std::fmt;
use std::env;
use std::process;
use csv;
use serde::Deserialize;
use std::error::Error;

#[derive(Deserialize)]
struct Row {
    t: String,
    val: String,
}


fn read_csv() -> Result<Vec<i16>, Box<dyn Error>> {
    let mut sample_data: Vec<i16> = Vec::new();

    let mut sample_csv = csv::Reader::from_path("sample.csv").unwrap();
    for result in sample_csv.records() {
        let rdata = result?;
        //println!("{:?}", rdata);
        let rrow : Row = rdata.deserialize(None).unwrap();
        //println!("{:?}", rrow.val.trim());
        let rval : i16 = rrow.val.trim().parse().unwrap();
        sample_data.push(rval);
        //let rt:u32 = rdata[0].parse()?;
        //let rval:i16 = rdata[1].parse()?;
        //sample_data.push(row.val);
        //println!("{:?}, {:?}", rt, rval);
    }
    Ok(sample_data)
}

fn main(){
    let args: Vec<String> = env::args().collect();
    if env::args().count() < 2
    {
        println!("{} sample_rate[Hz]", args[0]);
        process::exit(1);
    }

    let sr : u32 = args[1].parse().unwrap();

    //println!("{:?}", read_csv().unwrap());
    let samples = read_csv().unwrap();

    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: sr,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };

    let mut writer = hound::WavWriter::create("wav.wav",spec).unwrap();

    // 'as' is type casting
    for val in samples/*.map(|x| x as f32 / 44100.0)*/ {
        //let amplitude = i16::MAX as f32;
        writer.write_sample(val as i16).unwrap();
    }

    writer.finalize().unwrap();
}
