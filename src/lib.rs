#![allow(unused_imports)]
use minifb::{Key, Window, WindowOptions, Scale};
use minifb_fonts::*;
use std::mem;

#[allow(dead_code)]

#[derive(Debug)]
pub struct MathWin{
    pub width: usize,
    pub height: usize,
    pub bottom_margin: usize,
    pub left_margin: usize,
    pub x_start_digit: usize,
    pub x_end_digit: usize,
    pub y_start_digit: usize,
    pub y_end_digit: usize,
    pub win: minifb::Window,
    pub screen: Vec<u32>,
    pub x_start: f64,
    pub y_start: f64,
    pub x_end: f64,
    pub y_end: f64,
    pub x_dot_scale: f64,
    pub y_dot_scale: f64,

}

impl MathWin{
    pub const BLACK: u32 = 0x000000; //RGB 
	pub const RED: u32 = 0xff0000;
	pub const WHITE: u32 = 0xffffff;
	pub const BLUE: u32 = 0x0000ff;
	pub const GRAY: u32 = 0x888888; //RGB
	pub const GREEN: u32 = 0x00ff00; //RGB
	
    pub fn new(width: usize, height: usize) -> MathWin {
        if width < 100 {
            println!("Width must be lager then 100");
        };
        if height < 100 {
            println!("Height must be lager then 100");
        };
        MathWin{
            width: width,
            height: height,
            bottom_margin: 50,
            left_margin: 50,
            x_start_digit: 0,
            x_end_digit: 0,
            y_start_digit: 0,
            y_end_digit: 0,
            
            win: Window::new(
                "minifb+수학 - 종료: ESC",
                width,
                height,
                WindowOptions::default(),
            ).expect("No Window"),
            screen: vec![0; width * height],  
            x_start: -1.5E16,
            y_start: -1.5E16,
            x_end: 1.5E16,
            y_end: 1.5E16,
            x_dot_scale: 0.0E16,
            y_dot_scale: 0.0E16,      
        }
    }
    pub fn initialize(&mut self, x_start: f64, x_end: f64, y_start: f64, y_end: f64) {
        self.x_start_digit = self.left_margin;
        self.x_end_digit = self.width;
        self.y_start_digit = self.height - self.bottom_margin;
        self.y_end_digit = 0;
        self.x_start = x_start;
        self.x_end = x_end;
        self.y_start = y_start;
        self.y_end = y_end;
        self.x_dot_scale = (x_end - x_start) / (self.x_end_digit - self.x_start_digit) as f64;
        self.y_dot_scale = (y_end - y_start) / (self.y_start_digit - self.y_end_digit) as f64; 
    }
    pub fn print_report(& self){
        println!("x축 범위: {:+.2e} ~ {:+.2e}", self.x_start, self.x_end);
        println!("y축 범위: {:+.2e} ~ {:+.2e}", self.y_start, self.y_end);
        //println!("i64::MAX: {}", i64::MAX);
    }
    pub fn show(&mut self) {
        self.win
            .update_with_buffer(&self.screen, self.width, self.height)
            .unwrap();
    }
    pub fn point(&mut self, x: f64, y: f64, color: u32) {
        if x > self.x_end {return;}; 
        if y > self.y_end {return;};
        if x < self.x_start {return;}; 
        if y < self.y_start {return;};
        let x_digit = self.x_start_digit + ((x - self.x_start) / self.x_dot_scale) as usize;
        let y_digit = self.y_start_digit - ((y - self.y_start) / self.y_dot_scale) as usize;
        self.point_digit(x_digit as usize, y_digit as usize, color); 
    }
    pub fn circle(&mut self, x_center: f64, y_center: f64, radius: f64, color: u32) {
        let x_center_digit: usize = self.x_start_digit + 
            (((x_center - self.x_start) / self.x_dot_scale) as usize);
        let y_start_digit: usize = self.y_start_digit;
        let y_digit: usize = ((y_center - self.y_start) / self.y_dot_scale) as usize;
        let y_center_digit: usize = if y_start_digit > y_digit{ y_start_digit - y_digit }
                                    else { 0 };      
        let radius_digit: usize = (radius / self.x_dot_scale) as usize; //반지름 크기는 x축 스케일 
        self.circle_digit(x_center_digit, y_center_digit, radius_digit, color); 
    }
    pub fn circle_digit_radius(&mut self, x_center: f64, y_center: f64, color: u32, radius: usize) {
        let x_center_digit: usize = self.x_start_digit + 
            ((x_center - self.x_start) / self.x_dot_scale) as usize;
        let y_center_digit: usize = self.y_start_digit - 
            ((y_center - self.y_start) / self.y_dot_scale) as usize;
        self.circle_digit(x_center_digit, y_center_digit, radius, color); 
    }
    pub fn line(&mut self, x_start: f64, y_start: f64, x_end: f64, y_end: f64, color: u32){
        let delta_x0: f64 = x_start - self.x_start;
        if delta_x0 < 0. {return;};  
        let delta_y0: f64 = y_start - self.y_start;
        if delta_y0 < 0. {return;};  
        let delta_x1: f64 = x_end - self.x_start;
        if delta_x1 < 0. {return;};  
        let delta_y1: f64 = y_end - self.y_start;
        if delta_y1 < 0. {return;}; 
        
        let x0: usize = self.x_start_digit + (delta_x0 / self.x_dot_scale) as usize;
        let y0: usize = self.y_start_digit - (delta_y0 / self.y_dot_scale) as usize;
        let x1: usize = self.x_start_digit + (delta_x1 / self.x_dot_scale) as usize;
        let y1: usize = self.y_start_digit - (delta_y1 / self.y_dot_scale) as usize;
        
        self.line_digit(x0, y0, x1, y1, color);    
    }
    pub fn clear_screen(&mut self, color: u32){
        for i in self.screen.iter_mut(){
            *i = color;
        }
    }
    pub fn draw_x_axis_with_grid(&mut self, num_grid: usize, color: u32){
        let len_pin: usize = 10; // 축에 있는 핀의 길이
        let position_y: usize = self.y_start_digit;
        let range_x_digit: usize = self.x_end_digit - self.x_start_digit; 
        let delta_digit: usize;
        if num_grid == 0{
            delta_digit = range_x_digit;    
        } else{
            delta_digit = range_x_digit / num_grid;
        }
        for i in 0..range_x_digit {
            self.point_digit(self.x_start_digit + i, position_y, color);
            if (i % delta_digit) == 0 {
                for j in 0..len_pin {
                    self.point_digit(self.x_start_digit + i, position_y - j, color);
                }
                self.print_str6x8(self.x_start_digit + i, position_y + 3, 
                    format!("{:.2e}", 
                        (self.x_start + (i as f64 * self.x_dot_scale))).as_str(), color);
            }
        }
    }
    pub fn draw_y_axis_with_grid(&mut self, num_grid: usize, color: u32){
        let len_pin: usize = 10; // 축에 있는 핀의 길이
        let position_x: usize = self.x_start_digit;
        let range_y_digit: usize = self.y_start_digit - self.y_end_digit; 
        let delta_digit: usize;
        if num_grid == 0{
            delta_digit= range_y_digit;    
        } else{
            delta_digit = range_y_digit / num_grid;
        }
        for i in 0..range_y_digit {
            self.point_digit(position_x, self.y_start_digit - i, color);
            if (i % delta_digit) == 0 {
                for j in 0..len_pin {
                    self.point_digit(position_x + j, self.y_start_digit - i, color);
                }
                self.print_str6x8(position_x - 40, self.y_start_digit - i, 
                    format!("{:.2e}", 
                        (self.y_start + (i as f64 * self.y_dot_scale))).as_str(), color);
            }
        }
    }
    pub fn draw_x_axis_at_y_zero(&mut self, color: u32){
        if self.y_start <= 0.0 && self.y_end >= 0.0 {
            self.line(self.x_start, 0.0, self.x_end, 0.0, color);
        } 
    }
    pub fn draw_y_axis_at_x_zero(&mut self, color: u32){
        if self.x_start <= 0.0 && self.x_end >= 0.0 {
            self.line(0.0, self.y_start, 0.0, self.y_end, color);
        } 
    }
    pub fn write_zero(&mut self, color: u32){
        if self.y_start <= 0.0 && self.y_end >= 0.0 {
            if self.x_start <= 0.0 && self.x_end >= 0.0 {
                let mut x_pos: usize = ((0.0 - self.x_start) / self.x_dot_scale) as usize;
                let mut y_pos: usize = ((0.0 - self.y_start) / self.y_dot_scale) as usize;
                x_pos = self.x_start_digit + x_pos;
                y_pos = self.y_start_digit - y_pos;
                self.print_str6x8(x_pos - 10, y_pos + 10, "0", color);            
            }   
        }
    }
    
        
    pub fn print_str5x8(&mut self, x_pos: usize, y_pos: usize, string: &str, color: u32) {
        let mut text = font5x8::new_renderer(self.width, self.height, color);
        text.set_color(color);
        text.draw_text(&mut self.screen, x_pos, y_pos, string);
    }
    pub fn print_str6x8(&mut self, x_pos: usize, y_pos: usize, string: &str, color: u32) {
        let mut text = font6x8::new_renderer(self.width, self.height, color);
        text.set_color(color);
        text.draw_text(&mut self.screen, x_pos, y_pos, string);
    }
    //치역(Range) 찾기 
    pub fn detect_range(&mut self, y_eq_fx: fn(f64) -> f64, x_min: f64, x_max: f64, width_digit: usize) 
                        -> (f64, f64, f64) {
        let x_delta: f64 = (x_max - x_min) / width_digit as f64;
        let mut y_min: f64 = x_max;
        let mut y_max: f64 = x_min;
        for i in 0..width_digit{
            let x = x_min + (i as f64 * x_delta);
            let y = y_eq_fx(x);
            if y < y_min {y_min = y; };
            if y > y_max {y_max = y; };
        }
        (y_min, y_max, x_delta)
    }
       
