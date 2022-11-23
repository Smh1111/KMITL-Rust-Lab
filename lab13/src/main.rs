
use std::io::BufReader;
use std::fs::File;
use std::io::BufRead;
use std::io::Read;
use std::io::prelude::*;

#[derive(Debug)]
struct GPS{
    lat:    f64,
    long:   f64,
}



fn read_file_line_by_line(filepath: &str) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open(filepath)?;
    let reader = BufReader::new(file);
    
    let mut i = 0;
    
    let temp: String = "".to_string();

    for line in reader.lines() {
        
        println!("line {}: {}", i, line?);
        i += 1; 
    }

    println!("{:?}", temp);
    Ok(())


}

fn remove(start: usize, stop: usize, s: &str) -> String {
    // start = 20, end = 49
    let mut rslt = "".to_string();
    for (i, c) in s.chars().enumerate() {
        if start > i || stop < i + 1 {
            rslt.push(c);
        }
    }
    rslt
}

fn mean(latt_v:Vec<f64>, long_v:Vec<f64>) -> (f64, f64){
    let mut totatl_latt = 0.0;
    
    let mut totatl_long = 0.0;

    let mut lenght_latt = latt_v.len() as f64;
    let mut lenght_long = long_v.len() as f64;

    println!("{} {}", lenght_latt, lenght_long);

    for i in &latt_v
    {
        totatl_latt += i;
        
    }
    
    for j in &long_v
    {
        totatl_long += j;
        
    }
    let latt_mean = totatl_latt/ lenght_latt;
    let long_mean = totatl_long / lenght_long;
    (latt_mean, long_mean)

}

fn standard_deviation(latt_v:Vec<f64>, long_v:Vec<f64>, mean:(f64, f64)) -> (f64, f64){

    let (mean_lat, mean_long) = mean;

    let mut total_lat = 0.0;
    let mut total_long = 0.0;
    for i in 0..latt_v.len()
    {
        total_lat +=  (latt_v[i] - mean_lat) * (latt_v[i] - mean_lat);
    }

    for j in 0..long_v.len()
    {
        total_long +=  (long_v[j] - mean_long) * (long_v[j] - mean_long);
    
    }
    let SD_lat = total_lat / latt_v.len() as f64;
    let SD_long = total_long / long_v.len() as f64;

    //println!("{}", SD_lat);
    //println!("{}", SD_lat.sqrt());

    //println!("{}", SD_long);
    //println!("{}", SD_long.sqrt());

    (SD_lat.sqrt(), SD_long.sqrt())
}

fn meters(st_dev_lat:f64, st_dev_long:f64) -> (f64, f64)
{
    let meter_lat = st_dev_lat * (111139 as f64);
    let meter_long = st_dev_lat * (107963 as f64);
    (meter_lat, meter_long)
}

fn min(latt_v:Vec<f64>, long_v:Vec<f64>) -> (f64, f64){
    let mut min_latt = latt_v[0];
    let mut min_long = long_v[0];

    for i in latt_v
    {
        if i < min_latt
        {
            min_latt = i;
        }
    }

    for j in long_v
    {
        if j < min_long
        {
            min_long = j;
        }
    }

    (min_latt, min_long)
}
fn max(latt_v:Vec<f64>, long_v:Vec<f64>) -> (f64, f64){
    let mut max_latt = latt_v[0];
    let mut max_long = long_v[0];

    for i in latt_v
    {
        if i > max_latt
        {
            max_latt = i;
        }
    }

    for j in long_v
    {
        if j > max_long
        {
            max_long = j;
        }
    }

    (max_latt, max_long)
}

fn count_latt(v:Vec<f64>, min:f64, max:f64) -> (Vec<f64> , Vec<i32>)
{
   
    let mut start = min - 0.000004;
    let stepsize = 0.00001; 
    
    
    let mut new_vec = vec![];

    new_vec.push((start * 100000.0).round() / 100000.0);
    
    while start <= max
    {
        
        start = start + stepsize;
        let y = (start * 100000.0).round() / 100000.0;
        new_vec.push(y);
    }
    let mut count_v = vec![];

    let mut i = 0;
    

    while i != new_vec.len() - 1
    {
        
        let mut flag = 0;
        for j in &v
        {
            if j >= &new_vec[i] && j <= &new_vec[i+1]
            {
                flag += 1;
            }
        }
        
        count_v.push(flag);

        i += 1;
    }

   
    let mut i = 0;
    while (i != count_v.len())
    {
        let mut flag = 0;
        let mut result = "".to_string();

        let mut j = count_v[i];
        while (j != 0)
        {
            result.push_str("*");
            j -= 1;
        }
        
        println!("{:.5} {}", new_vec[i], result);
        i += 1;
    }
    println!("{:.6}", new_vec[new_vec.len() - 1]);
    
    (new_vec, count_v)
    
}

