pub use enum_generator_macros::generate_enum;

#[test]
fn run_tests() {

	// demo test case 1 
	// generate enum variants with a template.
	// continues the pattern shown (either counting up integers or going through the alphabet)
	// starting index does not necessarily have to be A or 1.

	generate_enum!{ 
		#[derive(Debug)]
		enum FunctionKey {
			F1: F _,
			...
			F24
		}
	}
	// expands to: 
	// #[derive(Debug)] enum FunctionKey
	// {
    //   F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14, F15, F16,
    //   F17, F18, F19, F20, F21, F22, F23, F24
	// }

	let e = FunctionKey::F12;

	// demo test case 2
	generate_enum!{ 
		#[derive(Debug)]
		enum LetterKey {
			LetterA: Letter _,
			...
			LetterZ
		}
	}

	let e = LetterKey::LetterZ;
	println!("{:?}", e);

	// test case - suffix
	generate_enum!{ 
		#[derive(Debug)]
		enum ExampleEnum {
			Prefix1Suffix:  Prefix _ Suffix,
			...
			Prefix5Suffix
		}
	}


	let e = ExampleEnum::Prefix3Suffix;
	let e2 = ExampleEnum::Prefix4Suffix;	
	assert!(e as u8 != e2 as u8);

	// test case - capital letter & generics & struct
	generate_enum!{ 
		#[derive(Debug)]
		enum ExampleEnum3<G> {
			PrefixASuffix{ g: G } :  Prefix _ Suffix,
			...
			PrefixZSuffix{ g: G } 
		}
	}

	let e = ExampleEnum3::PrefixZSuffix::<usize>{ g: 20 } ;	


	// test case - lowercase letter & generics & struct
	generate_enum!{ 
		#[derive(Debug)]
		enum ExampleEnum4<G> {
			PrefixaSuffix{ g: G } :  Prefix _ Suffix,
			...
			PrefixzSuffix{ g: G } 
		}
	}

	let e = ExampleEnum4::PrefixqSuffix::<usize>{ g: 20 } ;	

}