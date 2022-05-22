# RustConcurrent
This some practice for the concurrency subject. On this concurrent
we are simulating the popular fable "the Turoise and the Rabbit".
Each object or element(Turtoise/Rabbit) is running in its own thread.
The progress or advancement of the character will depend on a random
choice with a weight or probability. On this implementation will slip
or move forward. The probability and movement of the characters if based
on the following table:


Each time the animal moves, it will get printed on the terminal.

For this implementation we also made use of Mutex for accessing or
sharing some space on the memory; this way the threads can access 
some value and evaluate it.

In the following video you'll see the program running.
