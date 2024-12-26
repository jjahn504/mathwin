#![allow(unused_assignments)]
use minifb::Key;
use mathwin::MathWin;

mod body;    //body.rs 를 사용한다는 선언 


fn main() {

    //창 크기는 가급적 정사각형으로 100도트 이상을 사용합니다.
    let mut my_math = MathWin::new(640, 640); //창 크기
    //전체 프로그램에서 사용할 상수는 여기서 지정하는 것이 좋겠습니다.
    let fps: usize = 1200; //화면 갱신 주기: 초당 60개 이하 or 계산량 증가 시 느려집니다.
    let xy_max: f64 = 2.50E12; //2D 상자 크기
    let x_max: f64 = xy_max; //2D 상자 크기
    let y_max: f64 = xy_max; //2D 상자 크기
	let x_min: f64 = -1.0 * xy_max; //2D 상자 크기
    let y_min: f64 = -1.0 * xy_max; //2D 상자 크기
	
    let mut string_disp_0 = "                 ".to_owned();  // 상자 안 메세지(1)  
    let mut string_disp_1 = "                 ".to_owned();  // 상자 안 메세지(1)  
    
    //입자 개수, 위치... 세팅
    let num_of_body: usize = 3;
    let mut bodies: Vec<body::Body> = Vec::new();
    body::Body::make_bodies(&mut bodies, num_of_body);
    body::Body::init_bodies_solar_scale(&mut bodies, x_min, x_max, y_min, y_max);

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
        body::Body::calc_new_location(&mut bodies, 1000.0);
        body::Body::merge_if_too_close(&mut bodies, 1000000.0);

        // Display solars in MathWin 
        for body in &mut bodies {
            my_math.circle(body.x_old, body.y_old, body.r, MathWin::BLACK);
			my_math.circle(body.x, body.y, body.r, MathWin::RED);
			body.x_old = body.x;
			body.y_old = body.y;
	    }
				
        my_math.draw_x_axis_with_grid(10, MathWin::WHITE); //position_y
    	my_math.draw_y_axis_with_grid(10, MathWin::WHITE);  //position_x
    	
		string_disp_0 = format!("Three Solars Problem"); 
		string_disp_1 = format!("R : Reset!"); 
		my_math.print_str6x8(100, 100, &string_disp_0, MathWin::BLACK); //메세지 지우기 
        my_math.print_str6x8(100, 115, &string_disp_1, MathWin::BLACK); //메세지 지우기 
        
        my_math.print_str6x8(100, 100, &string_disp_0, MathWin::WHITE); //메세지 쓰기
		my_math.print_str6x8(100, 115, &string_disp_1, MathWin::WHITE); //메세지 쓰기
		
        my_math.show();

		if my_math.win.is_key_down(Key::R) {
        	body::Body::init_bodies_solar_scale(&mut bodies, x_min, x_max, y_min, y_max);
			my_math.clear_screen(MathWin::BLACK);
        }  		
    }
}