    // 아래는 내부용 함수로만 사용함    
    fn point_digit(&mut self, x: usize, y: usize, color: u32){
        if x >= self.width {return;}; 
        if y >= self.height {return;};
        let index_screen = y * self.width + x;
        self.screen[index_screen] = color; 
    }
    fn circle_digit(&mut self, x_center: usize, y_center: usize, radius: usize, color: u32){
        let x_end: i32 = radius as i32;
        let y_end: i32 = radius as i32;
        let r: i32 = radius as i32;
        let color_circle: u32 = color;



        for x in 0_i32..x_end{
            for y in 0_i32..y_end{
                let criterion = x * x + y * y - r * r;
                if criterion > 0{
                    break;
                }
                self.point_digit(x_center + x as usize, y_center + y as usize, color_circle);
                let mut temp_x: i32 = x_center as i32 - x;
                if temp_x >= 0 {
                    self.point_digit(temp_x as usize, y_center + y as usize, color_circle);
                }
                let mut temp_y: i32 = y_center as i32 - y;
                if temp_y >= 0 {
                    self.point_digit(x_center + x as usize, temp_y as usize, color_circle);
                }
                temp_x = x_center as i32 - x;
                temp_y = y_center as i32 - y;
                if temp_x >= 0 && temp_y >= 0 {
                    self.point_digit(temp_x as usize, temp_y as usize, color_circle);
                }
            }
        }
    }
    
