To run Daikon,

 * `touch main.dtrace && cargo run`
 * `java -cp $DAIKONDIR daikon.Daikon main.dtrace main.decls`

The daikon library is included privately in each file for now. They should be identical across
the four files.

Everything in the impl blocks is generated, and everything in the function bodies in main.rs is generated.
