use waterloo_crypto::CongruenceSolver;

fn main() {
    let mut solver = CongruenceSolver::new();
    solver.add_congruence(3, 7);
    solver.add_congruence(4, 9);
    let (sol, modulo) = solver.get_sol();
    assert!(solver.validate());
    println!("Sol: {sol:?}, Mod: {modulo}");

    let mut solver = CongruenceSolver::new();
    solver.add_congruence(13, 71);
    solver.add_congruence(41, 97);
    let (sol, modulo) = solver.get_sol();
    assert!(solver.validate());
    println!("Sol: {sol:?}, Mod: {modulo}");

    let mut solver = CongruenceSolver::new();
    solver.add_congruence(4, 7);
    solver.add_congruence(5, 8);
    solver.add_congruence(11, 15);
    let (sol, modulo) = solver.get_sol();
    assert!(solver.validate());
    println!("Sol: {sol:?}, Mod: {modulo}");
}
