use std::cell::RefCell;

thread_local! {
    static CALLED_TASKS: RefCell<Vec<String>> = RefCell::new(Vec::new());
}
