use minifb::Key;
use mathwin::MathWin;

fn main() {
    //창 크기는 가급적 정사가형으로 100도트 이상을 사용합니다.
    let mut my_math = MathWin::new(640, 640); //창 크기
    //전체 프로그램에서 사용할 상수는 여기서 지정하는 것이 좋겠습니다.
    //let r_max: f64 = i64::MAX as f64; //숫자 공간을 i64 의 최댓값 까지 설정 
    let r_max: f64 = 1000000.0; //수학적 2D 공간을 임의의 크기로 설정 
    let half_r_max: f64 = r_max / 2.0;
    //초기화: x축 숫자 영역, y축 숫자 영역을 설정합니다.
    my_math.initialize(0.0, r_max, 0.0, r_max); //x_start, x_end,y_..
    //커맨드 창에 기본 정보를 표시합니다.
    my_math.print_report();
    //화면 갱신 주기를 초당 N개의 프레임으로 낮출 수 있습니다.
    my_math.win.set_target_fps(60);

    //Demo 2: 원운동
    let size_particle: usize = 15;
    let mut x: f64 = half_r_max;
    let mut y: f64 = half_r_max;
    let mut x_old: f64 = x;
    let mut y_old: f64 = y;
    let pi_2: f64 = 2.0 * std::f64::consts::PI;

    let mut angle: f64 = 0.0;      
    while my_math.win.is_open() && !my_math.win.is_key_down(Key::Escape) {
    //창을 닫거나 ESC 키를 계속 누르고 있으면 프로그램이 점잖게 종료합니다.(강제 종료: Ctrl+C)
    //연속 동작을 원하는 코드를 작성합니다.
        my_math.line(half_r_max, half_r_max, x_old, y_old, MathWin::BLACK);
        my_math.circle_digit_radius(x_old, y_old, MathWin::BLACK, size_particle); 
        x = half_r_max + 0.5 * half_r_max * (angle.sin());
        y = half_r_max + 0.5 * half_r_max * (angle.cos());
        my_math.line(half_r_max, half_r_max, x, y, MathWin::BLUE);
        my_math.circle_digit_radius(x, y, MathWin::RED, size_particle); 
        my_math.draw_x_axis_with_grid(10, MathWin::WHITE); //position_y
        my_math.draw_y_axis_with_grid(10, MathWin::WHITE);  //position_x
        my_math.show();
        x_old = x;
        y_old = y;    
        angle += 0.01;
        if angle >= pi_2 {
            angle = 0.0;
        }
        my_math.clear_screen(MathWin::BLACK);        
    }
}
