# To Run

Dev
`cargo run mandel.png 4000x3000 -1.20,0.35 -1.0,0.20`

Build release
`cargo build -r`

Benchmark on build
`time target/release/mandelbrot mandel.png 4000x3000 -1.20,0.35 -1.0,0.20`

## Result

![alt text](mandel.png)
