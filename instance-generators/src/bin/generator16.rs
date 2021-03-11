use instance_generators::generator::{
    DependencyBoundsType, DigraphStructure, GraphParameters, WeightingDistribution, generate_instance
};

/// this generates instances with
/// - weighting chosen from [0, 50] with 80% bias to [15, 35]
/// - G with n \in {30, 60, 90, 120, 150} and density choosen from
/// {0.25, 0.50, 0.75}
/// - D is built as an dag with density in {0.25, 0.50, 0.75}
/// - l and u are constant zero functions

fn main() {
    let weighting = WeightingDistribution::Biased((0, 50), 0.80, (15, 35));
    let ns = [30, 60, 90, 120];
    let densities = [0.25, 0.50, 0.75];
    let dag_densities = [0.25, 0.50, 0.75];

    for n in ns.iter() {
        for d in densities.iter() {
            for dag_d in dag_densities.iter() {
                let g_params = GraphParameters::new(*n, *d);
                let d_params = DigraphStructure::Dag(*dag_d);
                let lu_params = DependencyBoundsType::Constant(0, 0);

                for i in 0..5 {
                    let instance = generate_instance(&g_params, &d_params, &weighting, &lu_params);

                    let common_suffix = format!("{}_{}_{}-{}", n, d, dag_d, i).to_string();

                    let g_filename = "G_".to_string() + &common_suffix;
                    let d_filename = "D_".to_string() + &common_suffix;
                    let b_filename = "B_".to_string() + &common_suffix;

                    instance.to_files(&g_filename, &d_filename, &b_filename);
                }
            }
        }
    }
}
