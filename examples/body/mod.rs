#![allow(unused_imports, dead_code)]

use mathwin::MathWin;
use rand::distributions::{Distribution, Uniform};
use std::num;

#[derive(Debug)]
pub struct Body{
    pub id: usize,
    pub x: f64,
    pub y: f64,
    pub x_old: f64,
    pub y_old: f64,
    pub vx: f64,
    pub vy: f64,
    pub v_abs: f64,
    pub a_x: f64,
    pub a_y: f64,
    pub mass: f64,
	pub charge: f64,
    pub r: f64, //물체의 반지름
    pub color: u32,
    pub exist: bool, 
}
impl Copy for Body{}
impl Clone for Body {
	fn clone(&self) -> Body {
		*self
	}
}

impl Body{
    pub fn new() -> Body{
        let new_body = Body{
            id: 0,
            x: 0.0,
            y: 0.0,
            x_old: 0.0,
            y_old: 0.0,
            vx: 0.0,
            vy: 0.0,
            v_abs: 0.0,
            a_x: 0.0,
            a_y: 0.0,
            mass: 0.0,
			charge: 0.0,
            r: 0.0, //물체의 반지름
            color: MathWin::BLACK,
            exist: true, 
        };
        return new_body;
    }
    pub fn make_bodies(bodies: &mut Vec<Body>, num_of_body: usize){
        for _i in 0..num_of_body {
            let one_body = Body::new();
            bodies.push(one_body);
        }
    }
	//입자: 질량, 전하.. 전자 수준으로 초기화
    pub fn init_bodies_atom_scale(bodies: &mut Vec<Body>, 
									x_min: f64, x_max: f64, y_min: f64, y_max: f64) {
        let range_mass_core = Uniform::new(1.60E-27, 1.70E-27); 
		let range_mass_electron = Uniform::new(9.0E-31, 9.2E-31); 
		let range_charge_core = Uniform::new(1.60E-19, 1.70E-19); 
		let range_charge_electron = Uniform::new(1.60E-19, 1.70E-19); 
		
		let range_x = Uniform::new(x_min/2.0, x_max/2.0);
		let range_y = Uniform::new(y_min/2.0, y_max/2.0);
		let range_v_xy = Uniform::new(-3.0E5, 3.0E5); //광속의 약 1/100
		let mut rng = rand::thread_rng();
		let mut id: usize = 0;
		for body in bodies {
			body.id = id;
			if id == 0 {
				body.x = 0.0;
				body.y = 0.0;
				body.x_old = body.x;
				body.y_old = body.y;
				body.mass = range_mass_core.sample(&mut rng);
				body.charge = range_charge_core.sample(&mut rng);
				body.r = 4.0E-11;
				body.vx = 0.0;
				body.vy = 0.0;
				body.v_abs = 0.0;
			
			}else {
				body.x = range_x.sample(&mut rng);
				body.y = range_y.sample(&mut rng);
				body.x_old = body.x;
				body.y_old = body.y;
				body.mass = range_mass_electron.sample(&mut rng);
				body.charge = range_charge_electron.sample(&mut rng);
				body.r = 2.0E-11;
				body.vx = range_v_xy.sample(&mut rng);
				body.vy = range_v_xy.sample(&mut rng);
				body.v_abs = (body.vx.powf(2.0) + body.vy.powf(2.0)).sqrt();			
			}
			id += 1;					
		}
	}
	pub fn force_atom_disp(r: f64)-> f64 {
			let scale: f64 = 1.0E-28;
			let a: f64 = 1.0E-10; 
			let b: f64 = -0.05E1; //
			let c: f64 = 1.0E-10;
			let e: f64 =  std::f64::consts::E;
			let mu: f64 = 0.5;
			let mu_r: f64 = mu * r;
			scale * e.powf(-1.0 * mu_r) * (a/mu_r + b/mu_r.powf(2.0) + c/mu_r.powf(3.0)) 
	}
	//중심력 장: 퍼텐셜의 모양은 레너드-존스(Lennard-Jones) 퍼텐셜의 모양으로 설정하였음
	pub fn force_atom(r: f64)-> f64 {
			let scale: f64 = 9.0E3;
			let a: f64 = 1.0E-10; 
			let b: f64 = -0.05E1; //
			let c: f64 = 1.0E-10;
			let e: f64 =  std::f64::consts::E;
			let mu: f64 = 0.5;
			let mu_r: f64 = mu * r;
			scale * e.powf(-1.0 * mu_r) * (a/mu_r + b/mu_r.powf(2.0) + c/mu_r.powf(3.0)) 
	}
	//입자: 질량, 전하.. 태양 수준으로 초기화
    pub fn init_bodies_solar_scale(bodies: &mut Vec<Body>, 
		x_min: f64, x_max: f64, y_min: f64, y_max: f64) {
		let range_mass_core = Uniform::new(1.60E30, 1.70E31); 
		let range_charge_core = Uniform::new(1.60E13, 1.70E17); 
		
		let range_x = Uniform::new(x_min/2.0, x_max/2.0);
		let range_y = Uniform::new(y_min/2.0, y_max/2.0);
		let range_v_xy = Uniform::new(-1.0E4, 1.0E4); 
		let mut rng = rand::thread_rng();
		let mut id: usize = 0;
		for body in bodies {
			body.x = range_x.sample(&mut rng);
			body.y = range_y.sample(&mut rng);
			body.x_old = body.x;
			body.y_old = body.y;
			body.mass = range_mass_core.sample(&mut rng);
			body.charge = range_charge_core.sample(&mut rng);
			body.r = 4.0E10;
			body.vx = range_v_xy.sample(&mut rng);
			body.vy = range_v_xy.sample(&mut rng);
			body.v_abs = (body.vx.powf(2.0) + body.vy.powf(2.0)).powf(-2.0);	
			body.exist = true; 
			body.id = id;
			id += 1;				
		}
	}
	pub fn calc_new_location(bodies: &mut Vec<Body>, time_interval: f64){
		let num_bodies: usize = bodies.len();
		let mut distance_sq: f64;
		let mut distance_xy: f64;
		
		let mut direction_force: f64; // +1.0 or -1.0
		let mut acceleration: f64;
		const G: f64 = 6.67430E-11; //gravitational_constant
		for i in 0..num_bodies {
			if bodies[i].exist == false{
				continue;
			}
			bodies[i].a_x = 0.0;
			bodies[i].a_y = 0.0;
			for j in 0..num_bodies {
				if bodies[j].exist == false{ 
					continue;
				}
				if bodies[i].id == bodies[j].id{
					continue;
				}
				distance_sq = (bodies[j].x - bodies[i].x).powf(2.0) + (bodies[j].y - bodies[i].y).powf(2.0);
				//calc_x_acceleration
				distance_xy = bodies[j].x - bodies[i].x;
				if distance_xy < 0.0 {
					direction_force = -1.0; 
				}else{
					direction_force = 1.0;
				}
				acceleration = G * bodies[j].mass / distance_sq * direction_force;
				bodies[i].a_x += acceleration;
				//calc_y_acceleration
				distance_xy = bodies[j].y - bodies[i].y;
				if distance_xy < 0.0 {
					direction_force = -1.0; 
				}else{
					direction_force = 1.0;
				}
				acceleration = G * bodies[j].mass / distance_sq * direction_force;
				bodies[i].a_y += acceleration;
			}		
			bodies[i].x += bodies[i].vx * time_interval + 0.5 * bodies[i].a_x * time_interval.powf(2.0);
			bodies[i].y += bodies[i].vy * time_interval + 0.5 * bodies[i].a_y * time_interval.powf(2.0);
			bodies[i].vx += bodies[i].a_x * time_interval;
			bodies[i].vy += bodies[i].a_y * time_interval;  				
		}
	}
	pub fn merge_if_too_close(bodies: &mut Vec<Body>, merge_distance: f64){
		let num_bodies: usize = bodies.len();
		let mut distance_sq: f64;
		let merge_distance_sq: f64 = merge_distance.powf(2.0);
		let mut momentum_x: f64;
		let mut momentum_y: f64;

		for i in 0..num_bodies {
			if bodies[i].exist{
				for j in 0..num_bodies {
					if bodies[i].id == bodies[j].id{
						continue;
					}
					if bodies[j].exist {
						distance_sq = (bodies[j].x - bodies[i].x).powf(2.0) + (bodies[j].y - bodies[i].y).powf(2.0);
						if distance_sq <= merge_distance_sq{
					
							//전체 운동량 보존
							momentum_x = bodies[i].mass * bodies[i].vx + bodies[j].mass * bodies[j].vx;
							momentum_y = bodies[i].mass * bodies[i].vy + bodies[j].mass * bodies[j].vy;
							bodies[i].mass = bodies[i].mass + bodies[j].mass; //질량 보존
							bodies[j].mass = 0.1E-10; //먼지가 되었음
							bodies[j].vx = 0.0;
							bodies[j].vy = 0.0;
							bodies[j].x = 0.0;
							bodies[j].y = 0.0;
							bodies[j].exist = false;

							bodies[i].vx = momentum_x / bodies[i].mass;
							bodies[i].vy = momentum_y / bodies[i].mass;
							bodies[i].r = (bodies[i].r.powf(3.0) + bodies[j].r.powf(3.0)).powf(1.0/3.0); //부피 보존
						}
					}
				}
			}
		}
	}            
}

