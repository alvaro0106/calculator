### Calculator

Vincular Github con Subl
https://docs.github.com/es/get-started/getting-started-with-git/associating-text-editors-with-git


### Execute program

```
cargo run -- 1 + 1
```

### step 5. analyzer string in number

Method parse convert a str a int

### step 6. To perform arithmetic operation

rust to use the operations standard sum, mult, and div.
for operations manager define a functions operate that take 3: operation whit char  and 2 numbers whit f32

now to call operate with operator, first convert in char this to do method chars, in the struct String.

### formating the exit
to get the result to want, the variables primer_numero, segundo_numero y resultado should formating, using format! macro to create a String.


### gather all

for create code of a executable binary:

```
cargo build --release
```
the flag -- release indicate to cargo that compil the binary in mode -- release

the binary this constructed in director target/release. for execute binary and test the application: 

```
target/release/calculator 1 + 1
```

 