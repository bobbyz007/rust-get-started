use concurrency::utils::{rayon_join, rayon_on_off, rayon_par_iter};

fn main() {
    rayon_par_iter();
    rayon_on_off(1_000_000);
    rayon_join(12);
    // rayon_on_off(100_000_000);
    // rayon_join(42);
}