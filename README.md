# puzzle-helper
Simple math for killer sudoku and calcudoku puzzlers


```shell
$ cargo build
   Compiling puzzle-helper v0.1.0 (/Users/mnp/prj/puzzle-helper)
    Finished dev [unoptimized + debuginfo] target(s) in 0.48s

$ ./target/debug/puzzle-helper
>>
Help:
   + a b c...          - sum of arguments
   f n                 - factorize n
   p count max n       - partitions of n
>> p 3 8 20
  8 8 4
  8 7 5
  8 6 6
  7 7 6
>> f 124
  2 2 31
>> + 12 3 41
  56
```
