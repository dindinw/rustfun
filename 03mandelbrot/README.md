```
$ time target/release/mandelbrot mandel.png 4000x3000 -1.20,0.35 -1,0.20 fast

real    0m3.559s
user    0m11.968s
sys     0m0.044s

$ time target/release/mandelbrot mandel.png 4000x3000 -1.20,0.35 -1,0.20 slow

real    0m11.701s
user    0m11.625s
sys     0m0.056s
```
