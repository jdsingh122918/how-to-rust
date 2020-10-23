### Counter `iterator`
This is an example of an `iterator` which only counts upto 5. See how we only have to implement 
the `next` function as part of `Iterator` implementation of counter. Because `Counter` is an 
implementation of `iterator` `trait`, it can use various `methods` of `Iterator` like `zip`. 
Check out the unit test `using_other_iterator_trait_methods`.