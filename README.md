# mathwin ( feat: minifb of Rust )

I want to make Light GUI for Mathematical Play based on minifb crate with Rust programming language.

minifb 크레이트는 크기가 작아서 작동 원리를 이해하기 쉽고.

처리 속도가 빠른 편이라고 생각합니다.

이러한 minifb 크레이트를 기반으로... 

대량의 데이터 를 빠르게 처리하여,

2차원 애니메이션이나 그래프로 볼 수 있는 GUI 를 만드는 중입니다.



## To run this example:

1) 원 그리는 기능 테스트
```
cargo run --example heart
```
<img src="pic/heart.gif" width="400" height="400"/>

<br>

2) 선 그리는 기능 테스트
```
cargo run --example rotation
```

<img src="pic/rotation.gif" width="400" height="400"/>

<br>

3) 입자론으로 시뮬레이션 한 원자 내부.
('R' 키를 누르면 리셋 됨)

  전자와 양성자 간의 전기력만으로는

전자가 원자핵에 접근하지 못하는 현상을 설명할 수 없기에

분자간에 작용하는 반데르발스 힘을 설명할 때 등장하는

레나드-존스 퍼텐셜과 유사한 힘이 원자 내부에서 존재한다고 가정하고 만들어보았어요.

```rust
cargo run --example atoms
```

<img src="pic/atoms.gif" width="400" height="400"/>

<br>

4) 입자론으로 시뮬레이션 한 원자 내부
(전자 1만 개의 거동을 동시에 관찰함)

  전자와 양성자 간의 전기력만으로는
  
전자가 원자핵에 접근하지 못하는 현상을 설명할 수 없기에

분자간에 작용하는 반데르발스 힘을 설명할 때 등장하는

레나드-존스 퍼텐셜과 유사한 힘이 원자 내부에서 존재한다고 가정하고 만들어보았어요.

```rust
cargo run --example atoms_10000 --release
```
초기 모습 (양자역학의 도움 없이도 전자 궤도에 대해 말할 수 있을 것 같은 착각이 듭니다.)

<img src="pic/atoms_10000.gif" width="400" height="400"/>
<br>

시간이 흐른 후의 모습(이 모델은 전자 궤도에 대해 말할 수 없다는 생각이 듭니다.)

<img src="pic/atoms_10000-2.gif" width="400" height="400"/>

<br>

5) 함수의 그래프 그리기

5-1) 1차 함수
```rust
cargo run --example graph_order_1
```

<img src="pic/graph_order_1.png" width="400" height="400"/>

<br>

5-2) 2차 함수
```rust
cargo run --example graph_order_2
```

<img src="pic/graph_order_2.png" width="400" height="400"/>

<br>


5-3) 3차 함수
```rust
cargo run --example graph_order_3
```

<img src="pic/graph_order_3.png" width="400" height="400"/>

<br>

5-4) 4차 함수
```rust
cargo run --example graph_order_4
```

<img src="pic/graph_order_4.png" width="400" height="400"/>

<br>

```rust
fn y_eq_fx(x: f64) -> f64{
    (x + 5.0) * (x + 2.5) * (x - 3.0) * (x - 5.0)
}
```
<br>


5-5) heart 함수
```rust
cargo run --example graph_hert
```

<img src="pic/graph_heart.png" width="400" height="400"/>

<br>

```rust
fn y_eq_fx_1(x: f64) -> f64{
    (1.0 - (x.abs() - 1.0).powf(2.0)).powf(0.5) 
}

fn y_eq_fx_2(x: f64) -> f64{
    (1.0 - x.abs()).acos() -  std::f64::consts::PI 
}
```
<br>
<br>

6) 50개 태양의 움직임(중력만 고려했음)
```rust
cargo run --example solar_n_body
```

<img src="pic/solar_n_body_50.gif" width="400" height="400"/>

<br>
