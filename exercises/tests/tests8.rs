// I AM NOT DONE

fn main() {
    #[cfg(feature = "pass")]
    {
        return;
    }
    panic!("You should pass the --cfg 'feature=\"pass\"' argument to rustc.");
}
