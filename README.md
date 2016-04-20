***Problem Statements:***
-
    -Searching
    -Sorting
    -Concurrency

***Build Tools***
-
    -Cargo
        -Official Build Manager
        -Cargo is a build tool that builds and compiles projects using external files 
        to declare dependencies on libraries known as crates 
        cargo run will fetch crates, build and run the project all in one line.

***Safety in Rust***
-
        -Strong static typing
    
Ownership/Borrowing
=
    -Variables have ownership of memory they are bound to.
    -Memory can only be owned by one binding
        int foo = 10;
        int bar = foo;
    -Legal in other languages, illegal in Rust.
    -Designed with safety in mind.
    -Solution: Borrowing
    -Can declare bindings as references to another binding, or mutable references
    
        let mut foo = 10;
        {
            let mut bar = &mut foo;
             //do things with bar here
        }
        println!("{}", foo); // println borrows foo here 
       
        
    -Curly brackets (to define scope) point to rules of borrowing:
        -Borrow's scope cannot be greater than the original binding's scope
        -Only one type of borrow at a time (mutable/immutable, again safety)
        
    -Borrow Checker big feature of the Rust compiler
    
***Code Highlights***
=

Search
-
    -Generics
        -Type T with trait PartialOrd
    -built-in Option enum
    -passing references to the search function, recieving with unwrap
    
Sort
-
    -Use of generics
    -User defined types for function as an argument. Need a reference to a new type.
    
Concurrency
-
    
    
