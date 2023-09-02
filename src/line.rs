extern crate colored;

use colored::*;
use std::thread;
use std::time::Duration;

pub enum CellStatus {
    Unknow,
    Empty,
    Filled,
}

pub enum LineOrientation {
    Row,
    Colunm,
}

pub struct HoleInfos {
    pub value: u32,
    pub is_done: bool,
}

pub struct Line {
    pub orientation: LineOrientation,
    pub position: u32,
    pub holes: Vec<HoleInfos>,
    pub data: Vec<CellStatus> 
}

impl Line {
    fn new(&mut self, size:u32, hvl: Vec<u32>) {
        self.orientation = LineOrientation::Row;
        self.position = 0;
        self.data = vec![CellStatus::Unknow; size];
        self.holes = vec![{0,false}; hv1.len()];
        for i in 1..hvl.len() {
            self.holes[i].value = hvl[i];
        }
    }

    pub fn write_data(&mut self, pos:usize, status: CellStatus) {
        assert!(pos < self.data.len());
        thread::sleep(Duration::from_millis(100));
        self.data[pos] = status;
        self.show_line();
    }

    pub fn validate_hole(&mut self, pos:u32, val:bool) {
        assert!(pos < self.holes.len());
        thread::sleep(Duration::from_millis(100));
        self.holes[pos].is_done = val;
        self.show_line();
    }

    pub fn show_line(&self) {
        let str:string = string::Empty;
        
        for h in self.holes {
            if h.is_done {
                str += h.value.to_string().red() + ' ';
            } else {
                str += h.value.to_string() + ' ';
            }
        }
        str += ' ';
        for d in self.data {
            match d {
                CellStatus::Unknow => str += '\u{25A0}',
                CellStatus::Empty => str += '\u{25A1}',
                CellStatus::Filled => str += '\u{25A3}',
            } 
        }
        println!(str);
    }
}