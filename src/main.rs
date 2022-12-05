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
    let mut state = initial_state().choose().0;
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
use cetkaik_full_state_transition::message::*;
use cetkaik_full_state_transition::state::*;
use cetkaik_full_state_transition::*;

let config = Config::cerke_online_alpha();
let mut state = initial_state().choose().0;
"#,
        test_name
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
        let pure_move_str = format!("{:?}", pure_move).replace("InfAfterStep {", "message::InfAfterStep {");
        writeln!(&mut w, "let pure_move = {};", pure_move_str).unwrap();

        if pure_move.is_none() {
            break;
        }
        writeln!(&mut w, "if pure_move.is_none() {{ return; }}").unwrap();

        let pure_move = pure_move.unwrap();
        writeln!(&mut w, "let pure_move = pure_move.unwrap();").unwrap();
        writeln!(
            &mut w,
            "assert!(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move));"
        )
        .unwrap();

        let hnr_state = match pure_move {
            PureMove::NormalMove(m) => {
                let hnr_state = apply_normal_move(&state, m, config).unwrap().choose().0;
                writeln!(
                    &mut w,
                    r#"
let hnr_state = match pure_move {{
    PureMove::NormalMove(m) => {{
        apply_normal_move(&state, m, config).unwrap().choose().0
    }},
    _ => {{ panic!() }}
}};"#
                )
                .unwrap();
                if very_verbose {
                    writeln!(&mut w, r#"assert_eq!(hnr_state, {:?});"#, &hnr_state).unwrap();
                }
                hnr_state
            }
            PureMove::InfAfterStep(m) => {
                let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose().0;
                let aha_move = ext_state
                    .get_candidates(config)
                    .choose(&mut rng)
                    .copied()
                    .unwrap();
                writeln!(
                    &mut w,
                    r#"
let hnr_state = match pure_move {{
    PureMove::InfAfterStep(m) => {{
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose().0;
        let aha_move = {:?};
        let hnr_state = apply_after_half_acceptance(&ext_state, aha_move, config)
            .unwrap()
            .choose()
            .0;
        
        hnr_state
    }},
    _ => {{ panic!() }}
}};"#,
                    aha_move
                )
                .unwrap();

                let hnr_state = apply_after_half_acceptance(&ext_state, aha_move, config)
                    .unwrap()
                    .choose()
                    .0;

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
    _ => {{ panic!() }}
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
        state = if_tymok;
    }},
    _ => {{ panic!() }}
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
        assert_eq!(if_taxot, {:?});
"#,
                            if_taxot
                        )
                        .unwrap();
                        match if_taxot {
                            IfTaxot::NextSeason(ps) => {
                                writeln!(
                                    &mut w,
                                    r#"
        match if_taxot {{
            IfTaxot::NextSeason(ps) => state = ps.clone().choose().0,
            IfTaxot::VictoriousSide(v) => {{
                panic!()
            }}
        }}
    }},
    _ => {{ panic!() }}
}}
                                "#
                                )
                                .unwrap();
                                state = ps.clone().choose().0
                            }
                            IfTaxot::VictoriousSide(v) => {
                                writeln!(
                                    &mut w,
                                    r#"
        match if_taxot {{
            IfTaxot::NextSeason(ps) => {{ panic!() }},
            IfTaxot::VictoriousSide(v) => {{
                assert_eq!(v, {:?});
                return;
            }}
        }}
    }},
    _ => {{ panic!() }}
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
    _ => {{ panic!() }}
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
    do_match(5, false);
}