fn count_long(v:Vec<f64>, min:f64, max:f64) -> (Vec<f64> , Vec<i32>)
{
   
    let mut start = min - 0.000004;
    let stepsize = 0.00001; 
    
    
    let mut new_vec = vec![];

    new_vec.push((start * 100000.0).round() / 100000.0);
    
    while start <= max
    {
        
        start = start + stepsize;
        let y = (start * 100000.0).round() / 100000.0;
        new_vec.push(y);
    }
    let mut count_v = vec![];

    let mut i = 0;
    

    while i != new_vec.len() - 1
    {
        
        let mut flag = 0;
        for j in &v
        {
            if j >= &new_vec[i] && j <= &new_vec[i+1]
            {
                flag += 1;
            }
        }
        
        count_v.push(flag);

        i += 1;
    }

   
    let mut i = 0;
    while (i != count_v.len())
    {
        let mut flag = 0;
        let mut result = "".to_string();

        let mut j = count_v[i];
        while (j != 0)
        {
            result.push_str("*");
            j -= 1;
        }
        
        println!("{:.5} {}", new_vec[i], result);
        i += 1;
    }
    println!("{:.6}", new_vec[new_vec.len() - 1]);
    
    (new_vec, count_v)
    
}
fn main() {
    //read_file_line_by_line("GPSA.csv");

    let mut file = File::open("GPSA.csv").expect("File not found");
    let mut data = String::new();
    file.read_to_string(&mut data).expect("Error while reading file");

    //println!("{}", data);

    let mut v = vec![];
    for line in data.lines()
    {
        let result = remove(21, line.len(), line);
        let strings: Vec<&str> = result.split(", ").collect();
        //println!("{:?}", strings);
        v.push(
            GPS
            {
                lat: strings[0].parse::<f64>().unwrap(),
                long: strings[1].parse::<f64>().unwrap(),
            }
        );
    }
 
    let mut lat = vec![];
    let mut long = vec![];

    let mut lat1 = vec![];
    let mut long1 = vec![];

    let mut lat2 = vec![];
    let mut long2 = vec![];

    let mut lat3 = vec![];
    let mut long3 = vec![];

    // Lab 13
    let mut lat4 = vec![];
    let mut long4 = vec![];


    //__________________________
    for i in v
    {
        lat.push(i.lat);
        long.push(i.long);


        lat1.push(i.lat);
        long1.push(i.long);

        lat2.push(i.lat);
        long2.push(i.long);

        lat3.push(i.lat);
        long3.push(i.long);
        
        lat4.push(i.lat);
        long4.push(i.long);

    }
    println!("{:?}", lat);
    println!("{:?}", long);

    
    let mean = mean(lat, long); // lat and long vec gone after this

    println!("mean = {:?}", mean);
    // println!("{:?}", lat1);
    // println!("{:?}", long1);
    let st_dev = standard_deviation(lat1, long1, mean);

    println!("{:#?}", st_dev);
    let (st_dev_latt, st_dev_long) =  st_dev;

    //println!("{:?} {:?}", st_dev_latt, st_dev_long);
    let (meter_latt, meter_long) = meters(st_dev_latt, st_dev_long);
    println!("Latt: {:?} meter, Long: {:?} meter", meter_latt, meter_long);

    let (min_latt, min_long) = min(lat2, long2);
    let (max_latt, max_long) = max(lat3, long3);

    println!("Min latt: {:?}, Min long: {:?}", min_latt, min_long);
    println!("Max latt: {:?}, Max long: {:?}", max_latt, max_long);

    let bin_size_latt = max_latt - min_latt;
    let bin_size_long = max_long - min_long;

    println!("bin_size_latt: {:?}, bin_size_long: {:?}", bin_size_latt, bin_size_long);


    // Lab 15 Date - November 22, 2022

    // Latt bins
    let (bins, frequency) = count_latt(lat4, min_latt, max_latt);

    println!("{:?} \n{:?}", bins, frequency);

    
    let mut f1 = File::create("lat.csv").expect("Unable to create file"); 

    let mut i = 0;
    let length = bins.len();

    while i!= length - 1
    {   
        write!(f1, "{} {}\n", bins[i], frequency[i]);
        i += 1
    }
    write!(f1, "{}", bins[length-1]);

    // longitude  bins
    let (bins, frequency) = count_long(long4, min_long, max_long);

    println!("{:?} \n{:?}", bins, frequency);

    
    let mut f1 = File::create("long.csv").expect("Unable to create file"); 

    let mut i = 0;
    let length = bins.len();

    while i!= length - 1
    {   
        write!(f1, "{} {}\n", bins[i], frequency[i]);
        i += 1
    }
    write!(f1, "{}", bins[length-1]);
}
