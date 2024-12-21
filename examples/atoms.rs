#![allow(unused_assignments)]
use minifb::Key;
use mathwin::MathWin;

mod body;    //body.rs 를 사용한다는 선언 


fn main() {

    //창 크기는 가급적 정사각형으로 100도트 이상을 사용합니다.
    let mut my_math = MathWin::new(640, 640); //창 크기
    //전체 프로그램에서 사용할 상수는 여기서 지정하는 것이 좋겠습니다.
    let fps: usize = 1200; //화면 갱신 주기: 초당 60개 이하 or 계산량 증가 시 느려집니다.
    let x_max: f64 = 25.0E-10; //2D 상자 크기
    let y_max: f64 = 25.0E-10; //2D 상자 크기
	let x_min: f64 = -25.0E-10; //2D 상자 크기
    let y_min: f64 = -25.0E-10; //2D 상자 크기
	
    let mut string_disp_0 = "                 ".to_owned();  // 상자 안 메세지(1)  
    let mut string_disp_1 = "                 ".to_owned();  // 상자 안 메세지(2)  
    let mut string_disp_2 = "                 ".to_owned();  // 상자 안 메세지(2)  
    let mut string_disp_3 = "                 ".to_owned();  // 상자 안 메세지(2)  
    
    //입자 개수, 위치... 세팅
    let num_of_body: usize = 10;
    let mut bodies: Vec<body::Body> = Vec::new();
    body::Body::make_bodies(&mut bodies, num_of_body);
    body::Body::init_bodies_atom_scale(&mut bodies, x_min, x_max, y_min, y_max);

    //GUI:초기화: x축 숫자 영역, y축 숫자 영역을 설정합니다.
    my_math.initialize(x_min, x_max, y_min, y_max); //x_start, x_end,y_..
    //GUI:커맨드 창에 기본 정보를 표시합니다.
    my_math.print_report();
    //GUI:화면 갱신 주기를 초당 N개의 프레임으로 낮출 수 있습니다.
    my_math.win.set_target_fps(fps);
    my_math.clear_screen(MathWin::BLACK);
	
    //연속 동작을 원하는 코드를 작성합니다.
    while my_math.win.is_open() && !my_math.win.is_key_down(Key::Escape) {
    //창을 닫거나 ESC 키를 계속 누르고 있으면 프로그램이 점잖게 종료합니다.(강제 종료: Ctrl+C)        
        for body in &mut bodies { //새 위치 계산
			if body.id == 0 {continue}
			else{			
				let r: f64 = (body.x.powf(2.0) + body.y.powf(2.0)).powf(0.5);
				let accel: f64 = body::Body::force_atom(r); //레너드-존스(Lennard-Jones) 퍼텐셜
				let a_x: f64 = accel * body.x / r;
				let a_y: f64 = accel * body.y / r;
				let t_delta: f64 = 2.0E-19;
				let t_delta_sqr: f64 = t_delta.powf(2.0);
				body.x = body.x + body.vx*t_delta + 0.5*a_x*t_delta_sqr;
				body.y = body.y + body.vy*t_delta + 0.5*a_y*t_delta_sqr;	
				body.vx = body.vx + a_x*t_delta;
				body.vy = body.vy + a_y*t_delta;
			}
		}
		
		for body in &mut bodies {
			my_math.circle(body.x_old, body.y_old, body.r, MathWin::BLACK);
			my_math.circle(body.x, body.y, body.r, MathWin::RED);
			body.x_old = body.x;
			body.y_old = body.y;
	    }
		
		//힘 그래프 그리기(r: 0.5~3.5)
		my_math.line(0.0, 0.0, x_max, 0.0, MathWin::GRAY);
		my_math.line(0.0, y_min/2.0, 0.0, y_max, MathWin::GRAY);
		for i in (10..1300).map(|x| x as f64 * 1.0E-12) {
			let f: f64 = body::Body::force_atom_disp(i);
			if f < y_max {
				my_math.circle(i, f, 1.2E-11, MathWin::GREEN);
			}
		}
		
        my_math.draw_x_axis_with_grid(10, MathWin::WHITE); //position_y
    	my_math.draw_y_axis_with_grid(10, MathWin::WHITE);  //position_x
    	
		string_disp_0 = format!("Positron & Electron in a Atom"); 
		string_disp_1 = format!("R : Reset!"); 
		string_disp_2 = format!("Potential Energy of Center Particle"); 
        string_disp_3 = format!("(Lennard-Jones potential)"); 
        my_math.print_str6x8(100, 100, &string_disp_0, MathWin::BLACK); //메세지 지우기 
        my_math.print_str6x8(100, 115, &string_disp_1, MathWin::BLACK); 
        my_math.print_str6x8(400, 100, &string_disp_2, MathWin::BLACK); 
        my_math.print_str6x8(400, 115, &string_disp_3, MathWin::BLACK); 
        
        my_math.print_str6x8(100, 100, &string_disp_0, MathWin::WHITE); //메세지 쓰기
		my_math.print_str6x8(100, 115, &string_disp_1, MathWin::WHITE); 
        my_math.print_str6x8(400, 100, &string_disp_2, MathWin::GREEN); 
        my_math.print_str6x8(400, 115, &string_disp_3, MathWin::GREEN); 
        
        my_math.show();

		if my_math.win.is_key_down(Key::R) {
        	body::Body::init_bodies_atom_scale(&mut bodies, x_min, x_max, y_min, y_max);
			my_math.clear_screen(MathWin::BLACK);
        }  		
        if my_math.win.is_key_down(Key::Escape) {
            break;
        }
    }
}
