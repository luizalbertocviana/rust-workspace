use instance_generators::generator::{
    generate_instance, DependencyBoundsType, DigraphStructure, GraphParameters,
    LowerBoundDependencyType, UpperBoundDependencyType, WeightingDistribution,
};

/// this generates instances with
/// - weighting uniformly distributed between [0, 50]
/// - G with n \in {30, 60, 90, 120, 150} and density choosen from
/// {0.25, 0.50, 0.75}
/// - D is built as an arborescence with branching factor in {2, 4, 8}
/// - l and u map the edge set to {0, dep - 1} and dep, respectively,
/// not allowing l(e) > u(e)

fn main() {
    let weighting = WeightingDistribution::Uniform((0, 50));
    let ns = [30, 60, 90, 120, 150];
    let densities = [0.25, 0.50, 0.75];
    let branching_factors = [2, 4, 8];

    for n in ns.iter() {
        for d in densities.iter() {
            for b in branching_factors.iter() {
                let g_params = GraphParameters::new(*n, *d);
                let d_params = DigraphStructure::Arborescence(*b);

                let l_param = LowerBoundDependencyType::NearlyStrong;
                let u_param = UpperBoundDependencyType::Weak;
                let lu_param = DependencyBoundsType::Custom(l_param, u_param);

                for i in 0..5 {
                    let instance = generate_instance(&g_params, &d_params, &weighting, &lu_param);

                    let common_suffix = format!("{}_{}_{}-{}", n, d, b, i).to_string();

                    let g_filename = "G_".to_string() + &common_suffix;
                    let d_filename = "D_".to_string() + &common_suffix;
                    let b_filename = "B_".to_string() + &common_suffix;

                    instance.to_files(&g_filename, &d_filename, &b_filename);
                }
            }
        }
    }
}
