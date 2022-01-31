# Type inference engine

A simple type inference engine with a CLI to play with.

## Description

Given a set of constraints type_solver finds a solution by unification.
It contains a REPL for debugging purposes, but is meant to be used as an API.

## Getting Started

### Dependencies
In order to use this library you should be able to compile Rust. I recommend using Cargo, you can find how to install and
run this tool at the [Rust official site](https://www.rust-lang.org).
### Using the CLI mode

* Clone the repository
* Go inside your local repo
* Run ``` cargo build --release ```
* Inside target/release run the type_solver binary.
* Use the ``` :help ``` command to see all the available commands
#### Writing types and type constraints 
This engine accepts three kinds of types, their textual form are:<br />
**Constants**: they are written as ```C(a)``` where *a* is a non-negative integer.<br />
**Variables**: they are written as ```V(a)``` where *a* is a non-negative integer.<br />
**Functions**: they are written as ```T1 -> T2``` where *T1* and *T2* are well-formed types.<br />
Writing constraints is as simple as writing ```T1 = T2``` where *T1* and *T2* are well-formed types.
#### Execution example
```
|> ./type_solver <== we are inside the target/release folder
<>:load ../../tests/test.t <== you can find this file in the repo!
Current state:
[0] V(0) = V(1)
[1] V(1) = C(0) -> V(3)
[2] V(0) = C(0) -> V(4)
[3] V(4) = C(1)
<>:infer
Current state:
[0] V(0) = C(0) -> C(1)
[1] V(1) = C(0) -> C(1)
[2] V(3) = C(1)
[3] V(4) = C(1)
<>:exit
```

### Embedding the engine inside your project
The idea is that programming language enthusiasts could use this engine as a type-inference back-end for their languages.
Right now this project is not uploaded to crates.io, so cloning the repo and adding it to the workspace is needed.<br />
Once the engine is embedded in your project you could focus in the *solver_interface* module:<br />
**solver_interface/ir**: defines the IR used by the engine, in the future I hope to add macros for easily writing the IR
in your front-end. Right now you can write the IR directly or in textual form using the From<&str> functionality.<br />
**solver_interface/blackboard**: defines the struct *BlackBoard* which is used to communicate with the engine. Your front-end can 
add constraints, remove constraints and apply the inference algorithm to them. It also returns a new  constant id or variable id 
when needed.<br />
#### Example
Assuming you receive the following code in your project.
```
x = 2;
a = f(x);
b = a + 2;
```
Your program is now in charge of producing the constraints that will be solved by this engine. First, types should be assigned.
```
x: V(0)
N: C(0), where N is an integer
a: V(1)
f: V(2)
b: V(3)
```
Now, the front-end needs to produce the constraints. A valid set of constraints could be the following one:
```
x = 2; ==> V(0) = C(0)
b = f(x); ==> V(2) = V(4) -> V(5), V(0) = V(4), V(3) = V(5)
b = a + 2; ==> V(1) = C(0), V(3) = C(0) 
```
Our set of constraints is then:
```
V(0) = C(0)
V(2) = V(4) -> V(5)
V(0) = V(4)
V(3) = V(5)
V(1) = C(0)
V(3) = C(0) 
```
When given to the engine, the solution being returned is:
```
[0] V(0) = C(0)
[1] V(2) = C(0) -> C(0)
[2] V(4) = C(0)
[3] V(3) = C(0)
[4] V(1) = C(0)
[5] V(5) = C(0)
```

## Future work
This project is miles away of being production ready and, to be honest, I don't think that is my goal.
As of now I want this project to be an easy-to-use type inference engine where adding modifications does not require lots of knowledge.<br />
In future iterations I will focus on simplifying the process of writing constraints through the API.
## Authors

Herme Garcia, @hermeGarcia on GitHub.

## License

This project is licensed under the MIT License - see the LICENSE.md file for details