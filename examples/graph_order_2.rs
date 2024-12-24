use minifb::Key;
use mathwin::MathWin;

fn y_eq_fx(x: f64) -> f64{
    (2.5 * x * x) - (4.1 * x)  - 13.7
}

fn main() {
    let width: usize = 640;
    //창 크기는 가급적 정사각형으로 100도트 이상을 사용합니다.
    let mut my_math = MathWin::new(width, width); //창 크기
    //전체 프로그램에서 사용할 상수는 여기서 지정하는 것이 좋겠습니다.
    let x_min: f64 = -10.0; //정의역(Domain)
    let x_max: f64 = 10.0; //정의역(Domain)
    let y_range: (f64, f64, f64) = my_math.detect_range(y_eq_fx, x_min, x_max, width);//치역(Range)
    //보기 좋은 그래프를 위해 5% 여유 공간을 설정함
    let y_min: f64 = y_range.0 * 1.05 - 30.0; // y<0 영역 확장함
    let y_max: f64 = y_range.1 * 1.05;  
    let x_delta: f64 = y_range.2; //중복 계산 회피함 
    let radius_of_point: f64 = x_delta * 1.5;
    //초기화: x축 숫자 영역, y축 숫자 영역을 설정합니다.
    my_math.initialize(x_min, x_max, y_min, y_max); //x_start, x_end,y_..
    //커맨드 창에 기본 정보를 표시합니다.
    my_math.print_report();
    //화면 갱신 주기를 초당 N개의 프레임으로 낮출 수 있습니다.
    my_math.win.set_target_fps(60);

    while my_math.win.is_open() && !my_math.win.is_key_down(Key::Escape) {
    //창을 닫거나 ESC 키를 계속 누르고 있으면 프로그램이 점잖게 종료합니다.(강제 종료: Ctrl+C)
    //연속 동작을 원하는 코드를 작성합니다.
        
        for i in 0..width{
            let x: f64 = x_min + (i as f64 * x_delta);  
            let y: f64 = y_eq_fx(x);
            
            my_math.circle(x, y, radius_of_point, MathWin::RED);
        }
        my_math.draw_x_axis_at_y_zero(MathWin::BLUE);
        my_math.draw_y_axis_at_x_zero(MathWin::BLUE);
        my_math.write_zero(MathWin::WHITE);
        
        my_math.draw_x_axis_with_grid(10, MathWin::WHITE); //position_y
        my_math.draw_y_axis_with_grid(10, MathWin::WHITE);  //position_x
        my_math.show();        
    }
}
