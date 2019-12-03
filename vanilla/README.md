# Vanilla
 
## `Box<dyn Error>`

This method allow to transform automatically using the `?` the various errors into a `Box` retaining the informations 
and deal wih the heterogeneous errors.

The `?` use the `From` trait to box the error and leverage the dynamic trait object `Box<dyn Error>` to store all the errors.

### Problem

The typing of the errors doesnt generate problems anymore but contextualising errors or throwing custom errors need 
some boilerplate codes.


### Failed attempt 
 - [Test using impl Trait for Error](testimplerror.md)