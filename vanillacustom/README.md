# Vanilla wih custom error definition
 
## `Box<dyn Error>`

This method allow to transform automatically using the `?` the various errors into a `Box` retaining the informations 
and deal wih the heterogeneous errors.

The `?` use the `From` trait to box the error and leverage the dynamic trait object `Box<dyn Error>` to store all the errors.

## Defining custom error

To be able to trace and iprove error management we can create custom errors in our program.

The vanilla way to do it is by defining a struct modelising the information for the error and deriving the traits `Error`.

If not redefining anything `Debug` and `Display` by transitivity of the default implementation [impl example](src/main.rs#L62).

## Problems

_Boilerplate code_ : Defining error have some boilerplate code that can be painful to read

_Cause error_ : Managing the cause of errors to chain them and have all the needed contexts is not really straightforward

_Typing error_ : During impl for the cause I ran in the problem of typing a source error `impl` into a `Box<dyn Error>` even knowing why I can' it's messing with the [error creation code](src/main.rs#L69) and can't be factorised

_Mapping verbosity_ : Creating its own error to contextualize an error in a `Result` need to `map_err` and can be quite verbose due to various implementations of the errors