
use std::fmt;
use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::io::prelude::*
;
use linked_hash_map::LinkedHashMap;
#[derive(Copy, Clone, Debug)]
struct GPS{
    lat:    f64,
    long:   f64,
}

const Degree: &str = "\u{00b0}";

impl fmt::Display for GPS {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        
        let mut lat_direction = "";
        let mut long_direction = "";

        if self.lat < 0.0 {
            lat_direction = "S";
        }
        else{
            lat_direction = "N";
        }

        if self.long < 0.0 {
            long_direction = "W";
        }
        else{
            long_direction = "E";
        }
        
        write!(f, "(Latitude: {}{} {}, Longitude: {}{} {})", self.lat, Degree, lat_direction, self.long, Degree, long_direction)
    }
}
impl fmt::Display for Manager {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Manager: {}, age: {}, location: {}, staff: {:?}",
            self.name, self.age, self.location, self.staff_list
        )
    }
}

impl fmt::Display for SoftEng {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "SoftEng: {}, age: {}, location: {} Skills: {:?}",
            self.name, self.age, self.location, self.skills
        )
    }
}
impl fmt::Display for Translator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Translator: {}, age: {}, location: {} languages: {:?}",
            self.name, self.age, self.location, self.languages
        )
    }
}
struct Manager{
    name: String,
    age: u8,
    location: GPS,
    staff_list: String,  
}

struct SoftEng{
    name: String,
    age: u8,
    location: GPS,
    skills: String,
}

struct Translator{
    name: String,
    age: u8,
    location: GPS,
    languages: String,
}

trait Employee{
    fn get_name(&self) -> String;
    fn get_age(&self) -> u8;
    fn get_location(&self) -> GPS;
    fn get_abilities(&self) -> String;
}

impl Employee for Manager{
    fn get_name(&self) -> String{
       self.name.to_string()
    }

    fn get_age(&self) -> u8{
        self.age
    }

    fn get_location(&self) -> GPS{
        self.location
    }

    fn get_abilities(&self) -> String{
        self.staff_list.to_string()
    }

}

impl Employee for SoftEng{
    fn get_name(&self) -> String{
       self.name.to_string()
    }

    fn get_age(&self) -> u8{
        self.age
    }

    fn get_location(&self) -> GPS{
        self.location
    }

    fn get_abilities(&self) -> String{
        self.skills.to_string()
    }
}

impl Employee for Translator{
    fn get_name(&self) -> String{
       self.name.to_string()
    }

    fn get_age(&self) -> u8{
        self.age
    }

    fn get_location(&self) -> GPS{
        self.location
    }

    fn get_abilities(&self) -> String{
        self.languages.to_string()
     }
}

fn read_file(file_path: String){
    println!("In file {}", file_path);
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    println!("With text:\n{contents}");

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

fn assign(l_h_m: &mut LinkedHashMap<&str, &str>) -> (Manager, SoftEng, Translator){
    //println!("{:?}", l_h_m);

        // empty structure prepare for data
    let mut m = Manager{
        name: String::from(""),
        age: 0,
        location: GPS{
            lat: 0.0,
            long: 0.0,
        },
        staff_list: String::from(""),
    };

    let mut s = SoftEng{
        name: String::from(""),
        age: 0,
        location: GPS{
            lat: 0.0,
            long: 0.0,
        },
        skills: String::from(""),
    };

    let mut t = Translator{
        name: String::from(""),
        age: 0,
        location: GPS{
            lat: 0.0,
            long: 0.0,
        },
        languages: String::from(""),
    };
    
    let pos_arr = l_h_m["Pos"].split(", ").collect::<Vec<&str>>();
    
    if l_h_m["Role"] == "Manager"
    {
        
        m = Manager{
            name: l_h_m["Name"].to_string(),
            age: l_h_m["Age"].parse::<u8>().unwrap(),
            location: GPS{
                lat: pos_arr[0].parse::<f64>().unwrap(),
                long: pos_arr[1].parse::<f64>().unwrap(),
            },
            staff_list: l_h_m["StaffList"].to_string(),    
        };
       
    } 
    
    else if l_h_m["Role"] == "SoftEng"
    {
        
        s = SoftEng{
            name: l_h_m["Name"].to_string(),
            age: l_h_m["Age"].parse::<u8>().unwrap(),
            location: GPS{
                lat: pos_arr[0].parse::<f64>().unwrap(),
                long: pos_arr[1].parse::<f64>().unwrap(),
            },
            
            skills: l_h_m["Skills"].to_string(),    
        };
        
    }
    
    else if l_h_m["Role"] == "Translator"
    {
        t = Translator{
            name: l_h_m["Name"].to_string(),
            age: l_h_m["Age"].parse::<u8>().unwrap(),
            location: GPS{
                lat: pos_arr[0].parse::<f64>().unwrap(),
                long: pos_arr[1].parse::<f64>().unwrap(),
            },
            languages: l_h_m["Languages"].to_string(),    
        };
    }
    
    return (m, s, t)
}

fn print_result(mut l_h_m: &mut LinkedHashMap<&str, &str>) -> u8{
    let (m, s, t) = assign(&mut l_h_m);
    

    if m.name != "".to_string(){
        println!("Manager: {}", m.get_name());
        println!("Age: {}", m.get_age());
        println!("StaffList: {:?}", m.get_abilities());
        println!("Location: {}\n", m.location);

        return m.get_age();
    }
    else if s.name != "".to_string(){
        println!("SoftEng: {}", s.get_name());
        println!("Age: {}", s.get_age());
        
        println!("Skills: {:?}", s.get_abilities());
        println!("Location: {}\n", s.location);

        return s.get_age();
    }
    else if t.name != "".to_string(){
        println!("Translator: {}", t.get_name());
        println!("Age: {}", t.get_age());
        
        println!("Languages: {:?}", m.get_abilities());
        println!("Location: {}\n", t.location);

        return t.get_age();
    }
    else
    {
        return 0;
    }
    
}


fn main() {  
    
    let softeng = SoftEng{
        name: "John".to_string(),
        age : 21,
        location: GPS{
            lat: 4331.6556, 
            long: 4331.6556,
        },

        skills: "Python, Rust".to_string(),

    };

    let manager = Manager{
        name: "Peter".to_string(),
        age : 65,
        location: GPS{
            lat: 12.3, 
            long: 34.90,
        },

        staff_list: "John, Emma".to_string(),

    };

    let translator = Translator{
        name: "Peter".to_string(),
        age : 65,
        location: GPS{
            lat: 12.3, 
            long: 34.90,
        },

        languages: "Japanese, Chinese".to_string(),

    };
    let mut total = 0;
    let mut count = 0;
    //read_file_line_by_line(file_url);
    
    let mut file = File::open("Employee_data.txt").expect("File not found");
    let mut data = String::new();
    file.read_to_string(&mut data).expect("Error while reading file");
    
    let trim1 = data.trim();
      
    let trim1 = trim1.replace("  ", "");
    let trim1 = trim1.replace("\r", "");

    println!();

    let mut linked_hash_map: LinkedHashMap<&str, &str> = LinkedHashMap::new();
    
    for line in trim1.lines()
    {
        let strings: Vec<&str> = line.split(": ").collect();
        //println!("{:?}", strings);

        if strings.len() == 2
        {
            linked_hash_map.insert(strings[0], strings[1]);
        }
        if strings[0] == ""
            {
                
                total += print_result(&mut linked_hash_map);
                count += 1;
                linked_hash_map.clear();
            }
        
        
    }
    
    total += print_result(&mut linked_hash_map);
    count += 1;
    println!("Average age = {}", total / count);
    
    
    
}
