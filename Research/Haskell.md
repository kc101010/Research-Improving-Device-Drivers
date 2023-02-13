## Monad
[Monad (functional programming) - Wikipedia](https://en.wikipedia.org/wiki/Monad_(functional_programming))
Software design pattern in functional programming that combines functions and wraps their return values in a type with additional computation. 

[All About Monads - HaskellWiki](https://wiki.haskell.org/All_About_Monads)
An endofunctor - a function mapping a category to itself - together with 2 natural transformations required to fulfill certain coherence conditions. 

Examples
+ List
+ Maybe type
+ I/O

F#, monads are known as 'workflows' - a way to describe how to get things done. Strategies for solving coding problems that recur often.

A kind of meta-strategy for combining computations into more complex computations. 

Is a way to structure computations in terms of values and sequences of computations using typed values. As sequencing is done by feeding one function into the next, you get some type discipline and computational leverage over what kinds of operations can follow previous operations. 

Metaphor of 'programmable semi-colons'

The monad determines how combined computations form a new computation, freeing the programming from having to manually code the combination each time it is required.

Maybe - represents the type of computations which may fail to return a result. The maybe type suggests a strategy for combining computations which return Maybe values.


With Haskell, Monads are central to the I/O system so understanding the I/O monad will improve code and extend capabilities.

Monads are useful for structuring functional programs with 3 properties that are especially useful.
1. Modularity - Computations can be composed of simpler computations and separate the combination strategy from the computations being performed.
2. Flexibility - Allow functional programs to be more adaptable than equivalent programs written without monads. This is because the monad distills the computational strategy into a single place rather than requiring it be distributed throughout the entire program. 
3. Isolation - Can be used to create imperative-style computational structures which remain safely isolated form main body of functional program. Useful for incorporating side-effects (I/O) and state (which violates referential transparency) into a pure functional language like Haskell.


https://www.haskell.org/tutorial/io.html
Haskell I/O system is purely functional but still holds the expression powers found in conventional programming languages.

Imperative languages carry this out via actions that examine and change the current state with typical actions being reading & setting global variables, writing files, reading input and so on. These actions are still a part of Haskell but separated from the pure functional core of the language.

In Haskell, actions are defined rather than invoked within the expression.