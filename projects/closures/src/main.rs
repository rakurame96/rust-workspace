#[allow(unused)]
fn main() {
    // -------------------- //
    // START OF EXAMPLE: 1
    // -------------------- //
    {
        // regular function
        fn add(x: i32, y: i32) -> i32 {
            x + y
        }

        // f is a pointer to a function
        let f = add;
        println!("{}", f(2, 4));    // this works

        // add function written as closure
        let f = |x: i32, y: i32| {
            x + y
        };
        println!("{}", f(4, 4));

        // Simplified closure because of single expression 
        let f = |x: i32, y: i32| x + y;
        println!("{}", f(4, 8));

        // Closure with automatic type inference
        let f = |x, y| x + y;
        println!("{}", f(6, 8));

        // inline closure
        println!("{}", (|x, y| x + y)(8, 8));

        // Closure with no arguments
        let f = || 2 + 2;
        println!("{}", f());
    }
    // -------------------- //
    // END OF EXAMPLE: 1
    // -------------------- //
    
    // -------------------- //
    // START OF EXAMPLE: 2
    // -------------------- //
    {
        fn add(x: i32, y: i32) -> i32 {
            x + y
        }
        // complex usecase
        fn calc_and_print(x: i32, y:i32, calculator: fn(i32, i32) -> i32) {
            let result = calculator(x, y);
            println!("Ex: 2 -> Result: {}", result);
        }

        calc_and_print(1, 2, add);  // by passing the regular function as a argument
        calc_and_print(2, 4, |x, y| x + y);  // by passing the inline closure as a argument


        // Closure with environment variables (negative/failure case)
        let z = 3;
        // The following Closure does not work because it uses/captures z from the environment
        // Therefore, it cannot act as a function pointer.
        // The Closure consists of the function plus the captured variable.

        // error[E0434]: can't capture dynamic environment in a fn item
        // calc_and_print(1, 2, |x, y| x + y + z);      // uncomment this line to see the error
    }
    // -------------------- //
    // END OF EXAMPLE: 2
    // -------------------- //

    // -------------------- //
    // START OF EXAMPLE: 3
    // -------------------- //
    {    
        fn add(x: i32, y: i32) -> i32 {
            x + y
        }
        // complex usecase with Box
        // '_ -> lifetime elision  ==> means lifetime of a variable is the same as the lifetime of the function
        fn calc_and_print_box(x: i32, y:i32, calculator: Box<dyn Fn(i32, i32) -> i32 + '_>) {
            let result = calculator(x, y);
            println!("Ex: 3 -> Result: {}", result);
        }

        calc_and_print_box(1, 2, Box::new(add));  // by passing the regular function as a argument
        calc_and_print_box(2, 4, Box::new(|x, y| x + y));  // by passing the inline closure as a argument


        // Now we can also pass a closure with capturing environment variables to the underlying function
        let z = 3;
        calc_and_print_box(1, 2, Box::new(|x, y| x + y + z));
    }
    // -------------------- //
    // END OF EXAMPLE: 3
    // -------------------- //

    // -------------------- //
    // START OF EXAMPLE: 4
    // -------------------- //
    {
        // conceptual example of Fn
        struct AdderClosure {
            z: i32,
        }

        trait MyAdder {
            fn add(&self, x: i32, y: i32) -> i32;
        }

        impl MyAdder for AdderClosure {
            fn add(&self, x: i32, y: i32) -> i32 {
                x + y + self.z
            }       
        }

        fn calc_and_print_box_cpt(x: i32, y:i32, calculator: Box<dyn MyAdder>) {
            let result = calculator.add(x, y);
            println!("Ex: 4 -> Result: {}", result);
        }

        let closure = AdderClosure { 
            z: 3 
        };
        calc_and_print_box_cpt(1, 2, Box::new(closure));
    }
    // -------------------- //
    // END OF EXAMPLE: 4
    // -------------------- //

    // -------------------- //
    // START OF EXAMPLE: 5 - FnMut Trait
    // -------------------- //
    {
        let mut result = 0;

        // Closure with mutable borrow
        let mut calc_result = |x, y| {
            result = x + y;
        };
        calc_result(2, 3);
        println!("Ex: 5.1 -> Result: {}", result);

        // Store closure in "FnMut" variable before calling it
        let mut result_calculator: Box<dyn FnMut(i32, i32)> = Box::new(|x, y| {
            result = x + y;
        });
        result_calculator(3, 4);
        drop(result_calculator); // drop the reference
        println!("Ex: 5.2 -> Result: {}", result);
    }
    // -------------------- //
    // END OF EXAMPLE: 5
    // -------------------- //
    
    // -------------------- //
    // START OF EXAMPLE: 6
    // -------------------- //
    {    
        // Closure with consuming iterator
        let numbers_iter = vec![1, 2, 3, 4, 5].into_iter();
        let sum_calculator = move || numbers_iter.sum();
        let result: i32 = sum_calculator();
        println!("Ex: 6.1 -> Result: {}", result); 
        
        // Store closure in "FnOnce" variable before calling it
        let numbers_iter = vec![1, 2, 3, 4, 5].into_iter();
        let sum_calculator: Box<dyn FnOnce() -> i32> = Box::new(move || numbers_iter.sum());
        let result: i32 = sum_calculator();
        println!("Ex: 6.2 -> Result: {}", result); 
    }
    // -------------------- //
    // END OF EXAMPLE: 6
    // -------------------- //
    
    // -------------------- //
    // START OF EXAMPLE: 7
    // -------------------- //
    {
        // Disjoint Closure 
        #[derive(Debug)]
        struct Lentil {
            size: f32,
        }

        struct CinderellaTask {
            lentils: Vec<Lentil>,
            eat: Box<dyn Fn(&Lentil) -> bool>,
        }

        impl CinderellaTask {
            fn sort_lentils(&mut self) {
                // The following line works in Rust 2021, but it does not work in Rust 2018.
                // In the 2018 edition, the closure captures the entire 'self' object.
                // In the 2021 edition, the closure just captures 'self.eat'.
                // So, using 'self.lentils' outside is now possible
                self.lentils.retain(|l| !(self.eat)(l));
            }
        }

        let lentils = vec![
            Lentil { size: 1.0 }, 
            Lentil { size: 2.0 }, 
            Lentil { size: 3.0 },
            Lentil { size: 4.0 }, 
            Lentil { size: 5.0 }, 
            Lentil { size: 6.0 }, 
            Lentil { size: 7.0 }, 
            Lentil { size: 1.0 }
        ];

        let mut task = CinderellaTask {
            lentils,
            eat: Box::new(|l| l.size < 5.0),
        };

        task.sort_lentils();
        println!("Ex: 7 -> {:?}", task.lentils)
    }
    // -------------------- //
    // END OF EXAMPLE: 7
    // -------------------- //
}
