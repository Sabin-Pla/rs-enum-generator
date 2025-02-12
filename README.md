## Enum Generator

A crate that demos templated enum definitions. Currently only 2 types of basic templating patterns are supported: 
	- counting integers
	- going through the alphabet (either upper or lowercase characters)

The part after the colon is used to tell the macro what part of the variant name is supposed to be incremented every variant (it takes the place of the `_` token)


The part of the variant name that changes may be in the middle of the identifier, in which case it would need to be added to the right side of the `_` token. 


All types enum variants are supported, though the examples shown below use unit variants.


Variants generated with this macro are all generated with the same fields.

```keyboard_example_program.rs
use enum_generator::generate_enum;

// generate enum of 24 letter variants:

generate_enum!{ 
#[derive(Debug)]
enum FunctionKey {
    F1: F _,
    ...
    F24
}
}

// generate enum of 26 letter variants:
generate_enum!{ 
#[derive(Debug)]
enum LetterKey {
    LetterA: Letter _,
    ...
    LetterZ
}
}


```