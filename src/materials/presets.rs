use once_cell::sync::Lazy;
use crate::materials::Material;

/// List of built-in materials (Cu, Si, C).
pub static PRESETS: Lazy<Vec<Material>> = Lazy::new(|| {
    vec![
        Material {
            name: "Copper".to_string(),
            atomic_number: 29,
            density_g_cm3: 8.96,
        },
        Material {
            name: "Silicon".to_string(),
            atomic_number: 14,
            density_g_cm3: 2.33,
        },
        Material {
            name: "Carbon".to_string(),
            atomic_number: 6,
            density_g_cm3: 2.0,
        },
    ]
});
