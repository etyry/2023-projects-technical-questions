1.  Identify one problem in the below code block, will this code compile? Discuss the related Rust feature regarding the problem you have identified, why does Rust choose to include this feature? A few sentences are good enough.

    ```rust
        let data = vec![1, 2, 3];
        let my_ref_cell = RefCell::new(69);
        let ref_to_ref_cell = &my_ref_cell;

        std::thread::spawn(move || {

            println!("captured {data:?} by value");

            println!("Whats in the cell?? {ref_to_ref_cell:?}")

        }).join().unwrap();
    ```

    A: I believe the problem has to do with `std::thread::spawn`. The program is creating a new thread, but the code won't compile since `Sync` is not implmemented for `RefCell<i32>`, which occurs since its unsafe to share `RefCell` among multiple threads. Since `std::thread::spawn` is creating a new thread, alongside the thread the program starts off with, `RefCell` could potentially be shared across the threads and cause a data race.

2.  Shortly discuss, when modelling a response to a HTTP request in Rust, would you prefer to use `Option` or `Result`?

    A: Probably `Result` since it either returns an okay or error result, whereas the `Option` type could contain none. For a HTTP request this would be not be good since you want to see whether the HTTP response was successful or not.

3.  In `student.psv` there are some fake student datas from UNSW CSE (no doxx!). In each row, the fields from left to right are

    - UNSW Course Code
    - UNSW Student Number
    - Name
    - UNSW Program
    - UNSW Plan
    - WAM
    - UNSW Session
    - Birthdate
    - Sex

    Write a Rust program to find the course which has the highest average student WAM. **Write your program in the cargo project q3**.