    fn line_digit(&mut self, x_start: usize, y_start: usize, x_end: usize, y_end: usize, color: u32){
        let mut x_0: i32 = x_start as i32;
        let mut x_1: i32 = x_end as i32;
        let mut y_0: i32 = y_start as i32;
        let mut y_1: i32 = y_end as i32;
        
        let steep = (x_1 - x_0).abs() < (y_1 - y_0).abs();
        if steep {  
            mem::swap(&mut x_0, &mut y_0); 
            mem::swap(&mut x_1, &mut y_1);
        }
        if x_0 > x_1 {
            mem::swap(&mut x_0, &mut x_1);
            mem::swap(&mut y_0, &mut y_1);
        }
        let delta_x = x_1 - x_0;
        let delta_y = y_1 - y_0;
        let delta_error2 = delta_y.abs() * 2;
        let mut error2 = 0;
        let mut y = y_0;
        let mut x = x_0;
        while x <= x_1 {
            if steep {
                self.point_digit(y as usize, x as usize, color); 
            } else {
                self.point_digit(x as usize, y as usize, color);
            }
            error2 += delta_error2;
            if error2 > delta_x {
                y += if y_1 > y_0 {1} else {-1};
                error2 -= delta_x * 2;
            }
            x+= 1;        
        }
    }    
}

