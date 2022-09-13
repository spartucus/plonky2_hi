use plonky2::plonk::{circuit_data::CircuitConfig, circuit_builder::CircuitBuilder};
use plonky2::field::goldilocks_field::GoldilocksField;
use plonky2::plonk::config::PoseidonGoldilocksConfig;
use plonky2::iop::witness::{PartialWitness, Witness}; 

type F = GoldilocksField;
type C = PoseidonGoldilocksConfig;

fn main() {
    // 2 * (a^2 + b*c) = d
    // a = 10, b = 5, c = 3, d = 230
    let config = CircuitConfig::standard_recursion_config();
    let mut builder = CircuitBuilder::<F, 2>::new(config.clone());

    // build circuit
    let a_t = builder.add_virtual_target();
    let a2_t = builder.exp_u64(a_t, 2);

    let b_t = builder.add_virtual_target();
    let c_t = builder.add_virtual_target();
    let bc_t = builder.mul(b_t, c_t);

    let add_t = builder.add(a2_t, bc_t);
    let lhs_t = builder.mul_const(GoldilocksField(2), add_t);

    let d_t = builder.add_virtual_target();
    builder.connect(lhs_t, d_t);
    let data = builder.build::<C>();

    // assign witness data
    let mut pw = PartialWitness::<F>::new();
    pw.set_target(a_t, GoldilocksField(10));
    pw.set_target(b_t, GoldilocksField(5));
    pw.set_target(c_t, GoldilocksField(3));
    pw.set_target(d_t, GoldilocksField(230));

    // prove
    let proof = data.prove(pw).unwrap();
    match data.verify(proof) {
        Ok(()) => println!("Proof succeed!"),
        Err(x) => println!("Error with {}", x)
    }
}
