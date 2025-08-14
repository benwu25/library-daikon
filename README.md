To run Daikon,

 * `touch main.dtrace && cargo run`
 * `java -cp $DAIKONDIR/daikon.jar daikon.Daikon main.dtrace main.decls`

The daikon library is included privately in each file for now. They should be identical across
the four files.

Every impl block is generated, and every function body (except for main) in main.rs is generated.
