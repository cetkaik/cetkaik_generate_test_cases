use cetkaik_full_state_transition::message::*;
use cetkaik_full_state_transition::state::*;
use cetkaik_full_state_transition::*;
use rand::prelude::*;
pub struct HandExists {
    pub if_tymok: GroundState,
    pub if_taxot: IfTaxot,
}

#[derive(Clone)]
pub enum TymokOrTaxot {
    Tymok(GroundState),
    Taxot(IfTaxot),
}

fn do_match(max_turn_num: usize, very_verbose: bool) {
    use std::fs::File;
    use std::io::Write;
    let mut w = Vec::new();
    let config = Config::cerke_online_alpha();
    let mut rng = SmallRng::from_entropy();
    let chooser: f64 = rng.gen();
    let mut state = initial_state().choose_by_uniform_random_variable(chooser).0;
    let test_name = rng.gen_range(0u64..=u64::MAX);
    writeln!(
        &mut w,
        r#"
#[test]
fn test{}() {{
use cetkaik_full_state_transition::message::PureMove::*;
use cetkaik_full_state_transition::message::NormalMove::*;
use cetkaik_core::absolute::Coord;
use cetkaik_core::absolute::Row::*;
use cetkaik_core::absolute::Column::*;
use cetkaik_core::Color::*;
use cetkaik_core::Profession::*;
use cetkaik_full_state_transition::Rate::*;
use cetkaik_full_state_transition::message::*;
use cetkaik_full_state_transition::state::*;
use cetkaik_full_state_transition::*;
use cetkaik_core::absolute::Side::*;
use cetkaik_full_state_transition::Season::*;

let config = Config::cerke_online_alpha();
let chooser = {:?};
let mut state = initial_state().choose_by_uniform_random_variable(chooser).0;
"#,
        test_name, chooser
    )
    .unwrap();
    for turn_num in 1..=max_turn_num {
        writeln!(&mut w, "// Turn #{}", turn_num).unwrap();
        let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
        writeln!(
            &mut w,
            "let (hop1zuo1_candidates, candidates) = state.get_candidates(config);"
        )
        .unwrap();

        let pure_move_1 = hop1zuo1_candidates.choose(&mut rng);
        let pure_move_2 = candidates.choose(&mut rng);

        let pure_move = { pure_move_1.or(pure_move_2).cloned() };
        let pure_move_str =
            format!("{:?}", pure_move).replace("InfAfterStep {", "message::InfAfterStep {");
        writeln!(&mut w, "let pure_move = {};", pure_move_str).unwrap();

        if pure_move.is_none() {
            break;
        }
        writeln!(&mut w, "if pure_move.is_none() {{ return; }}").unwrap();

        let pure_move = pure_move.unwrap();
        writeln!(&mut w, "let pure_move = pure_move.unwrap();").unwrap();
        writeln!(
            &mut w,
            r#"
if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {{
    println!("hop1zuo1_candidates: {{:?}}", hop1zuo1_candidates);
    println!("candidates: {{:?}}", candidates);
    println!("pure_move: {{:?}}", pure_move);
    panic!("pure_move not contained in the candidates");
}}
"#
        )
        .unwrap();

        let hnr_state = match pure_move {
            PureMove::NormalMove(m) => {
                let chooser: f64 = rng.gen();
                let hnr_state = apply_normal_move(&state, m, config)
                    .unwrap()
                    .choose_by_uniform_random_variable(chooser)
                    .0;
                writeln!(
                    &mut w,
                    r#"
let hnr_state = match pure_move {{
    PureMove::NormalMove(m) => {{
        let chooser: f64 = {:?};
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    }},
    _ => {{ panic!("expected PureMove::NormalMove") }}
}};"#,
                    chooser
                )
                .unwrap();
                if very_verbose {
                    writeln!(&mut w, r#"assert_eq!(hnr_state, {:?});"#, &hnr_state).unwrap();
                }
                hnr_state
            }
            PureMove::InfAfterStep(m) => {
                let chooser: f64 = rng.gen();
                let ext_state = apply_inf_after_step(&state, m, config)
                    .unwrap()
                    .choose_by_uniform_random_variable(chooser)
                    .0;
                let aha_move = ext_state
                    .get_candidates(config)
                    .choose(&mut rng)
                    .copied()
                    .unwrap();

                let chooser2: f64 = rng.gen();
                let hnr_state = apply_after_half_acceptance(&ext_state, aha_move, config)
                    .unwrap()
                    .choose_by_uniform_random_variable(chooser2)
                    .0;
                writeln!(
                    &mut w,
                    r#"
let hnr_state = match pure_move {{
    PureMove::InfAfterStep(m) => {{
        let chooser = {:?};
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = {:?};
        let chooser2 = {:?};
        let hnr_state = apply_after_half_acceptance(&ext_state, aha_move, config)
            .unwrap()
            .choose_by_uniform_random_variable(chooser2)
            .0;
        
        hnr_state
    }},
    _ => {{ panic!("Expected PureMove::InfAfterStep") }}
}};"#, chooser,
                    aha_move, chooser2
                )
                .unwrap();

                if very_verbose {
                    writeln!(&mut w, r#"assert_eq!(hnr_state, {:?});"#, &hnr_state).unwrap();
                }

                hnr_state
            }
        };
        let resolved = resolve(&hnr_state, config);
        writeln!(&mut w, "let resolved = resolve(&hnr_state, config);").unwrap();
        match &resolved {
            HandResolved::NeitherTymokNorTaxot(s) => {
                state = s.clone();
                writeln!(
                    &mut w,
                    r#"
match &resolved {{
    HandResolved::NeitherTymokNorTaxot(s) => {{
        state = s.clone();
    }},
    _ => {{ panic!("Expected HandResolved::NeitherTymokNorTaxot") }}
}}
"#
                )
                .unwrap();
            }
            HandResolved::HandExists { if_tymok, if_taxot } => {
                let he = HandExists {
                    if_tymok: if_tymok.clone(),
                    if_taxot: if_taxot.clone(),
                };

                let choice = [
                    TymokOrTaxot::Tymok(he.if_tymok.clone()),
                    TymokOrTaxot::Taxot(he.if_taxot.clone()),
                ]
                .choose(&mut rng)
                .cloned()
                .unwrap();

                match choice {
                    TymokOrTaxot::Tymok(s) => {
                        writeln!(
                            &mut w,
                            r#"
match &resolved {{
    HandResolved::HandExists {{ if_tymok, if_taxot }} => {{
        state = if_tymok.clone();
    }},
    _ => {{ panic!("Expected HandResolved::HandExists") }}
}}
"#,
                        )
                        .unwrap();
                        state = s
                    }
                    TymokOrTaxot::Taxot(if_taxot) => {
                        writeln!(
                            &mut w,
                            r#"
match &resolved {{
    HandResolved::HandExists {{ if_tymok, if_taxot }} => {{
        
"#,
                        )
                        .unwrap();
                        match if_taxot {
                            IfTaxot::NextSeason(ps) => {
                                let chooser: f64 = rng.gen();
                                state = ps.clone().choose_by_uniform_random_variable(chooser).0;
                                writeln!(
                                    &mut w,
                                    r#"
        match if_taxot {{
            IfTaxot::NextSeason(ps) => {{
                let chooser = {:?};
                state = ps.clone().choose_by_uniform_random_variable(chooser).0
            }},
            IfTaxot::VictoriousSide(v) => {{
                panic!("Expected IfTaxot::VictoriousSide")
            }}
        }}
    }},
    _ => {{ panic!("Expected HandResolved::HandExists") }}
}}
                                "#,
                                    chooser
                                )
                                .unwrap();
                            }
                            IfTaxot::VictoriousSide(v) => {
                                writeln!(
                                    &mut w,
                                    r#"
        match if_taxot {{
            IfTaxot::NextSeason(ps) => {{ panic!("Expected IfTaxot::NextSeason") }},
            IfTaxot::VictoriousSide(v) => {{
                assert_eq!(v, {:?});
                return;
            }}
        }}
    }},
    _ => {{ panic!("Expected HandResolved::HandExists") }}
}}
                                "#,
                                    v
                                )
                                .unwrap();
                                break;
                            }
                        }
                    }
                }
            }
            HandResolved::GameEndsWithoutTymokTaxot(v) => {
                writeln!(
                    &mut w,
                    r#"
match &resolved {{
    HandResolved::GameEndsWithoutTymokTaxot(v) => {{
        assert_eq!(v, {:?});
        return;
    }},
    _ => {{ panic!("Expected HandResolved::GameEndsWithoutTymokTaxot") }}
}}
"#,
                    v
                )
                .unwrap();
                break;
            }
        }
    }
    writeln!(&mut w, "}}").unwrap();

    let mut f = File::create(format!("src/test{}.rs", test_name)).expect("Unable to create file");
    f.write_all(&w).expect("Unable to write data");
    use std::fs::OpenOptions;
    let mut file_ref = OpenOptions::new()
        .append(true)
        .open("src/lib.rs")
        .expect("Unable to open file");
    file_ref
        .write_all(format!("mod test{};\n", test_name).as_bytes())
        .expect("write failed");
}

fn main() {
    do_match(480, false);
}
