mod random_player;
use cetkaik_core::absolute::NonTam2Piece;
use cetkaik_core::absolute::Side;
use cetkaik_full_state_transition::message::*;
use cetkaik_full_state_transition::state::*;
use cetkaik_full_state_transition::*;
use random_player::*;

fn do_match(
    config: Config,
    ia_player: &mut RandomPlayer,
    a_player: &mut RandomPlayer,
    quiet: bool,
) {
    let mut state = initial_state().choose().0;
    let mut turn_count = 0;
    loop {
        if !quiet {
            fn to_s(v: &[NonTam2Piece]) -> String {
                let mut s = String::new();
                for (idx, &e) in v.iter().enumerate() {
                    if idx > 0 {
                        s += " ";
                    }
                    s += &format!("{}", e);
                }
                s
            }
            println!(
                "{}, Turn: {:?}, Season: {:?}, Scores: (IA:{}, A:{}), hop1zuo1: (IA: {}, A: {})",
                turn_count,
                state.whose_turn,
                state.season,
                state.scores.ia(),
                state.scores.a(),
                to_s(&state.f.ia_side_hop1zuo1),
                to_s(&state.f.a_side_hop1zuo1),
            );
        }
        let searcher: &mut RandomPlayer = match state.whose_turn {
            Side::IASide => ia_player,
            Side::ASide => a_player,
        };
        let pure_move = searcher.search(&state);
        if pure_move.is_none() {
            break;
        }
        let pure_move = pure_move.unwrap();
        if !quiet {
            println!("Move: {:?}", pure_move);
        }
        let hnr_state = match pure_move {
            PureMove::NormalMove(m) => apply_normal_move(&state, m, config).unwrap().choose().0,
            PureMove::InfAfterStep(m) => {
                let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose().0;
                let aha_move = searcher.search_excited(&m, &ext_state).unwrap();
                if !quiet {
                    println!("Move(excited): {:?}", aha_move);
                }
                apply_after_half_acceptance(&ext_state, aha_move, config)
                    .unwrap()
                    .choose()
                    .0
            }
        };
        let resolved = resolve(&hnr_state, config);
        match &resolved {
            HandResolved::NeitherTymokNorTaxot(s) => state = s.clone(),
            HandResolved::HandExists { if_tymok, if_taxot } => {
                let he = HandExists {
                    if_tymok: if_tymok.clone(),
                    if_taxot: if_taxot.clone(),
                };
                match searcher.search_hand_resolved(&he).unwrap() {
                    TymokOrTaxot::Tymok(s) => state = s,
                    TymokOrTaxot::Taxot(t) => {
                        if !quiet {
                            println!("Taxot!");
                        }
                        match t {
                            IfTaxot::NextSeason(ps) => state = ps.clone().choose().0,
                            IfTaxot::VictoriousSide(v) => {
                                println!("Won: {:?}", v);
                                break;
                            }
                        }
                    }
                }
            }
            HandResolved::GameEndsWithoutTymokTaxot(v) => {
                println!("Won: {:?}", v);
                break;
            }
        }
        turn_count += 1;
    }
}

fn main() {
    let config = Config::cerke_online_alpha();

    do_match(
        config,
        &mut RandomPlayer::new(config),
        &mut RandomPlayer::new(config),
        false,
    );
}
