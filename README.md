# RustConcurrent
This some practice for the concurrency subject. On this concurrent
we are simulating the popular fable "the Turoise and the Rabbit".
Each object or element(Turtoise/Rabbit) is running in its own thread.
The progress or advancement of the character will depend on a random
choice with a weight or probability. On this implementation will slip
or move forward. The probability and movement of the characters if based
on the following table:

![random movement](https://github.com/H3cth0r/RustConcurrent/blob/main/resources/table_movement.png?raw=true)


Each time the animal moves, it will get printed on the terminal.

For this implementation we also made use of Mutex for accessing or
sharing some space on the memory; this way the threads can access 
some value and evaluate it.

In the following video you'll see the program running.

https://user-images.githubusercontent.com/43997408/169716908-6cf3c40f-310a-41bc-ac5c-49447578343f.mp4


## BUILD & RUN
Rust and cargo makes it easy to build and run a project. In this case
the directory of the cargo project is named **RabbitNTurtle**, so
firts of all just must go into the direcotory. After moving to the
directory, you must type the following commands that will let you
build and run the project.
```
cargo build
cargo run
```

This will automatically run the proyect. Just make sure you have the
corrent rust installation for using cargo and you'll be all set.
