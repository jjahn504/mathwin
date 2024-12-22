# mathwin ( feat: minifb of Rust )

I want to make Light GUI for Mathematical Play based on minifb crate with Rust programming language.

minifb 크레이트는 크기가 작아서 작동 원리를 이해하기 쉽고.

처리 속도가 빠른 편이라고 생각합니다.

이러한 minifb 크레이트를 기반으로... 

대량의 데이터 를 빠르게 처리하여,

2차원 애니메이션이나 그래프로 볼 수 있는 GUI 를 만드는 중입니다.



## To run this example:

```
cargo run --example heart
```
<img src="pic/heart.gif" width="400" height="400"/>
<img src="pic/heart.gif" style="zoom:50%;" />원 그리는 기능 테스트




```
cargo run --example rotation
```

<img src="pic/rotation.gif" style="zoom:50%;" /> 선 그리는 기능 테스트




```rust
cargo run --example atoms
```

<img src="pic/atoms.gif" style="zoom:50%;" /> 'R' 키를 누르면 리셋 됨.



```rust
cargo run --example atoms_10000 --release
```

<img src="pic/atoms_10000.gif" style="zoom:50%;" /> 초기 모습

<img src="pic/atoms_10000-2.gif" alt="atoms_10000-2" style="zoom:50%;" />시간이 흐른 후의 모습



[.gitignore 파일의 내용]

target/
