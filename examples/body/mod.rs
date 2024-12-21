#![allow(unused_imports, dead_code)]

use mathwin::MathWin;
use rand::distributions::{Distribution, Uniform};
use std::num;

//#[derive(Debug)]
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
    pub exist: usize, //0: No, 1: Yes
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
            exist: 1, //0: No, 1: Yes
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
            
}

