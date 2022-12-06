
#[test]
fn test12318032663907906532() {
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
let chooser = 0.9928221238907486;
let mut state = initial_state().choose_by_uniform_random_variable(chooser).0;

// Turn #1
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(TamMoveNoStep { src: Coord(O, Z), first_dest: Coord(U, Z), second_dest: Coord(U, X) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.9112566570235758;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #2
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(IA, T), step: Coord(AU, T), dest: Coord(AU, Z) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.5444269844462769;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #3
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(I, Z), step: Coord(I, T), planned_direction: Coord(Y, T) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::InfAfterStep(m) => {
        let chooser = 0.031348805018039605;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: None };
        let chooser2 = 0.5853197672318521;
        let hnr_state = apply_after_half_acceptance(&ext_state, aha_move, config)
            .unwrap()
            .choose_by_uniform_random_variable(chooser2)
            .0;
        
        hnr_state
    },
    _ => { panic!("Expected PureMove::InfAfterStep") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #4
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(TamMoveStepsDuringFormer { src: Coord(U, X), step: Coord(I, C), first_dest: Coord(U, C), second_dest: Coord(O, C) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.27745298333858814;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #5
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(E, L), step: Coord(I, L), planned_direction: Coord(O, L) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::InfAfterStep(m) => {
        let chooser = 0.14068686454724655;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: None };
        let chooser2 = 0.9096078074713803;
        let hnr_state = apply_after_half_acceptance(&ext_state, aha_move, config)
            .unwrap()
            .choose_by_uniform_random_variable(chooser2)
            .0;
        
        hnr_state
    },
    _ => { panic!("Expected PureMove::InfAfterStep") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #6
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(TamMoveNoStep { src: Coord(O, C), first_dest: Coord(O, M), second_dest: Coord(O, C) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.21069937840029385;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #7
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(I, C), step: Coord(O, C), dest: Coord(AI, C) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.48422003719321727;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #8
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(TamMoveNoStep { src: Coord(O, C), first_dest: Coord(U, X), second_dest: Coord(O, Z) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.7619036270121498;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #9
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveFromHopZuo { color: Huok2, prof: Kauk2, dest: Coord(Y, C) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.9290880991513821;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #10
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(TamMoveStepsDuringLatter { src: Coord(O, Z), step: Coord(I, X), first_dest: Coord(U, X), second_dest: Coord(E, C) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.8080441885714559;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #11
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(E, P), step: Coord(A, P), dest: Coord(E, P) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.9020695104213078;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #12
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(TamMoveNoStep { src: Coord(E, C), first_dest: Coord(I, C), second_dest: Coord(U, X) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.8868001821789901;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #13
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveSrcDst { src: Coord(AI, C), dest: Coord(AI, M) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.7976873535123847;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #14
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(AU, T), step: Coord(AI, N), planned_direction: Coord(O, K) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::InfAfterStep(m) => {
        let chooser = 0.3358463464897563;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: Some(Coord(O, K)) };
        let chooser2 = 0.4162527660331923;
        let hnr_state = apply_after_half_acceptance(&ext_state, aha_move, config)
            .unwrap()
            .choose_by_uniform_random_variable(chooser2)
            .0;
        
        hnr_state
    },
    _ => { panic!("Expected PureMove::InfAfterStep") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #15
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveFromHopZuo { color: Kok1, prof: Kauk2, dest: Coord(U, T) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.45803288950665844;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #16
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(AU, P), step: Coord(AI, P), planned_direction: Coord(AI, M) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::InfAfterStep(m) => {
        let chooser = 0.8421187316887933;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: Some(Coord(AI, M)) };
        let chooser2 = 0.9433741452348314;
        let hnr_state = apply_after_half_acceptance(&ext_state, aha_move, config)
            .unwrap()
            .choose_by_uniform_random_variable(chooser2)
            .0;
        
        hnr_state
    },
    _ => { panic!("Expected PureMove::InfAfterStep") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #17
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(I, X), step: Coord(I, Z), dest: Coord(U, Z) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.8756637216501494;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #18
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveFromHopZuo { color: Huok2, prof: Kauk2, dest: Coord(AU, C) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.17924106983174715;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #19
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(TamMoveNoStep { src: Coord(U, X), first_dest: Coord(O, Z), second_dest: Coord(Y, T) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.7162021668315082;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #20
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(AU, L), step: Coord(AI, L), planned_direction: Coord(Y, L) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::InfAfterStep(m) => {
        let chooser = 0.683318348989651;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: Some(Coord(O, L)) };
        let chooser2 = 0.1952070229658156;
        let hnr_state = apply_after_half_acceptance(&ext_state, aha_move, config)
            .unwrap()
            .choose_by_uniform_random_variable(chooser2)
            .0;
        
        hnr_state
    },
    _ => { panic!("Expected PureMove::InfAfterStep") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #21
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(TamMoveStepsDuringLatter { src: Coord(Y, T), step: Coord(U, Z), first_dest: Coord(O, Z), second_dest: Coord(U, X) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.8530645448229276;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #22
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveSrcDst { src: Coord(AU, Z), dest: Coord(AU, T) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.18930515346499255;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #23
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(TamMoveNoStep { src: Coord(U, X), first_dest: Coord(I, C), second_dest: Coord(E, C) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.056872759529893346;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #24
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(AI, M), step: Coord(AI, X), dest: Coord(Y, X) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.9053141932235843;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #25
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(E, P), step: Coord(E, M), planned_direction: Coord(IA, K) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::InfAfterStep(m) => {
        let chooser = 0.5532237174874413;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: Some(Coord(U, X)) };
        let chooser2 = 0.4736955755811765;
        let hnr_state = apply_after_half_acceptance(&ext_state, aha_move, config)
            .unwrap()
            .choose_by_uniform_random_variable(chooser2)
            .0;
        
        hnr_state
    },
    _ => { panic!("Expected PureMove::InfAfterStep") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #26
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(TamMoveStepsDuringFormer { src: Coord(E, C), step: Coord(E, X), first_dest: Coord(E, C), second_dest: Coord(I, C) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.5620119796115018;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #27
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(I, M), step: Coord(I, C), dest: Coord(I, M) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.6005887813780896;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #28
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(Y, X), step: Coord(E, X), planned_direction: Coord(A, C) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::InfAfterStep(m) => {
        let chooser = 0.1381340544369991;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: None };
        let chooser2 = 0.9688899818737985;
        let hnr_state = apply_after_half_acceptance(&ext_state, aha_move, config)
            .unwrap()
            .choose_by_uniform_random_variable(chooser2)
            .0;
        
        hnr_state
    },
    _ => { panic!("Expected PureMove::InfAfterStep") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #29
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(TamMoveStepsDuringLatter { src: Coord(I, C), step: Coord(U, X), first_dest: Coord(U, C), second_dest: Coord(I, C) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.07764031953737227;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #30
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveSrcDst { src: Coord(AU, T), dest: Coord(AU, N) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.9703052036215722;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #31
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(E, M), step: Coord(I, C), planned_direction: Coord(O, P) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::InfAfterStep(m) => {
        let chooser = 0.2672738303166423;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: Some(Coord(U, M)) };
        let chooser2 = 0.29894239909841624;
        let hnr_state = apply_after_half_acceptance(&ext_state, aha_move, config)
            .unwrap()
            .choose_by_uniform_random_variable(chooser2)
            .0;
        
        hnr_state
    },
    _ => { panic!("Expected PureMove::InfAfterStep") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #32
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(TamMoveStepsDuringFormer { src: Coord(I, C), step: Coord(U, M), first_dest: Coord(O, M), second_dest: Coord(Y, P) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.7568738703432135;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #33
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(U, X), step: Coord(AI, P), planned_direction: Coord(U, X) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::InfAfterStep(m) => {
        let chooser = 0.7862110965757589;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: Some(Coord(U, X)) };
        let chooser2 = 0.9765282550115443;
        let hnr_state = apply_after_half_acceptance(&ext_state, aha_move, config)
            .unwrap()
            .choose_by_uniform_random_variable(chooser2)
            .0;
        
        hnr_state
    },
    _ => { panic!("Expected PureMove::InfAfterStep") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #34
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(TamMoveStepsDuringLatter { src: Coord(Y, P), step: Coord(Y, C), first_dest: Coord(AI, M), second_dest: Coord(O, M) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.8945417308862037;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #35
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(E, X), step: Coord(A, C), dest: Coord(E, X) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.6905893819714536;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #36
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(AU, N), step: Coord(AI, N), dest: Coord(Y, L) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.5745215055112994;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #37
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(TamMoveNoStep { src: Coord(O, M), first_dest: Coord(Y, M), second_dest: Coord(O, C) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.3336666199161249;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #38
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(Y, X), step: Coord(Y, C), planned_direction: Coord(IA, T) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::InfAfterStep(m) => {
        let chooser = 0.06745820604786212;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: None };
        let chooser2 = 0.6193230853420326;
        let hnr_state = apply_after_half_acceptance(&ext_state, aha_move, config)
            .unwrap()
            .choose_by_uniform_random_variable(chooser2)
            .0;
        
        hnr_state
    },
    _ => { panic!("Expected PureMove::InfAfterStep") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #39
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(TamMoveStepsDuringFormer { src: Coord(O, C), step: Coord(U, X), first_dest: Coord(I, C), second_dest: Coord(E, M) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.618949800878656;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #40
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveSrcDst { src: Coord(O, L), dest: Coord(O, X) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.63321991314542;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #41
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(E, K), step: Coord(E, L), planned_direction: Coord(E, K) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::InfAfterStep(m) => {
        let chooser = 0.9839143127913418;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: None };
        let chooser2 = 0.8034068013821183;
        let hnr_state = apply_after_half_acceptance(&ext_state, aha_move, config)
            .unwrap()
            .choose_by_uniform_random_variable(chooser2)
            .0;
        
        hnr_state
    },
    _ => { panic!("Expected PureMove::InfAfterStep") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #42
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveSrcDst { src: Coord(Y, X), dest: Coord(Y, N) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.303769726250277;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #43
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(E, L), step: Coord(A, L), planned_direction: Coord(E, L) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::InfAfterStep(m) => {
        let chooser = 0.1499031721735381;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: Some(Coord(E, L)) };
        let chooser2 = 0.5856064440510934;
        let hnr_state = apply_after_half_acceptance(&ext_state, aha_move, config)
            .unwrap()
            .choose_by_uniform_random_variable(chooser2)
            .0;
        
        hnr_state
    },
    _ => { panic!("Expected PureMove::InfAfterStep") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #44
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(TamMoveStepsDuringLatter { src: Coord(E, M), step: Coord(A, M), first_dest: Coord(E, C), second_dest: Coord(E, M) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.1290912928562843;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #45
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(U, X), step: Coord(E, M), planned_direction: Coord(E, P) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::InfAfterStep(m) => {
        let chooser = 0.9756753042406208;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: None };
        let chooser2 = 0.7366489544459549;
        let hnr_state = apply_after_half_acceptance(&ext_state, aha_move, config)
            .unwrap()
            .choose_by_uniform_random_variable(chooser2)
            .0;
        
        hnr_state
    },
    _ => { panic!("Expected PureMove::InfAfterStep") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #46
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(TamMoveStepsDuringLatter { src: Coord(E, M), step: Coord(A, M), first_dest: Coord(E, C), second_dest: Coord(E, M) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.8591150821491141;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #47
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(A, T), step: Coord(A, Z), dest: Coord(E, Z) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.5546235166823441;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #48
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(AU, M), step: Coord(AU, C), planned_direction: Coord(AU, M) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::InfAfterStep(m) => {
        let chooser = 0.14276564106947764;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: Some(Coord(AU, M)) };
        let chooser2 = 0.44204296323238834;
        let hnr_state = apply_after_half_acceptance(&ext_state, aha_move, config)
            .unwrap()
            .choose_by_uniform_random_variable(chooser2)
            .0;
        
        hnr_state
    },
    _ => { panic!("Expected PureMove::InfAfterStep") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #49
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(A, Z), step: Coord(A, X), dest: Coord(E, C) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.7477483404911075;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #50
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(TamMoveStepsDuringFormer { src: Coord(E, M), step: Coord(I, P), first_dest: Coord(U, P), second_dest: Coord(O, M) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.031648884467650684;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #51
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveSrcDst { src: Coord(U, X), dest: Coord(U, P) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.8350877891886156;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #52
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveSrcDst { src: Coord(AI, Z), dest: Coord(U, Z) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.8545633463728324;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #53
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(U, P), step: Coord(O, M), planned_direction: Coord(Y, M) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::InfAfterStep(m) => {
        let chooser = 0.26283687816011636;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: Some(Coord(AI, M)) };
        let chooser2 = 0.8287308693124804;
        let hnr_state = apply_after_half_acceptance(&ext_state, aha_move, config)
            .unwrap()
            .choose_by_uniform_random_variable(chooser2)
            .0;
        
        hnr_state
    },
    _ => { panic!("Expected PureMove::InfAfterStep") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #54
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveFromHopZuo { color: Kok1, prof: Kauk2, dest: Coord(U, X) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.8043466695914161;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #55
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(E, C), step: Coord(I, M), dest: Coord(E, M) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.5195139436063345;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #56
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(TamMoveStepsDuringLatter { src: Coord(O, M), step: Coord(AI, P), first_dest: Coord(Y, M), second_dest: Coord(Y, M) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.5335569878518538;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #57
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(Y, C), step: Coord(Y, M), dest: Coord(Y, C) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.4763072965320412;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #58
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(TamMoveStepsDuringFormer { src: Coord(Y, M), step: Coord(Y, C), first_dest: Coord(O, M), second_dest: Coord(O, C) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.2523511710605809;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #59
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveSrcDst { src: Coord(AI, M), dest: Coord(Y, M) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.24340297410534928;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #60
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(Y, N), step: Coord(Y, L), planned_direction: Coord(Y, C) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::InfAfterStep(m) => {
        let chooser = 0.7427900938091212;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: Some(Coord(Y, T)) };
        let chooser2 = 0.7269166758365914;
        let hnr_state = apply_after_half_acceptance(&ext_state, aha_move, config)
            .unwrap()
            .choose_by_uniform_random_variable(chooser2)
            .0;
        
        hnr_state
    },
    _ => { panic!("Expected PureMove::InfAfterStep") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #61
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(Y, M), step: Coord(Y, C), planned_direction: Coord(Y, T) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::InfAfterStep(m) => {
        let chooser = 0.26183493552460746;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: Some(Coord(Y, X)) };
        let chooser2 = 0.6334004986194741;
        let hnr_state = apply_after_half_acceptance(&ext_state, aha_move, config)
            .unwrap()
            .choose_by_uniform_random_variable(chooser2)
            .0;
        
        hnr_state
    },
    _ => { panic!("Expected PureMove::InfAfterStep") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #62
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(TamMoveStepsDuringLatter { src: Coord(O, C), step: Coord(I, M), first_dest: Coord(U, C), second_dest: Coord(U, C) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.9085542662553809;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #63
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(U, T), step: Coord(I, T), dest: Coord(U, T) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.7012849708189766;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #64
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(IA, P), step: Coord(AI, P), planned_direction: Coord(Y, P) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::InfAfterStep(m) => {
        let chooser = 0.22614265379005793;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: Some(Coord(Y, P)) };
        let chooser2 = 0.960547924527994;
        let hnr_state = apply_after_half_acceptance(&ext_state, aha_move, config)
            .unwrap()
            .choose_by_uniform_random_variable(chooser2)
            .0;
        
        hnr_state
    },
    _ => { panic!("Expected PureMove::InfAfterStep") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #65
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveSrcDst { src: Coord(U, M), dest: Coord(O, C) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.012686320490403413;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #66
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(TamMoveStepsDuringLatter { src: Coord(U, C), step: Coord(U, M), first_dest: Coord(O, C), second_dest: Coord(O, C) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.9707584942695419;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #67
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(Y, X), step: Coord(U, X), planned_direction: Coord(I, C) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::InfAfterStep(m) => {
        let chooser = 0.5336865986979084;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: Some(Coord(I, C)) };
        let chooser2 = 0.8784972709620238;
        let hnr_state = apply_after_half_acceptance(&ext_state, aha_move, config)
            .unwrap()
            .choose_by_uniform_random_variable(chooser2)
            .0;
        
        hnr_state
    },
    _ => { panic!("Expected PureMove::InfAfterStep") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #68
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(TamMoveStepsDuringFormer { src: Coord(O, C), step: Coord(O, X), first_dest: Coord(U, C), second_dest: Coord(I, X) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.8557500448852877;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #69
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(I, C), step: Coord(I, X), planned_direction: Coord(E, C) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::InfAfterStep(m) => {
        let chooser = 0.30207624069532113;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: Some(Coord(E, C)) };
        let chooser2 = 0.16020331668236154;
        let hnr_state = apply_after_half_acceptance(&ext_state, aha_move, config)
            .unwrap()
            .choose_by_uniform_random_variable(chooser2)
            .0;
        
        hnr_state
    },
    _ => { panic!("Expected PureMove::InfAfterStep") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #70
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(TamMoveStepsDuringFormer { src: Coord(I, X), step: Coord(U, Z), first_dest: Coord(O, T), second_dest: Coord(Y, Z) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.40373821390851694;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #71
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(E, K), step: Coord(I, K), dest: Coord(U, K) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.15525347126444067;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #72
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveSrcDst { src: Coord(Y, T), dest: Coord(E, M) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.5039459565460976;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::HandExists { if_tymok, if_taxot } => {
        


        match if_taxot {
            IfTaxot::NextSeason(ps) => {
                let chooser = 0.9230106550687184;
                state = ps.clone().choose_by_uniform_random_variable(chooser).0
            },
            IfTaxot::VictoriousSide(v) => {
                panic!("Expected IfTaxot::VictoriousSide")
            }
        }
    },
    _ => { panic!("Expected HandResolved::HandExists") }
}
                                
// Turn #73
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(A, N), step: Coord(A, K), dest: Coord(A, N) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.9280384661841047;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #74
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(AI, Z), step: Coord(O, Z), planned_direction: Coord(I, Z) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::InfAfterStep(m) => {
        let chooser = 0.3928546784611131;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: None };
        let chooser2 = 0.9161492214777827;
        let hnr_state = apply_after_half_acceptance(&ext_state, aha_move, config)
            .unwrap()
            .choose_by_uniform_random_variable(chooser2)
            .0;
        
        hnr_state
    },
    _ => { panic!("Expected PureMove::InfAfterStep") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #75
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(TamMoveStepsDuringLatter { src: Coord(O, Z), step: Coord(AI, Z), first_dest: Coord(Y, Z), second_dest: Coord(Y, Z) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.32611093588076834;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #76
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(IA, X), step: Coord(AU, X), dest: Coord(AU, Z) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.8579680043646005;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #77
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(E, L), step: Coord(E, K), planned_direction: Coord(E, N) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::InfAfterStep(m) => {
        let chooser = 0.4112020584360655;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: Some(Coord(E, N)) };
        let chooser2 = 0.05676952488081621;
        let hnr_state = apply_after_half_acceptance(&ext_state, aha_move, config)
            .unwrap()
            .choose_by_uniform_random_variable(chooser2)
            .0;
        
        hnr_state
    },
    _ => { panic!("Expected PureMove::InfAfterStep") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #78
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(TamMoveStepsDuringFormer { src: Coord(Y, Z), step: Coord(AI, T), first_dest: Coord(Y, T), second_dest: Coord(O, Z) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.29545806368516636;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #79
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(I, Z), step: Coord(O, Z), dest: Coord(O, C) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.1441557904989168;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #80
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(TamMoveStepsDuringLatter { src: Coord(O, Z), step: Coord(O, C), first_dest: Coord(O, X), second_dest: Coord(U, X) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.3718234228115651;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #81
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(E, X), step: Coord(I, C), planned_direction: Coord(O, P) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::InfAfterStep(m) => {
        let chooser = 0.2783391395299599;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: Some(Coord(U, M)) };
        let chooser2 = 0.5931583961805534;
        let hnr_state = apply_after_half_acceptance(&ext_state, aha_move, config)
            .unwrap()
            .choose_by_uniform_random_variable(chooser2)
            .0;
        
        hnr_state
    },
    _ => { panic!("Expected PureMove::InfAfterStep") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #82
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(TamMoveStepsDuringLatter { src: Coord(U, X), step: Coord(I, T), first_dest: Coord(I, Z), second_dest: Coord(U, Z) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.8387600563918386;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #83
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(O, C), step: Coord(AI, C), planned_direction: Coord(AU, C) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::InfAfterStep(m) => {
        let chooser = 0.7935966658038544;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: None };
        let chooser2 = 0.344972617184907;
        let hnr_state = apply_after_half_acceptance(&ext_state, aha_move, config)
            .unwrap()
            .choose_by_uniform_random_variable(chooser2)
            .0;
        
        hnr_state
    },
    _ => { panic!("Expected PureMove::InfAfterStep") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #84
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(TamMoveStepsDuringFormer { src: Coord(U, Z), step: Coord(I, T), first_dest: Coord(U, N), second_dest: Coord(U, T) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.5333478723682313;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #85
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(E, P), step: Coord(A, P), dest: Coord(E, P) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.1777301899073247;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #86
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(TamMoveStepsDuringFormer { src: Coord(U, T), step: Coord(I, T), first_dest: Coord(U, T), second_dest: Coord(U, N) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.664632643104783;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #87
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(E, K), step: Coord(A, K), dest: Coord(E, K) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.944920006756656;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #88
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(IA, Z), step: Coord(AU, X), dest: Coord(IA, X) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.9892501629789234;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #89
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(TamMoveStepsDuringFormer { src: Coord(U, N), step: Coord(I, N), first_dest: Coord(U, T), second_dest: Coord(I, Z) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.9699781229016736;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #90
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(AU, M), step: Coord(AI, M), planned_direction: Coord(Y, M) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::InfAfterStep(m) => {
        let chooser = 0.41253312228888683;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: Some(Coord(Y, M)) };
        let chooser2 = 0.8009421830675009;
        let hnr_state = apply_after_half_acceptance(&ext_state, aha_move, config)
            .unwrap()
            .choose_by_uniform_random_variable(chooser2)
            .0;
        
        hnr_state
    },
    _ => { panic!("Expected PureMove::InfAfterStep") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #91
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(TamMoveStepsDuringLatter { src: Coord(I, Z), step: Coord(I, T), first_dest: Coord(U, T), second_dest: Coord(U, Z) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.9961216883280364;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #92
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(AU, T), step: Coord(AI, N), planned_direction: Coord(AU, T) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::InfAfterStep(m) => {
        let chooser = 0.38302849871152933;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: Some(Coord(IA, Z)) };
        let chooser2 = 0.034390683194898464;
        let hnr_state = apply_after_half_acceptance(&ext_state, aha_move, config)
            .unwrap()
            .choose_by_uniform_random_variable(chooser2)
            .0;
        
        hnr_state
    },
    _ => { panic!("Expected PureMove::InfAfterStep") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #93
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveSrcDst { src: Coord(I, L), dest: Coord(U, L) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.6924113479484421;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #94
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(TamMoveStepsDuringFormer { src: Coord(U, Z), step: Coord(I, X), first_dest: Coord(U, C), second_dest: Coord(O, X) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.5539803513881495;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #95
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(A, T), step: Coord(A, Z), dest: Coord(E, X) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.8436287533454797;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #96
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(AU, X), step: Coord(IA, C), dest: Coord(AU, X) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.6236547343728241;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #97
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(TamMoveStepsDuringLatter { src: Coord(O, X), step: Coord(I, X), first_dest: Coord(U, X), second_dest: Coord(U, C) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.44080835413585717;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #98
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(AU, K), step: Coord(AI, K), dest: Coord(Y, K) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.4970365929949766;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #99
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(TamMoveNoStep { src: Coord(U, C), first_dest: Coord(O, M), second_dest: Coord(Y, P) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.5336200711837228;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #100
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(AU, Z), step: Coord(AI, T), dest: Coord(AU, N) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.4583690705073217;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #101
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(O, C), step: Coord(AI, C), planned_direction: Coord(Y, C) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::InfAfterStep(m) => {
        let chooser = 0.6908970335031037;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: None };
        let chooser2 = 0.9484421424639177;
        let hnr_state = apply_after_half_acceptance(&ext_state, aha_move, config)
            .unwrap()
            .choose_by_uniform_random_variable(chooser2)
            .0;
        
        hnr_state
    },
    _ => { panic!("Expected PureMove::InfAfterStep") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #102
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(TamMoveNoStep { src: Coord(Y, P), first_dest: Coord(O, P), second_dest: Coord(U, P) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.3772839096485434;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #103
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveSrcDst { src: Coord(E, X), dest: Coord(E, C) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.7749129053320326;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #104
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveSrcDst { src: Coord(Y, M), dest: Coord(Y, T) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.034578048060386446;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #105
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(E, N), step: Coord(A, N), planned_direction: Coord(A, T) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::InfAfterStep(m) => {
        let chooser = 0.2938263975529306;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: Some(Coord(A, T)) };
        let chooser2 = 0.7285889083965394;
        let hnr_state = apply_after_half_acceptance(&ext_state, aha_move, config)
            .unwrap()
            .choose_by_uniform_random_variable(chooser2)
            .0;
        
        hnr_state
    },
    _ => { panic!("Expected PureMove::InfAfterStep") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #106
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(TamMoveStepsDuringFormer { src: Coord(U, P), step: Coord(I, P), first_dest: Coord(U, P), second_dest: Coord(O, M) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.2632993723429593;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #107
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(A, Z), step: Coord(A, T), dest: Coord(A, Z) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.1994374241002136;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #108
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(AU, L), step: Coord(AI, L), planned_direction: Coord(U, L) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::InfAfterStep(m) => {
        let chooser = 0.7895332952965667;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: None };
        let chooser2 = 0.72167586688962;
        let hnr_state = apply_after_half_acceptance(&ext_state, aha_move, config)
            .unwrap()
            .choose_by_uniform_random_variable(chooser2)
            .0;
        
        hnr_state
    },
    _ => { panic!("Expected PureMove::InfAfterStep") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #109
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(TamMoveStepsDuringLatter { src: Coord(O, M), step: Coord(O, C), first_dest: Coord(U, C), second_dest: Coord(U, C) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.25837736624680907;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #110
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(AU, L), step: Coord(AU, N), planned_direction: Coord(AU, L) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::InfAfterStep(m) => {
        let chooser = 0.9159784613549228;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: Some(Coord(AU, L)) };
        let chooser2 = 0.24581098199864015;
        let hnr_state = apply_after_half_acceptance(&ext_state, aha_move, config)
            .unwrap()
            .choose_by_uniform_random_variable(chooser2)
            .0;
        
        hnr_state
    },
    _ => { panic!("Expected PureMove::InfAfterStep") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #111
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(U, M), step: Coord(O, C), planned_direction: Coord(Y, X) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::InfAfterStep(m) => {
        let chooser = 0.944425714297001;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: Some(Coord(AI, Z)) };
        let chooser2 = 0.3839647347384515;
        let hnr_state = apply_after_half_acceptance(&ext_state, aha_move, config)
            .unwrap()
            .choose_by_uniform_random_variable(chooser2)
            .0;
        
        hnr_state
    },
    _ => { panic!("Expected PureMove::InfAfterStep") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #112
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(IA, T), step: Coord(AU, N), dest: Coord(AU, T) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.6078804271454195;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #113
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(I, C), step: Coord(I, X), dest: Coord(O, X) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.8662325184679526;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #114
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(TamMoveStepsDuringFormer { src: Coord(U, C), step: Coord(O, C), first_dest: Coord(Y, C), second_dest: Coord(Y, X) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.760190460380066;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #115
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(A, K), step: Coord(E, K), dest: Coord(E, L) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.7018569497343954;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #116
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(AU, N), step: Coord(AU, T), dest: Coord(AU, Z) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.922143678040832;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #117
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(TamMoveStepsDuringLatter { src: Coord(Y, X), step: Coord(AI, X), first_dest: Coord(Y, C), second_dest: Coord(AU, C) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.09252224877119308;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #118
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(AI, C), step: Coord(AU, C), dest: Coord(Y, C) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.8633721742539288;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #119
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(E, K), step: Coord(I, K), planned_direction: Coord(I, L) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::InfAfterStep(m) => {
        let chooser = 0.581487867497069;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: Some(Coord(I, L)) };
        let chooser2 = 0.6453429244332499;
        let hnr_state = apply_after_half_acceptance(&ext_state, aha_move, config)
            .unwrap()
            .choose_by_uniform_random_variable(chooser2)
            .0;
        
        hnr_state
    },
    _ => { panic!("Expected PureMove::InfAfterStep") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #120
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(Y, K), step: Coord(Y, T), planned_direction: Coord(O, Z) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::InfAfterStep(m) => {
        let chooser = 0.40244483717534374;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: Some(Coord(O, Z)) };
        let chooser2 = 0.5068264875447892;
        let hnr_state = apply_after_half_acceptance(&ext_state, aha_move, config)
            .unwrap()
            .choose_by_uniform_random_variable(chooser2)
            .0;
        
        hnr_state
    },
    _ => { panic!("Expected PureMove::InfAfterStep") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #121
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(TamMoveStepsDuringLatter { src: Coord(AU, C), step: Coord(AI, P), first_dest: Coord(AU, M), second_dest: Coord(AU, M) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.43612788557181226;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #122
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(AI, M), step: Coord(AU, M), dest: Coord(AU, C) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.006654370179211999;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #123
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(A, M), step: Coord(I, X), dest: Coord(O, M) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.6454686456300478;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #124
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(AI, T), step: Coord(Y, T), dest: Coord(AI, T) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.5789155242332221;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #125
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(O, C), step: Coord(Y, C), planned_direction: Coord(AU, C) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::InfAfterStep(m) => {
        let chooser = 0.37144942879204756;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: Some(Coord(AU, C)) };
        let chooser2 = 0.16813179882743035;
        let hnr_state = apply_after_half_acceptance(&ext_state, aha_move, config)
            .unwrap()
            .choose_by_uniform_random_variable(chooser2)
            .0;
        
        hnr_state
    },
    _ => { panic!("Expected PureMove::InfAfterStep") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #126
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(O, Z), step: Coord(AI, N), planned_direction: Coord(I, N) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::InfAfterStep(m) => {
        let chooser = 0.8157869838963095;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: Some(Coord(I, N)) };
        let chooser2 = 0.007747626587052303;
        let hnr_state = apply_after_half_acceptance(&ext_state, aha_move, config)
            .unwrap()
            .choose_by_uniform_random_variable(chooser2)
            .0;
        
        hnr_state
    },
    _ => { panic!("Expected PureMove::InfAfterStep") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #127
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveFromHopZuo { color: Kok1, prof: Kauk2, dest: Coord(U, T) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.7201097506113889;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #128
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveFromHopZuo { color: Huok2, prof: Kauk2, dest: Coord(E, X) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.6450961153068044;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #129
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(TamMoveStepsDuringFormer { src: Coord(AU, M), step: Coord(AU, C), first_dest: Coord(AU, M), second_dest: Coord(AI, M) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.3994071219653066;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #130
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(I, N), step: Coord(U, T), planned_direction: Coord(O, Z) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::InfAfterStep(m) => {
        let chooser = 0.21440599093081958;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: None };
        let chooser2 = 0.9856743489316238;
        let hnr_state = apply_after_half_acceptance(&ext_state, aha_move, config)
            .unwrap()
            .choose_by_uniform_random_variable(chooser2)
            .0;
        
        hnr_state
    },
    _ => { panic!("Expected PureMove::InfAfterStep") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #131
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(U, T), step: Coord(Y, T), dest: Coord(AI, T) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.9442280687905127;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #132
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveSrcDst { src: Coord(IA, C), dest: Coord(AI, C) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.9456344147520702;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #133
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveFromHopZuo { color: Kok1, prof: Kauk2, dest: Coord(IA, T) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.3170721991011781;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #134
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveSrcDst { src: Coord(IA, K), dest: Coord(AU, K) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.7509996411838177;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #135
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(A, N), step: Coord(I, N), dest: Coord(A, K) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.3965935043309643;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #136
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(AU, P), step: Coord(Y, C), planned_direction: Coord(Y, P) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::InfAfterStep(m) => {
        let chooser = 0.9923232839430538;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: None };
        let chooser2 = 0.6301316777933987;
        let hnr_state = apply_after_half_acceptance(&ext_state, aha_move, config)
            .unwrap()
            .choose_by_uniform_random_variable(chooser2)
            .0;
        
        hnr_state
    },
    _ => { panic!("Expected PureMove::InfAfterStep") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #137
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(A, Z), step: Coord(A, T), dest: Coord(E, Z) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.11364365889129135;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #138
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(IA, M), step: Coord(AI, X), dest: Coord(O, M) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.1951143347960469;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #139
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(A, T), step: Coord(A, L), planned_direction: Coord(A, Z) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::InfAfterStep(m) => {
        let chooser = 0.669883960545463;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: Some(Coord(A, T)) };
        let chooser2 = 0.9368057263796896;
        let hnr_state = apply_after_half_acceptance(&ext_state, aha_move, config)
            .unwrap()
            .choose_by_uniform_random_variable(chooser2)
            .0;
        
        hnr_state
    },
    _ => { panic!("Expected PureMove::InfAfterStep") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #140
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveFromHopZuo { color: Kok1, prof: Maun1, dest: Coord(U, K) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.47975245819348145;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #141
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(I, L), step: Coord(I, N), planned_direction: Coord(Y, N) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::InfAfterStep(m) => {
        let chooser = 0.8695868996279094;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: Some(Coord(AI, N)) };
        let chooser2 = 0.10774173289762456;
        let hnr_state = apply_after_half_acceptance(&ext_state, aha_move, config)
            .unwrap()
            .choose_by_uniform_random_variable(chooser2)
            .0;
        
        hnr_state
    },
    _ => { panic!("Expected PureMove::InfAfterStep") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #142
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveSrcDst { src: Coord(AU, P), dest: Coord(Y, P) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.2666094018336922;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #143
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveFromHopZuo { color: Huok2, prof: Kauk2, dest: Coord(AU, N) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.7654188539800335;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #144
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(Y, P), step: Coord(I, P), dest: Coord(U, P) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.2773343611404847;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #145
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(E, Z), step: Coord(A, X), dest: Coord(E, Z) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.5359124908555193;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #146
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(Y, T), step: Coord(E, M), planned_direction: Coord(E, C) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::InfAfterStep(m) => {
        let chooser = 0.20944132556033412;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: Some(Coord(E, C)) };
        let chooser2 = 0.004981444459672857;
        let hnr_state = apply_after_half_acceptance(&ext_state, aha_move, config)
            .unwrap()
            .choose_by_uniform_random_variable(chooser2)
            .0;
        
        hnr_state
    },
    _ => { panic!("Expected PureMove::InfAfterStep") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #147
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(TamMoveStepsDuringFormer { src: Coord(AI, M), step: Coord(AI, C), first_dest: Coord(Y, X), second_dest: Coord(O, C) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.27151087224758064;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #148
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveFromHopZuo { color: Huok2, prof: Uai1, dest: Coord(AI, M) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.4645558577225488;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #149
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(U, M), step: Coord(E, X), dest: Coord(A, Z) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.9431148786369318;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #150
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(AU, Z), step: Coord(AU, X), dest: Coord(AU, Z) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.4599816637728379;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #151
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveSrcDst { src: Coord(E, M), dest: Coord(A, M) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.6057434804025413;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #152
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(TamMoveStepsDuringLatter { src: Coord(O, C), step: Coord(Y, C), first_dest: Coord(Y, X), second_dest: Coord(Y, X) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.6920296555576511;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #153
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(AI, N), step: Coord(AI, Z), planned_direction: Coord(IA, C) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::InfAfterStep(m) => {
        let chooser = 0.9521769690838944;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: None };
        let chooser2 = 0.6697527295080766;
        let hnr_state = apply_after_half_acceptance(&ext_state, aha_move, config)
            .unwrap()
            .choose_by_uniform_random_variable(chooser2)
            .0;
        
        hnr_state
    },
    _ => { panic!("Expected PureMove::InfAfterStep") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #154
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(I, N), step: Coord(I, T), dest: Coord(U, T) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.5757856097674503;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #155
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(A, K), step: Coord(I, K), dest: Coord(O, K) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.5678595872391231;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #156
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(U, T), step: Coord(E, L), planned_direction: Coord(E, K) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::InfAfterStep(m) => {
        let chooser = 0.5374828215539533;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: None };
        let chooser2 = 0.7059408310458792;
        let hnr_state = apply_after_half_acceptance(&ext_state, aha_move, config)
            .unwrap()
            .choose_by_uniform_random_variable(chooser2)
            .0;
        
        hnr_state
    },
    _ => { panic!("Expected PureMove::InfAfterStep") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #157
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(TamMoveStepsDuringFormer { src: Coord(Y, X), step: Coord(O, X), first_dest: Coord(U, X), second_dest: Coord(U, C) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.4344301661641885;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #158
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(IA, N), step: Coord(AI, N), dest: Coord(O, Z) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.10947704208714981;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #159
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(AI, N), step: Coord(AU, T), planned_direction: Coord(AU, Z) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::InfAfterStep(m) => {
        let chooser = 0.2889117189466067;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: Some(Coord(AU, Z)) };
        let chooser2 = 0.8452749415450115;
        let hnr_state = apply_after_half_acceptance(&ext_state, aha_move, config)
            .unwrap()
            .choose_by_uniform_random_variable(chooser2)
            .0;
        
        hnr_state
    },
    _ => { panic!("Expected PureMove::InfAfterStep") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #160
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveSrcDst { src: Coord(U, K), dest: Coord(Y, N) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.8733941110889977;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #161
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveFromHopZuo { color: Huok2, prof: Uai1, dest: Coord(O, T) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.555517412393848;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #162
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveSrcDst { src: Coord(AI, K), dest: Coord(Y, K) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.3194522909264602;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #163
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(TamMoveStepsDuringFormer { src: Coord(U, C), step: Coord(O, M), first_dest: Coord(O, P), second_dest: Coord(Y, M) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.5369773417596867;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #164
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(U, P), step: Coord(U, T), planned_direction: Coord(I, Z) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::InfAfterStep(m) => {
        let chooser = 0.6484768254299386;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: Some(Coord(I, Z)) };
        let chooser2 = 0.11627968033078939;
        let hnr_state = apply_after_half_acceptance(&ext_state, aha_move, config)
            .unwrap()
            .choose_by_uniform_random_variable(chooser2)
            .0;
        
        hnr_state
    },
    _ => { panic!("Expected PureMove::InfAfterStep") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #165
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveSrcDst { src: Coord(A, T), dest: Coord(A, N) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.12499128718556674;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #166
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(TamMoveStepsDuringFormer { src: Coord(Y, M), step: Coord(Y, C), first_dest: Coord(O, C), second_dest: Coord(Y, M) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.36531312671117966;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #167
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(E, P), step: Coord(E, C), planned_direction: Coord(E, M) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::InfAfterStep(m) => {
        let chooser = 0.6889059869446045;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: Some(Coord(E, M)) };
        let chooser2 = 0.23767882153354636;
        let hnr_state = apply_after_half_acceptance(&ext_state, aha_move, config)
            .unwrap()
            .choose_by_uniform_random_variable(chooser2)
            .0;
        
        hnr_state
    },
    _ => { panic!("Expected PureMove::InfAfterStep") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #168
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(AI, M), step: Coord(AI, C), dest: Coord(AU, M) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.22371004014165652;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #169
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(E, Z), step: Coord(I, X), dest: Coord(U, Z) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.1425352967854312;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #170
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(TamMoveStepsDuringFormer { src: Coord(Y, M), step: Coord(O, M), first_dest: Coord(Y, P), second_dest: Coord(O, P) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.1651683660152844;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #171
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(O, T), step: Coord(Y, N), dest: Coord(Y, L) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.4003726658637977;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #172
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(U, T), step: Coord(U, P), planned_direction: Coord(I, P) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::InfAfterStep(m) => {
        let chooser = 0.014533982679234447;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: None };
        let chooser2 = 0.3792632024857704;
        let hnr_state = apply_after_half_acceptance(&ext_state, aha_move, config)
            .unwrap()
            .choose_by_uniform_random_variable(chooser2)
            .0;
        
        hnr_state
    },
    _ => { panic!("Expected PureMove::InfAfterStep") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #173
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveSrcDst { src: Coord(E, Z), dest: Coord(E, X) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.42549301784800686;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #174
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(TamMoveStepsDuringFormer { src: Coord(O, P), step: Coord(O, M), first_dest: Coord(O, C), second_dest: Coord(U, X) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.6246036928855168;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #175
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveFromHopZuo { color: Huok2, prof: Kauk2, dest: Coord(AI, M) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.13166521936829811;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #176
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(IA, Z), step: Coord(AU, T), dest: Coord(IA, Z) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.721965629607356;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #177
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(A, X), step: Coord(A, Z), dest: Coord(A, T) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.1912797319726638;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #178
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveSrcDst { src: Coord(AU, T), dest: Coord(AU, Z) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.18063927980652583;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #179
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(O, X), step: Coord(AI, X), dest: Coord(AU, X) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.4723298664618222;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #180
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveFromHopZuo { color: Kok1, prof: Tuk2, dest: Coord(U, C) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.45623661787726666;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #181
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveFromHopZuo { color: Kok1, prof: Dau2, dest: Coord(AI, N) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.023766541602878943;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #182
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(AU, L), step: Coord(IA, L), planned_direction: Coord(IA, K) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::InfAfterStep(m) => {
        let chooser = 0.8753521561848335;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: None };
        let chooser2 = 0.3785628869300506;
        let hnr_state = apply_after_half_acceptance(&ext_state, aha_move, config)
            .unwrap()
            .choose_by_uniform_random_variable(chooser2)
            .0;
        
        hnr_state
    },
    _ => { panic!("Expected PureMove::InfAfterStep") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #183
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(A, T), step: Coord(A, Z), dest: Coord(A, T) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.9380558418562345;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #184
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveSrcDst { src: Coord(U, T), dest: Coord(U, K) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.7393336136648244;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #185
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(A, Z), step: Coord(E, X), dest: Coord(A, Z) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.5206300596379686;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #186
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(TamMoveStepsDuringLatter { src: Coord(U, X), step: Coord(Y, C), first_dest: Coord(O, X), second_dest: Coord(O, C) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.30798247206966267;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #187
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(E, M), step: Coord(E, C), planned_direction: Coord(E, M) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::InfAfterStep(m) => {
        let chooser = 0.4998635497911562;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: Some(Coord(E, M)) };
        let chooser2 = 0.19350222595864386;
        let hnr_state = apply_after_half_acceptance(&ext_state, aha_move, config)
            .unwrap()
            .choose_by_uniform_random_variable(chooser2)
            .0;
        
        hnr_state
    },
    _ => { panic!("Expected PureMove::InfAfterStep") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #188
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(TamMoveStepsDuringLatter { src: Coord(O, C), step: Coord(U, C), first_dest: Coord(U, M), second_dest: Coord(U, X) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.36751674285142666;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #189
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveSrcDst { src: Coord(Y, L), dest: Coord(AI, L) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.07789920583127119;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #190
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(U, C), step: Coord(AI, C), planned_direction: Coord(Y, X) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::InfAfterStep(m) => {
        let chooser = 0.908237196328597;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: None };
        let chooser2 = 0.435077500937499;
        let hnr_state = apply_after_half_acceptance(&ext_state, aha_move, config)
            .unwrap()
            .choose_by_uniform_random_variable(chooser2)
            .0;
        
        hnr_state
    },
    _ => { panic!("Expected PureMove::InfAfterStep") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #191
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveFromHopZuo { color: Kok1, prof: Kauk2, dest: Coord(O, P) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.05478539539532368;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #192
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(E, C), step: Coord(U, C), planned_direction: Coord(I, X) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::InfAfterStep(m) => {
        let chooser = 0.09869343215169091;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: Some(Coord(I, X)) };
        let chooser2 = 0.8106027079953563;
        let hnr_state = apply_after_half_acceptance(&ext_state, aha_move, config)
            .unwrap()
            .choose_by_uniform_random_variable(chooser2)
            .0;
        
        hnr_state
    },
    _ => { panic!("Expected PureMove::InfAfterStep") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #193
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(AI, L), step: Coord(AU, L), dest: Coord(IA, N) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.839793754649332;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #194
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveFromHopZuo { color: Kok1, prof: Kauk2, dest: Coord(AU, P) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.5968694200404733;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #195
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveFromHopZuo { color: Kok1, prof: Kaun1, dest: Coord(O, X) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.7185098896048304;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #196
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(TamMoveStepsDuringLatter { src: Coord(U, X), step: Coord(O, M), first_dest: Coord(O, C), second_dest: Coord(U, M) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.8745788930324995;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #197
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(O, P), step: Coord(U, P), dest: Coord(Y, P) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.7873830944842272;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #198
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(U, P), step: Coord(Y, P), dest: Coord(O, P) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.5591028730530712;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #199
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(TamMoveStepsDuringLatter { src: Coord(U, M), step: Coord(I, M), first_dest: Coord(I, C), second_dest: Coord(I, C) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.4718362564574129;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #200
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(U, C), step: Coord(AI, C), planned_direction: Coord(AI, M) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::InfAfterStep(m) => {
        let chooser = 0.8673916997301309;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: None };
        let chooser2 = 0.08829820405028266;
        let hnr_state = apply_after_half_acceptance(&ext_state, aha_move, config)
            .unwrap()
            .choose_by_uniform_random_variable(chooser2)
            .0;
        
        hnr_state
    },
    _ => { panic!("Expected PureMove::InfAfterStep") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #201
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(TamMoveStepsDuringFormer { src: Coord(I, C), step: Coord(I, X), first_dest: Coord(I, C), second_dest: Coord(E, C) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.3825268001069275;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #202
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(I, X), step: Coord(E, C), planned_direction: Coord(A, X) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::InfAfterStep(m) => {
        let chooser = 0.9825797627091729;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: None };
        let chooser2 = 0.8426913914640086;
        let hnr_state = apply_after_half_acceptance(&ext_state, aha_move, config)
            .unwrap()
            .choose_by_uniform_random_variable(chooser2)
            .0;
        
        hnr_state
    },
    _ => { panic!("Expected PureMove::InfAfterStep") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #203
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(A, Z), step: Coord(E, T), dest: Coord(I, Z) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.5915658461244313;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #204
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(TamMoveStepsDuringFormer { src: Coord(E, C), step: Coord(E, M), first_dest: Coord(E, C), second_dest: Coord(I, C) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.43335593181158294;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #205
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(IA, N), step: Coord(IA, L), dest: Coord(AU, K) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.25364062827294087;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #206
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(TamMoveStepsDuringFormer { src: Coord(I, C), step: Coord(U, C), first_dest: Coord(O, C), second_dest: Coord(U, X) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.6664494041717829;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #207
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveFromHopZuo { color: Kok1, prof: Kua2, dest: Coord(E, Z) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.3823400713477002;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #208
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveSrcDst { src: Coord(U, C), dest: Coord(AI, T) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.39077920702712476;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #209
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(A, P), step: Coord(I, P), planned_direction: Coord(O, P) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::InfAfterStep(m) => {
        let chooser = 0.09467647147807712;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: None };
        let chooser2 = 0.5726707489848188;
        let hnr_state = apply_after_half_acceptance(&ext_state, aha_move, config)
            .unwrap()
            .choose_by_uniform_random_variable(chooser2)
            .0;
        
        hnr_state
    },
    _ => { panic!("Expected PureMove::InfAfterStep") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #210
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveFromHopZuo { color: Kok1, prof: Kauk2, dest: Coord(I, C) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.6236423998115058;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #211
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(TamMoveStepsDuringFormer { src: Coord(U, X), step: Coord(O, X), first_dest: Coord(O, Z), second_dest: Coord(Y, X) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.4189991349141515;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #212
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(AU, M), step: Coord(AI, C), dest: Coord(AU, M) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.902696586046259;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #213
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(TamMoveStepsDuringFormer { src: Coord(Y, X), step: Coord(O, X), first_dest: Coord(Y, X), second_dest: Coord(O, C) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.23489912060024598;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #214
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(IA, L), step: Coord(AI, T), dest: Coord(O, X) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.3737624998678474;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #215
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(AI, N), step: Coord(I, C), planned_direction: Coord(O, Z) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::InfAfterStep(m) => {
        let chooser = 0.8394803727999218;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: Some(Coord(Y, T)) };
        let chooser2 = 0.603450649078539;
        let hnr_state = apply_after_half_acceptance(&ext_state, aha_move, config)
            .unwrap()
            .choose_by_uniform_random_variable(chooser2)
            .0;
        
        hnr_state
    },
    _ => { panic!("Expected PureMove::InfAfterStep") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #216
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(IA, L), step: Coord(AI, T), dest: Coord(O, X) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.9390598860291332;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #217
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveSrcDst { src: Coord(A, P), dest: Coord(E, P) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.5134885541356441;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #218
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveFromHopZuo { color: Kok1, prof: Kaun1, dest: Coord(E, C) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.4403386788526389;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #219
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveSrcDst { src: Coord(O, K), dest: Coord(AI, K) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.06498854058335979;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #220
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(O, P), step: Coord(O, M), planned_direction: Coord(U, P) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::InfAfterStep(m) => {
        let chooser = 0.4785970511812332;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: None };
        let chooser2 = 0.37822514644736926;
        let hnr_state = apply_after_half_acceptance(&ext_state, aha_move, config)
            .unwrap()
            .choose_by_uniform_random_variable(chooser2)
            .0;
        
        hnr_state
    },
    _ => { panic!("Expected PureMove::InfAfterStep") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #221
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(A, N), step: Coord(A, T), planned_direction: Coord(A, Z) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::InfAfterStep(m) => {
        let chooser = 0.08656622792962032;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: None };
        let chooser2 = 0.11153590844555605;
        let hnr_state = apply_after_half_acceptance(&ext_state, aha_move, config)
            .unwrap()
            .choose_by_uniform_random_variable(chooser2)
            .0;
        
        hnr_state
    },
    _ => { panic!("Expected PureMove::InfAfterStep") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #222
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(AI, T), step: Coord(Y, T), planned_direction: Coord(IA, K) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::InfAfterStep(m) => {
        let chooser = 0.4595866229169826;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: None };
        let chooser2 = 0.3986926302126913;
        let hnr_state = apply_after_half_acceptance(&ext_state, aha_move, config)
            .unwrap()
            .choose_by_uniform_random_variable(chooser2)
            .0;
        
        hnr_state
    },
    _ => { panic!("Expected PureMove::InfAfterStep") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #223
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(A, C), step: Coord(I, C), dest: Coord(O, Z) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.4575284554962371;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #224
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(TamMoveStepsDuringLatter { src: Coord(O, C), step: Coord(AI, M), first_dest: Coord(Y, M), second_dest: Coord(Y, M) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.006612582534225253;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #225
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(E, P), step: Coord(E, M), dest: Coord(E, P) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.9239067710489001;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #226
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveSrcDst { src: Coord(Y, C), dest: Coord(O, C) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.4270785105848036;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #227
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveSrcDst { src: Coord(AU, C), dest: Coord(IA, C) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.6188293800878482;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #228
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(IA, X), step: Coord(IA, C), dest: Coord(AU, X) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.3911898385440671;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #229
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(TamMoveStepsDuringFormer { src: Coord(Y, M), step: Coord(AI, C), first_dest: Coord(Y, X), second_dest: Coord(O, C) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.18605743366697391;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #230
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveFromHopZuo { color: Huok2, prof: Kauk2, dest: Coord(O, Z) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.01563134337852856;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #231
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(TamMoveStepsDuringLatter { src: Coord(O, C), step: Coord(I, C), first_dest: Coord(U, M), second_dest: Coord(U, C) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.8504851246979047;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #232
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveSrcDst { src: Coord(AU, M), dest: Coord(AU, C) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.5752523284590974;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #233
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveSrcDst { src: Coord(A, C), dest: Coord(I, C) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.9528448149423383;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #234
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(AU, Z), step: Coord(AI, Z), dest: Coord(AU, T) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.9559060553251596;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #235
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveFromHopZuo { color: Kok1, prof: Kauk2, dest: Coord(A, C) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.17428534243596416;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #236
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(AU, X), step: Coord(IA, Z), dest: Coord(AU, X) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.13318888077707336;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #237
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveSrcDst { src: Coord(Y, T), dest: Coord(AI, Z) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.2426499897772888;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #238
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(TamMoveStepsDuringFormer { src: Coord(U, C), step: Coord(I, X), first_dest: Coord(U, C), second_dest: Coord(O, C) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.7993911214670303;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #239
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(A, N), step: Coord(A, L), planned_direction: Coord(A, K) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::InfAfterStep(m) => {
        let chooser = 0.03616641761497774;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: None };
        let chooser2 = 0.03435978660590655;
        let hnr_state = apply_after_half_acceptance(&ext_state, aha_move, config)
            .unwrap()
            .choose_by_uniform_random_variable(chooser2)
            .0;
        
        hnr_state
    },
    _ => { panic!("Expected PureMove::InfAfterStep") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #240
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(TamMoveStepsDuringFormer { src: Coord(O, C), step: Coord(O, M), first_dest: Coord(O, C), second_dest: Coord(U, X) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.17778313799107104;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #241
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(E, X), step: Coord(E, C), dest: Coord(I, X) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.3428476784436131;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #242
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(TamMoveStepsDuringFormer { src: Coord(U, X), step: Coord(O, Z), first_dest: Coord(U, T), second_dest: Coord(O, N) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.4171141505427657;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #243
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveFromHopZuo { color: Kok1, prof: Gua2, dest: Coord(U, P) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.3901281527738645;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #244
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(AI, T), step: Coord(AI, Z), dest: Coord(Y, Z) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.8287549481153851;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #245
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(TamMoveStepsDuringFormer { src: Coord(O, N), step: Coord(Y, T), first_dest: Coord(O, T), second_dest: Coord(U, Z) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.6157092638690835;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #246
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(AU, L), step: Coord(U, L), planned_direction: Coord(I, L) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::InfAfterStep(m) => {
        let chooser = 0.5987189231078339;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: None };
        let chooser2 = 0.03609356752910675;
        let hnr_state = apply_after_half_acceptance(&ext_state, aha_move, config)
            .unwrap()
            .choose_by_uniform_random_variable(chooser2)
            .0;
        
        hnr_state
    },
    _ => { panic!("Expected PureMove::InfAfterStep") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #247
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(E, T), step: Coord(I, Z), planned_direction: Coord(U, X) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::InfAfterStep(m) => {
        let chooser = 0.1991264212809667;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: Some(Coord(O, C)) };
        let chooser2 = 0.8937746668525544;
        let hnr_state = apply_after_half_acceptance(&ext_state, aha_move, config)
            .unwrap()
            .choose_by_uniform_random_variable(chooser2)
            .0;
        
        hnr_state
    },
    _ => { panic!("Expected PureMove::InfAfterStep") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #248
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(TamMoveStepsDuringLatter { src: Coord(U, Z), step: Coord(I, C), first_dest: Coord(U, X), second_dest: Coord(U, M) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.4036433273250575;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #249
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveSrcDst { src: Coord(I, C), dest: Coord(O, P) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.8220398680827943;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #250
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(Y, Z), step: Coord(O, Z), planned_direction: Coord(AU, M) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::InfAfterStep(m) => {
        let chooser = 0.23489285703788776;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: Some(Coord(Y, X)) };
        let chooser2 = 0.08061641248384488;
        let hnr_state = apply_after_half_acceptance(&ext_state, aha_move, config)
            .unwrap()
            .choose_by_uniform_random_variable(chooser2)
            .0;
        
        hnr_state
    },
    _ => { panic!("Expected PureMove::InfAfterStep") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #251
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveFromHopZuo { color: Kok1, prof: Tuk2, dest: Coord(A, X) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.7069043689840638;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #252
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveSrcDst { src: Coord(AU, L), dest: Coord(IA, L) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.13982790077365492;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #253
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(A, X), step: Coord(A, T), planned_direction: Coord(A, Z) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::InfAfterStep(m) => {
        let chooser = 0.816174719329007;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: None };
        let chooser2 = 0.1333844115150261;
        let hnr_state = apply_after_half_acceptance(&ext_state, aha_move, config)
            .unwrap()
            .choose_by_uniform_random_variable(chooser2)
            .0;
        
        hnr_state
    },
    _ => { panic!("Expected PureMove::InfAfterStep") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #254
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(Y, X), step: Coord(O, Z), planned_direction: Coord(I, N) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::InfAfterStep(m) => {
        let chooser = 0.8764780038391541;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: Some(Coord(E, L)) };
        let chooser2 = 0.28412783450531887;
        let hnr_state = apply_after_half_acceptance(&ext_state, aha_move, config)
            .unwrap()
            .choose_by_uniform_random_variable(chooser2)
            .0;
        
        hnr_state
    },
    _ => { panic!("Expected PureMove::InfAfterStep") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #255
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(TamMoveStepsDuringLatter { src: Coord(U, M), step: Coord(E, C), first_dest: Coord(I, C), second_dest: Coord(E, X) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.7102200549846427;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #256
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveFromHopZuo { color: Huok2, prof: Kua2, dest: Coord(I, N) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.31105388892419183;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #257
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(TamMoveStepsDuringFormer { src: Coord(E, X), step: Coord(I, X), first_dest: Coord(I, C), second_dest: Coord(U, M) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.1357427767014857;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #258
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(IA, Z), step: Coord(AU, T), dest: Coord(AI, N) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.5991862359852898;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #259
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(TamMoveStepsDuringFormer { src: Coord(U, M), step: Coord(I, M), first_dest: Coord(U, M), second_dest: Coord(I, C) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.6208252620191114;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #260
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(I, N), step: Coord(A, N), planned_direction: Coord(E, N) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::InfAfterStep(m) => {
        let chooser = 0.27043654557325103;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: Some(Coord(I, N)) };
        let chooser2 = 0.5298940759343754;
        let hnr_state = apply_after_half_acceptance(&ext_state, aha_move, config)
            .unwrap()
            .choose_by_uniform_random_variable(chooser2)
            .0;
        
        hnr_state
    },
    _ => { panic!("Expected PureMove::InfAfterStep") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #261
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(E, M), step: Coord(I, C), planned_direction: Coord(Y, C) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::InfAfterStep(m) => {
        let chooser = 0.7493754335995254;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: Some(Coord(Y, C)) };
        let chooser2 = 0.947776640041884;
        let hnr_state = apply_after_half_acceptance(&ext_state, aha_move, config)
            .unwrap()
            .choose_by_uniform_random_variable(chooser2)
            .0;
        
        hnr_state
    },
    _ => { panic!("Expected PureMove::InfAfterStep") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #262
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(AI, C), step: Coord(O, Z), dest: Coord(AI, C) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.38532815647759155;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #263
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveFromHopZuo { color: Huok2, prof: Kauk2, dest: Coord(E, T) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.04776725164690454;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #264
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveSrcDst { src: Coord(I, N), dest: Coord(E, N) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.7319453283138153;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #265
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveSrcDst { src: Coord(A, T), dest: Coord(E, N) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.8292159323989633;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #266
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(AU, X), step: Coord(AI, Z), dest: Coord(Y, T) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.6014378779714156;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #267
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveFromHopZuo { color: Huok2, prof: Kua2, dest: Coord(O, L) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.07755109810349892;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #268
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveFromHopZuo { color: Kok1, prof: Dau2, dest: Coord(IA, X) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.9138909428220257;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #269
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(A, N), step: Coord(A, X), planned_direction: Coord(E, X) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::InfAfterStep(m) => {
        let chooser = 0.9835329545608155;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: None };
        let chooser2 = 0.8663267351888326;
        let hnr_state = apply_after_half_acceptance(&ext_state, aha_move, config)
            .unwrap()
            .choose_by_uniform_random_variable(chooser2)
            .0;
        
        hnr_state
    },
    _ => { panic!("Expected PureMove::InfAfterStep") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #270
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveSrcDst { src: Coord(Y, T), dest: Coord(O, T) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.09182065284246421;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #271
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(U, L), step: Coord(O, L), dest: Coord(Y, L) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.8186637451624627;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #272
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveSrcDst { src: Coord(E, L), dest: Coord(A, L) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.40302987888837405;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #273
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(I, M), step: Coord(O, M), dest: Coord(Y, M) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.8006191265322143;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #274
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveFromHopZuo { color: Huok2, prof: Maun1, dest: Coord(O, K) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.8023190884294557;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #275
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(TamMoveStepsDuringLatter { src: Coord(I, C), step: Coord(O, P), first_dest: Coord(U, M), second_dest: Coord(U, M) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.8844126739492957;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #276
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(A, L), step: Coord(A, N), dest: Coord(E, N) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.1032757965263742;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #277
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(TamMoveStepsDuringLatter { src: Coord(U, M), step: Coord(O, M), first_dest: Coord(U, C), second_dest: Coord(U, M) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.49083694742579087;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #278
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveFromHopZuo { color: Kok1, prof: Uai1, dest: Coord(A, K) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.414577328220732;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #279
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(O, C), step: Coord(U, M), planned_direction: Coord(A, Z) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::InfAfterStep(m) => {
        let chooser = 0.14120927163798902;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: None };
        let chooser2 = 0.2936301022539415;
        let hnr_state = apply_after_half_acceptance(&ext_state, aha_move, config)
            .unwrap()
            .choose_by_uniform_random_variable(chooser2)
            .0;
        
        hnr_state
    },
    _ => { panic!("Expected PureMove::InfAfterStep") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #280
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(E, N), step: Coord(E, T), dest: Coord(A, T) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.12671104972652925;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #281
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(TamMoveStepsDuringFormer { src: Coord(U, M), step: Coord(O, P), first_dest: Coord(U, M), second_dest: Coord(I, M) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.8269726901638165;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #282
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(E, C), step: Coord(U, P), dest: Coord(Y, C) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.7970800387703862;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #283
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(A, N), step: Coord(A, K), planned_direction: Coord(A, N) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::InfAfterStep(m) => {
        let chooser = 0.5774921612100269;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: Some(Coord(A, N)) };
        let chooser2 = 0.8144727022122945;
        let hnr_state = apply_after_half_acceptance(&ext_state, aha_move, config)
            .unwrap()
            .choose_by_uniform_random_variable(chooser2)
            .0;
        
        hnr_state
    },
    _ => { panic!("Expected PureMove::InfAfterStep") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #284
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveFromHopZuo { color: Huok2, prof: Tuk2, dest: Coord(A, Z) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.4061742552239749;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #285
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(TamMoveStepsDuringLatter { src: Coord(I, M), step: Coord(A, C), first_dest: Coord(E, M), second_dest: Coord(E, C) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.7050916457427359;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #286
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(AU, P), step: Coord(AI, P), dest: Coord(Y, P) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.2627021456002102;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #287
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveSrcDst { src: Coord(O, L), dest: Coord(A, L) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.6534126953693028;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #288
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveFromHopZuo { color: Kok1, prof: Kauk2, dest: Coord(AI, T) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.8870822046240971;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #289
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(I, X), step: Coord(E, C), dest: Coord(I, X) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.07036786321944644;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #290
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(AU, C), step: Coord(AI, C), dest: Coord(Y, M) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.7637108462247398;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #291
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(AU, K), step: Coord(IA, L), dest: Coord(IA, N) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.9788224875400882;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #292
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveFromHopZuo { color: Kok1, prof: Kauk2, dest: Coord(O, T) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.1204644119660746;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #293
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(TamMoveStepsDuringLatter { src: Coord(E, C), step: Coord(A, C), first_dest: Coord(E, X), second_dest: Coord(E, X) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.22267159284239357;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #294
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(Y, T), step: Coord(AI, T), dest: Coord(Y, T) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.15383470563827983;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #295
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(TamMoveNoStep { src: Coord(E, X), first_dest: Coord(E, C), second_dest: Coord(E, X) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.7803735437429674;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #296
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(A, Z), step: Coord(E, Z), planned_direction: Coord(E, M) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::InfAfterStep(m) => {
        let chooser = 0.907826268832788;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: None };
        let chooser2 = 0.8245057029891506;
        let hnr_state = apply_after_half_acceptance(&ext_state, aha_move, config)
            .unwrap()
            .choose_by_uniform_random_variable(chooser2)
            .0;
        
        hnr_state
    },
    _ => { panic!("Expected PureMove::InfAfterStep") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #297
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(A, X), step: Coord(E, Z), planned_direction: Coord(A, Z) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::InfAfterStep(m) => {
        let chooser = 0.07420983725555541;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: Some(Coord(A, Z)) };
        let chooser2 = 0.3482316671043576;
        let hnr_state = apply_after_half_acceptance(&ext_state, aha_move, config)
            .unwrap()
            .choose_by_uniform_random_variable(chooser2)
            .0;
        
        hnr_state
    },
    _ => { panic!("Expected PureMove::InfAfterStep") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #298
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveSrcDst { src: Coord(IA, P), dest: Coord(IA, M) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.7788691697741129;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #299
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveFromHopZuo { color: Huok2, prof: Tuk2, dest: Coord(U, C) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.44782705489770225;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #300
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveSrcDst { src: Coord(IA, M), dest: Coord(AI, M) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.827944750970704;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #301
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(TamMoveStepsDuringFormer { src: Coord(E, X), step: Coord(I, Z), first_dest: Coord(E, X), second_dest: Coord(I, C) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.5828339538263915;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #302
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveFromHopZuo { color: Huok2, prof: Kauk2, dest: Coord(AU, X) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.5840775745109754;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #303
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(TamMoveStepsDuringFormer { src: Coord(I, C), step: Coord(I, X), first_dest: Coord(E, C), second_dest: Coord(I, C) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.5470138804915843;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #304
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(U, K), step: Coord(U, C), planned_direction: Coord(U, M) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::InfAfterStep(m) => {
        let chooser = 0.827777715938577;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: Some(Coord(U, M)) };
        let chooser2 = 0.725678487365089;
        let hnr_state = apply_after_half_acceptance(&ext_state, aha_move, config)
            .unwrap()
            .choose_by_uniform_random_variable(chooser2)
            .0;
        
        hnr_state
    },
    _ => { panic!("Expected PureMove::InfAfterStep") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #305
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveSrcDst { src: Coord(U, C), dest: Coord(U, Z) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.7964251632119118;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #306
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(U, M), step: Coord(I, C), planned_direction: Coord(O, P) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::InfAfterStep(m) => {
        let chooser = 0.832486110462596;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: None };
        let chooser2 = 0.3904488600304261;
        let hnr_state = apply_after_half_acceptance(&ext_state, aha_move, config)
            .unwrap()
            .choose_by_uniform_random_variable(chooser2)
            .0;
        
        hnr_state
    },
    _ => { panic!("Expected PureMove::InfAfterStep") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #307
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(U, Z), step: Coord(U, M), planned_direction: Coord(U, L) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::InfAfterStep(m) => {
        let chooser = 0.3944502619942847;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: Some(Coord(U, C)) };
        let chooser2 = 0.34297309583117097;
        let hnr_state = apply_after_half_acceptance(&ext_state, aha_move, config)
            .unwrap()
            .choose_by_uniform_random_variable(chooser2)
            .0;
        
        hnr_state
    },
    _ => { panic!("Expected PureMove::InfAfterStep") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #308
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(AU, T), step: Coord(AI, T), dest: Coord(AU, N) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.7864395428915223;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #309
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(TamMoveStepsDuringLatter { src: Coord(I, C), step: Coord(I, X), first_dest: Coord(E, X), second_dest: Coord(U, X) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.7883638119553003;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #310
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveFromHopZuo { color: Huok2, prof: Kauk2, dest: Coord(E, N) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.7714354744676594;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #311
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(TamMoveStepsDuringLatter { src: Coord(U, X), step: Coord(O, T), first_dest: Coord(U, Z), second_dest: Coord(U, N) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.20978397299619334;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #312
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveSrcDst { src: Coord(O, M), dest: Coord(I, X) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.6592539315344549;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::HandExists { if_tymok, if_taxot } => {
        


        match if_taxot {
            IfTaxot::NextSeason(ps) => {
                let chooser = 0.21577806229560403;
                state = ps.clone().choose_by_uniform_random_variable(chooser).0
            },
            IfTaxot::VictoriousSide(v) => {
                panic!("Expected IfTaxot::VictoriousSide")
            }
        }
    },
    _ => { panic!("Expected HandResolved::HandExists") }
}
                                
// Turn #313
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(TamMoveNoStep { src: Coord(O, Z), first_dest: Coord(Y, Z), second_dest: Coord(Y, T) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.983839667362914;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #314
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(A, T), step: Coord(A, N), dest: Coord(A, T) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.17738915076865447;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #315
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveSrcDst { src: Coord(AI, N), dest: Coord(AU, N) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.24360927759852902;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #316
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(E, M), step: Coord(E, X), planned_direction: Coord(E, Z) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::InfAfterStep(m) => {
        let chooser = 0.7415441222091838;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: Some(Coord(E, Z)) };
        let chooser2 = 0.5133495023159517;
        let hnr_state = apply_after_half_acceptance(&ext_state, aha_move, config)
            .unwrap()
            .choose_by_uniform_random_variable(chooser2)
            .0;
        
        hnr_state
    },
    _ => { panic!("Expected PureMove::InfAfterStep") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #317
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(TamMoveStepsDuringFormer { src: Coord(Y, T), step: Coord(AI, T), first_dest: Coord(Y, T), second_dest: Coord(Y, N) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.8450454108383695;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #318
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveSrcDst { src: Coord(I, N), dest: Coord(E, N) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.48551835945921784;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #319
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(TamMoveStepsDuringFormer { src: Coord(Y, N), step: Coord(AI, T), first_dest: Coord(Y, Z), second_dest: Coord(O, T) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.3953316082715427;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #320
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(E, L), step: Coord(A, L), planned_direction: Coord(E, L) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::InfAfterStep(m) => {
        let chooser = 0.44348140317623386;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: None };
        let chooser2 = 0.3911188689210793;
        let hnr_state = apply_after_half_acceptance(&ext_state, aha_move, config)
            .unwrap()
            .choose_by_uniform_random_variable(chooser2)
            .0;
        
        hnr_state
    },
    _ => { panic!("Expected PureMove::InfAfterStep") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #321
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(AI, C), step: Coord(AI, M), dest: Coord(Y, M) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.43353697427524585;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #322
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(I, Z), step: Coord(AI, Z), planned_direction: Coord(AU, Z) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::InfAfterStep(m) => {
        let chooser = 0.5225337537960562;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: Some(Coord(AU, Z)) };
        let chooser2 = 0.2741633582977936;
        let hnr_state = apply_after_half_acceptance(&ext_state, aha_move, config)
            .unwrap()
            .choose_by_uniform_random_variable(chooser2)
            .0;
        
        hnr_state
    },
    _ => { panic!("Expected PureMove::InfAfterStep") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #323
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(TamMoveNoStep { src: Coord(O, T), first_dest: Coord(Y, T), second_dest: Coord(AI, N) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.96413241870191;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #324
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(E, P), step: Coord(A, P), dest: Coord(E, P) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.9302467558056551;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #325
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(AU, T), step: Coord(AI, Z), dest: Coord(Y, T) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.9434195756847188;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #326
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(E, L), step: Coord(E, N), planned_direction: Coord(E, L) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::InfAfterStep(m) => {
        let chooser = 0.6012939410025299;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: None };
        let chooser2 = 0.8967597802384465;
        let hnr_state = apply_after_half_acceptance(&ext_state, aha_move, config)
            .unwrap()
            .choose_by_uniform_random_variable(chooser2)
            .0;
        
        hnr_state
    },
    _ => { panic!("Expected PureMove::InfAfterStep") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #327
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(IA, X), step: Coord(AU, Z), dest: Coord(AU, T) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.14107528336712583;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #328
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(TamMoveStepsDuringFormer { src: Coord(AI, N), step: Coord(Y, T), first_dest: Coord(O, T), second_dest: Coord(U, N) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.8988262464451912;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #329
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(IA, P), step: Coord(AU, P), planned_direction: Coord(IA, P) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::InfAfterStep(m) => {
        let chooser = 0.0796712898036116;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: Some(Coord(IA, P)) };
        let chooser2 = 0.1934984068629304;
        let hnr_state = apply_after_half_acceptance(&ext_state, aha_move, config)
            .unwrap()
            .choose_by_uniform_random_variable(chooser2)
            .0;
        
        hnr_state
    },
    _ => { panic!("Expected PureMove::InfAfterStep") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #330
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(E, Z), step: Coord(AI, Z), planned_direction: Coord(I, Z) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::InfAfterStep(m) => {
        let chooser = 0.5973395705300912;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: None };
        let chooser2 = 0.825500104620113;
        let hnr_state = apply_after_half_acceptance(&ext_state, aha_move, config)
            .unwrap()
            .choose_by_uniform_random_variable(chooser2)
            .0;
        
        hnr_state
    },
    _ => { panic!("Expected PureMove::InfAfterStep") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #331
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(Y, T), step: Coord(I, C), planned_direction: Coord(E, X) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::InfAfterStep(m) => {
        let chooser = 0.541748337180154;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: Some(Coord(E, X)) };
        let chooser2 = 0.5077088640463212;
        let hnr_state = apply_after_half_acceptance(&ext_state, aha_move, config)
            .unwrap()
            .choose_by_uniform_random_variable(chooser2)
            .0;
        
        hnr_state
    },
    _ => { panic!("Expected PureMove::InfAfterStep") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #332
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(TamMoveStepsDuringFormer { src: Coord(U, N), step: Coord(I, L), first_dest: Coord(U, L), second_dest: Coord(O, K) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.8371140846620776;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #333
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveFromHopZuo { color: Huok2, prof: Dau2, dest: Coord(U, C) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.7578663300650931;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #334
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(TamMoveNoStep { src: Coord(O, K), first_dest: Coord(Y, L), second_dest: Coord(AI, N) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.1614940487929002;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #335
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(AI, T), step: Coord(AI, N), dest: Coord(O, N) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.17799590041734958;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #336
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(TamMoveNoStep { src: Coord(AI, N), first_dest: Coord(Y, T), second_dest: Coord(O, N) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.18491639751695021;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #337
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveSrcDst { src: Coord(IA, Z), dest: Coord(AU, Z) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.8145359399151285;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #338
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(E, Z), step: Coord(AI, Z), planned_direction: Coord(AU, Z) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::InfAfterStep(m) => {
        let chooser = 0.5743103733430968;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: Some(Coord(AU, Z)) };
        let chooser2 = 0.427385937473042;
        let hnr_state = apply_after_half_acceptance(&ext_state, aha_move, config)
            .unwrap()
            .choose_by_uniform_random_variable(chooser2)
            .0;
        
        hnr_state
    },
    _ => { panic!("Expected PureMove::InfAfterStep") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::HandExists { if_tymok, if_taxot } => {
        state = if_tymok.clone();
    },
    _ => { panic!("Expected HandResolved::HandExists") }
}

// Turn #339
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveFromHopZuo { color: Kok1, prof: Nuak1, dest: Coord(O, T) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.8942376455081013;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #340
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveFromHopZuo { color: Huok2, prof: Io, dest: Coord(AU, C) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.46151768260420856;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

}
