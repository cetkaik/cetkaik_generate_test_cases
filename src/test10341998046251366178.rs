
#[test]
fn test10341998046251366178() {
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
let chooser = 0.017656362617417876;
let mut state = initial_state().choose_by_uniform_random_variable(chooser).0;

// Turn #1
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(IA, Z), step: Coord(AU, T), dest: Coord(AU, Z) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.8253588901127312;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(E, L), step: Coord(E, T), planned_direction: Coord(E, L) }));
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
        let chooser = 0.37056097585869996;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: Some(Coord(E, L)) };
        let chooser2 = 0.28019217814612074;
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

// Turn #3
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(TamMoveNoStep { src: Coord(O, Z), first_dest: Coord(O, X), second_dest: Coord(Y, Z) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.22482087844256304;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(A, M), step: Coord(I, X), dest: Coord(O, T) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.8038252626620049;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(TamMoveStepsDuringFormer { src: Coord(Y, Z), step: Coord(O, T), first_dest: Coord(U, T), second_dest: Coord(U, N) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.834068377612193;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(E, M), step: Coord(E, P), planned_direction: Coord(E, C) }));
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
        let chooser = 0.009935130316886531;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: None };
        let chooser2 = 0.17640131080404775;
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

// Turn #7
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(TamMoveStepsDuringFormer { src: Coord(U, N), step: Coord(O, T), first_dest: Coord(Y, N), second_dest: Coord(Y, T) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.7216061861639858;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(E, X), step: Coord(I, Z), dest: Coord(U, X) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.6750698799731982;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(IA, X), step: Coord(IA, C), dest: Coord(IA, X) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.9281168182837889;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(TamMoveStepsDuringLatter { src: Coord(Y, T), step: Coord(O, T), first_dest: Coord(O, Z), second_dest: Coord(U, N) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.1895051627479184;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcDst { src: Coord(AI, T), dest: Coord(Y, T) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.432104927375095;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(E, T), step: Coord(I, N), planned_direction: Coord(AI, C) }));
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
        let chooser = 0.7738398868946703;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: Some(Coord(U, T)) };
        let chooser2 = 0.05211688921362467;
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

// Turn #13
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(TamMoveStepsDuringFormer { src: Coord(U, N), step: Coord(O, T), first_dest: Coord(O, Z), second_dest: Coord(U, Z) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.7136968571895169;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(E, L), step: Coord(E, K), planned_direction: Coord(E, Z) }));
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
        let chooser = 0.15155759350092368;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: Some(Coord(E, L)) };
        let chooser2 = 0.4614117296021646;
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
let pure_move = Some(NormalMove(TamMoveStepsDuringFormer { src: Coord(U, Z), step: Coord(U, X), first_dest: Coord(O, X), second_dest: Coord(Y, X) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.5913810023235877;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(O, T), step: Coord(AI, X), planned_direction: Coord(IA, M) }));
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
        let chooser = 0.22205603512401584;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: None };
        let chooser2 = 0.4698171108993612;
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(IA, X), step: Coord(AU, Z), dest: Coord(AI, T) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.6426051904956254;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(I, Z), step: Coord(AI, Z), planned_direction: Coord(O, Z) }));
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
        let chooser = 0.5935941349906875;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: Some(Coord(O, Z)) };
        let chooser2 = 0.8338479235458577;
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

// Turn #19
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(TamMoveNoStep { src: Coord(Y, X), first_dest: Coord(Y, C), second_dest: Coord(O, M) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.6179881359594881;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
        let chooser: f64 = 0.5170764022502371;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
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
let pure_move = Some(NormalMove(TamMoveStepsDuringLatter { src: Coord(O, M), step: Coord(I, M), first_dest: Coord(U, C), second_dest: Coord(U, P) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.4298643595798256;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcDst { src: Coord(U, T), dest: Coord(AI, K) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.7045903390438801;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcDst { src: Coord(AU, Z), dest: Coord(IA, X) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.08088157349403213;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveFromHopZuo { color: Huok2, prof: Kauk2, dest: Coord(O, K) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.17236689543253425;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(AI, Z), step: Coord(O, Z), dest: Coord(O, X) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.794182501837906;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
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
        let chooser: f64 = 0.5278827405326761;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(TamMoveStepsDuringFormer { src: Coord(U, P), step: Coord(I, P), first_dest: Coord(U, M), second_dest: Coord(O, C) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.6014827426783541;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcDst { src: Coord(I, N), dest: Coord(U, N) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.30057337387675176;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(AU, M), step: Coord(IA, M), planned_direction: Coord(AU, M) }));
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
        let chooser = 0.614184378303829;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: Some(Coord(AU, M)) };
        let chooser2 = 0.45806474888783666;
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

// Turn #30
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(I, X), step: Coord(U, X), dest: Coord(I, X) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.08077359032866893;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(AU, M), step: Coord(AI, M), planned_direction: Coord(I, M) }));
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
        let chooser = 0.10570685342292929;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: None };
        let chooser2 = 0.08999300489071793;
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(E, Z), step: Coord(A, T), dest: Coord(E, Z) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.9247857626595093;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(TamMoveStepsDuringLatter { src: Coord(O, C), step: Coord(U, X), first_dest: Coord(U, C), second_dest: Coord(U, Z) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.044875469874888996;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(O, T), step: Coord(AI, L), dest: Coord(O, T) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.5539854046631313;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(TamMoveStepsDuringLatter { src: Coord(U, Z), step: Coord(O, T), first_dest: Coord(U, T), second_dest: Coord(U, Z) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.1770647711486839;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(U, X), step: Coord(AI, P), dest: Coord(Y, M) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.8351059697178731;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcDst { src: Coord(AU, X), dest: Coord(AI, Z) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.5911246878199834;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(A, M), step: Coord(I, M), planned_direction: Coord(E, M) }));
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
        let chooser = 0.1614161539616611;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: None };
        let chooser2 = 0.8661177955304407;
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
let pure_move = Some(NormalMove(NonTamMoveSrcDst { src: Coord(AI, T), dest: Coord(Y, N) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.8306122939885109;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(I, T), step: Coord(O, T), dest: Coord(Y, T) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.7870988940332153;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(IA, C), step: Coord(AI, C), dest: Coord(O, Z) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.8938947844720632;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
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
        let chooser: f64 = 0.5797737047898477;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveFromHopZuo { color: Kok1, prof: Nuak1, dest: Coord(Y, Z) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.7749724327732237;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
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
let pure_move = Some(NormalMove(TamMoveStepsDuringLatter { src: Coord(U, Z), step: Coord(O, Z), first_dest: Coord(U, T), second_dest: Coord(U, T) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.9476630627449508;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
        let chooser: f64 = 0.33100384473044697;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
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
let pure_move = Some(NormalMove(TamMoveStepsDuringFormer { src: Coord(U, T), step: Coord(O, T), first_dest: Coord(O, N), second_dest: Coord(Y, L) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.49184167262247835;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveFromHopZuo { color: Huok2, prof: Dau2, dest: Coord(I, N) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.7528074908715173;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(O, K), step: Coord(AI, K), dest: Coord(Y, K) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.6844520854149346;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(AI, N), step: Coord(AI, T), dest: Coord(Y, T) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.570175028365093;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(TamMoveStepsDuringLatter { src: Coord(Y, L), step: Coord(O, T), first_dest: Coord(O, N), second_dest: Coord(U, Z) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.8762385211146193;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveFromHopZuo { color: Kok1, prof: Kauk2, dest: Coord(Y, L) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.869096435721486;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(TamMoveStepsDuringFormer { src: Coord(U, Z), step: Coord(I, X), first_dest: Coord(U, Z), second_dest: Coord(I, Z) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.8434389515435986;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(O, Z), step: Coord(I, N), dest: Coord(A, K) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.5580198369149248;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
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
let pure_move = Some(NormalMove(TamMoveStepsDuringLatter { src: Coord(I, Z), step: Coord(I, X), first_dest: Coord(E, X), second_dest: Coord(I, Z) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.9879895201469474;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveFromHopZuo { color: Huok2, prof: Kua2, dest: Coord(U, Z) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.2398083470575363;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(A, M), step: Coord(I, M), planned_direction: Coord(Y, M) }));
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
        let chooser = 0.5237719931120732;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: Some(Coord(Y, M)) };
        let chooser2 = 0.9058628413238129;
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

// Turn #57
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(AU, L), step: Coord(AU, T), planned_direction: Coord(AI, T) }));
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
        let chooser = 0.5938441703457069;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: Some(Coord(AI, T)) };
        let chooser2 = 0.6546134671813697;
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

// Turn #58
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveFromHopZuo { color: Kok1, prof: Kauk2, dest: Coord(O, K) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.23182191456025192;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveFromHopZuo { color: Kok1, prof: Kauk2, dest: Coord(E, C) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.002469965207748781;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(TamMoveStepsDuringFormer { src: Coord(I, Z), step: Coord(E, Z), first_dest: Coord(E, T), second_dest: Coord(I, T) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.09801745712619225;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(IA, N), step: Coord(IA, K), dest: Coord(IA, N) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.639686804211661;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
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
let pure_move = Some(NormalMove(TamMoveStepsDuringLatter { src: Coord(I, T), step: Coord(E, Z), first_dest: Coord(E, T), second_dest: Coord(I, T) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.3009252276905726;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(AU, M), step: Coord(IA, M), planned_direction: Coord(Y, M) }));
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
        let chooser = 0.7759281468029565;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: None };
        let chooser2 = 0.7992598494097941;
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

// Turn #64
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveSrcDst { src: Coord(Y, M), dest: Coord(Y, Z) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.33782735122745233;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(AU, K), step: Coord(AU, T), planned_direction: Coord(AU, X) }));
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
        let chooser = 0.6453661268321812;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: Some(Coord(AU, X)) };
        let chooser2 = 0.3674838069471138;
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

// Turn #66
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(TamMoveStepsDuringLatter { src: Coord(I, T), step: Coord(U, Z), first_dest: Coord(I, Z), second_dest: Coord(I, Z) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.9671166534241338;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(U, Z), step: Coord(Y, Z), planned_direction: Coord(U, Z) }));
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
        let chooser = 0.4883938019256854;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: None };
        let chooser2 = 0.7854758647400845;
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
let pure_move = Some(NormalMove(TamMoveStepsDuringLatter { src: Coord(I, Z), step: Coord(U, Z), first_dest: Coord(U, X), second_dest: Coord(O, Z) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.22509956181832858;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(Y, Z), step: Coord(O, Z), planned_direction: Coord(Y, Z) }));
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
        let chooser = 0.33014250608347795;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: Some(Coord(Y, Z)) };
        let chooser2 = 0.41484934390027073;
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
let pure_move = Some(NormalMove(TamMoveStepsDuringFormer { src: Coord(O, Z), step: Coord(Y, Z), first_dest: Coord(Y, X), second_dest: Coord(Y, C) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.7168569417964702;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(AU, X), step: Coord(AI, X), planned_direction: Coord(O, M) }));
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
        let chooser = 0.17447480034059581;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: None };
        let chooser2 = 0.7561740701732115;
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

// Turn #72
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveSrcDst { src: Coord(O, T), dest: Coord(AI, L) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.7736832005373816;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #73
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveSrcDst { src: Coord(AU, T), dest: Coord(IA, Z) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.4054527997645717;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveFromHopZuo { color: Kok1, prof: Kauk2, dest: Coord(U, M) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.36804870165019965;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(AI, T), step: Coord(Y, T), planned_direction: Coord(O, N) }));
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
        let chooser = 0.3204094749009254;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: Some(Coord(U, L)) };
        let chooser2 = 0.6060700647315853;
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

// Turn #76
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(TamMoveStepsDuringFormer { src: Coord(Y, C), step: Coord(O, X), first_dest: Coord(O, Z), second_dest: Coord(O, T) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.4675993270542146;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(AU, P), step: Coord(IA, P), dest: Coord(AU, P) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.49206227361573573;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(A, N), step: Coord(I, N), dest: Coord(A, Z) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.6824878514701866;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(IA, X), step: Coord(AU, X), dest: Coord(IA, X) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.3313082174545622;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcDst { src: Coord(U, N), dest: Coord(O, N) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.9265594134468996;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(TamMoveStepsDuringLatter { src: Coord(O, T), step: Coord(I, L), first_dest: Coord(U, N), second_dest: Coord(U, N) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.5344918103558199;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(E, P), step: Coord(E, C), planned_direction: Coord(E, X) }));
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
        let chooser = 0.39461575425142315;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: Some(Coord(E, X)) };
        let chooser2 = 0.5787536784212084;
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

// Turn #83
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(TamMoveStepsDuringLatter { src: Coord(U, N), step: Coord(U, L), first_dest: Coord(O, L), second_dest: Coord(U, N) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.4237760395697461;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(E, Z), step: Coord(E, X), dest: Coord(I, Z) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.6297782259630328;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(AU, P), step: Coord(AU, M), dest: Coord(AI, M) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.7097279054074312;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(E, X), step: Coord(A, X), dest: Coord(E, X) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.92986169683577;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(IA, N), step: Coord(IA, Z), dest: Coord(IA, N) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.007476002981627317;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
        let chooser = 0.40864813081285245;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: Some(Coord(E, N)) };
        let chooser2 = 0.050477521419782456;
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

// Turn #89
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(TamMoveStepsDuringLatter { src: Coord(U, N), step: Coord(O, N), first_dest: Coord(O, T), second_dest: Coord(O, T) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.6479527062014993;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
        let chooser = 0.8566678857756876;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: Some(Coord(AI, P)) };
        let chooser2 = 0.7830702543992806;
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(Y, Z), step: Coord(Y, T), dest: Coord(Y, X) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.23354617766986385;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveFromHopZuo { color: Huok2, prof: Kauk2, dest: Coord(I, T) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.49160736365603896;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
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
let pure_move = Some(NormalMove(NonTamMoveSrcDst { src: Coord(Y, L), dest: Coord(O, L) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.6337431417553859;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(TamMoveStepsDuringLatter { src: Coord(O, T), step: Coord(U, Z), first_dest: Coord(O, Z), second_dest: Coord(O, T) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.3164027618684162;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
        let chooser: f64 = 0.2957188341850495;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(TamMoveStepsDuringLatter { src: Coord(O, T), step: Coord(AI, X), first_dest: Coord(Y, Z), second_dest: Coord(Y, Z) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.5009606164015848;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveFromHopZuo { color: Huok2, prof: Gua2, dest: Coord(U, N) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.4882750980643815;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(E, X), step: Coord(I, X), dest: Coord(E, X) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.9570311074510721;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(Y, N), step: Coord(AI, L), dest: Coord(Y, N) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.35822875809403865;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(TamMoveStepsDuringFormer { src: Coord(Y, Z), step: Coord(O, X), first_dest: Coord(O, Z), second_dest: Coord(O, T) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.2521938524055687;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(Y, M), step: Coord(Y, X), planned_direction: Coord(Y, C) }));
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
        let chooser = 0.9415953161560159;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: Some(Coord(Y, M)) };
        let chooser2 = 0.5655726186940343;
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
let pure_move = Some(NormalMove(TamMoveStepsDuringLatter { src: Coord(O, T), step: Coord(Y, X), first_dest: Coord(O, Z), second_dest: Coord(O, C) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.627790312583237;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(Y, M), step: Coord(Y, T), planned_direction: Coord(E, T) }));
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
        let chooser = 0.9953769622125758;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: Some(Coord(U, T)) };
        let chooser2 = 0.4423497290437439;
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

// Turn #104
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(AI, L), step: Coord(IA, T), dest: Coord(AI, L) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.7297975769324122;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(Y, N), step: Coord(Y, T), dest: Coord(Y, Z) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.6809696910340121;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(E, X), step: Coord(E, N), dest: Coord(I, N) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.841102838216494;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(TamMoveStepsDuringFormer { src: Coord(O, C), step: Coord(U, M), first_dest: Coord(O, C), second_dest: Coord(U, C) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.6892377762637591;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveFromHopZuo { color: Huok2, prof: Dau2, dest: Coord(Y, M) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.6753772541424933;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(U, T), step: Coord(U, Z), dest: Coord(O, Z) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.5538994878687706;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(TamMoveStepsDuringLatter { src: Coord(U, C), step: Coord(Y, X), first_dest: Coord(O, C), second_dest: Coord(Y, C) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.8322604741455508;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(O, Z), step: Coord(AI, Z), dest: Coord(AU, Z) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.15536501886497944;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(E, K), step: Coord(E, N), planned_direction: Coord(E, T) }));
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
        let chooser = 0.7572387917364576;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: None };
        let chooser2 = 0.6274243591711963;
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

// Turn #113
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(IA, K), step: Coord(IA, L), dest: Coord(IA, K) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.9793516482617465;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(TamMoveNoStep { src: Coord(Y, C), first_dest: Coord(O, C), second_dest: Coord(U, C) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.32400786535629444;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
        let chooser: f64 = 0.7078102775698218;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(TamMoveStepsDuringLatter { src: Coord(U, C), step: Coord(O, X), first_dest: Coord(O, C), second_dest: Coord(U, C) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.34655579687005267;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(AI, Z), step: Coord(Y, X), planned_direction: Coord(U, M) }));
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
        let chooser = 0.8821668412285745;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: None };
        let chooser2 = 0.30325494364289607;
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

// Turn #118
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(I, N), step: Coord(E, N), planned_direction: Coord(E, L) }));
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
        let chooser = 0.7470447147049372;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: None };
        let chooser2 = 0.8632099979373073;
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

// Turn #119
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(Y, Z), step: Coord(AI, X), dest: Coord(Y, Z) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.15556302985811832;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
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
let pure_move = Some(NormalMove(TamMoveStepsDuringLatter { src: Coord(U, C), step: Coord(U, M), first_dest: Coord(O, M), second_dest: Coord(U, C) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.6395045992837276;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(IA, X), step: Coord(AU, C), dest: Coord(IA, X) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.42001686363174606;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(AI, P), step: Coord(I, P), planned_direction: Coord(O, P) }));
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
        let chooser = 0.8033498562478605;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: Some(Coord(Y, P)) };
        let chooser2 = 0.4659879242381013;
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

// Turn #123
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(TamMoveStepsDuringLatter { src: Coord(U, C), step: Coord(I, X), first_dest: Coord(U, X), second_dest: Coord(U, X) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.34413114493801733;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(I, N), step: Coord(E, N), planned_direction: Coord(E, Z) }));
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
        let chooser = 0.6030710736727309;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: Some(Coord(E, X)) };
        let chooser2 = 0.9709502658600272;
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

// Turn #125
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(AI, X), step: Coord(Y, X), dest: Coord(AI, X) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.584260542107803;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(E, X), step: Coord(E, N), planned_direction: Coord(E, C) }));
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
        let chooser = 0.7776754886019378;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: None };
        let chooser2 = 0.9234825028819948;
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
let pure_move = Some(NormalMove(NonTamMoveSrcDst { src: Coord(Y, X), dest: Coord(Y, M) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.7833112022121553;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(E, X), step: Coord(I, X), planned_direction: Coord(U, C) }));
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
        let chooser = 0.9306219356893827;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: Some(Coord(O, M)) };
        let chooser2 = 0.273988807957561;
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

// Turn #129
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveFromHopZuo { color: Huok2, prof: Dau2, dest: Coord(E, L) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.6022833744884687;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(TamMoveStepsDuringLatter { src: Coord(U, X), step: Coord(U, M), first_dest: Coord(U, C), second_dest: Coord(O, P) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.9093291871884767;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(A, K), step: Coord(I, K), dest: Coord(I, N) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.444466649895337;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(O, M), step: Coord(A, T), dest: Coord(E, T) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.4198220603794337;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcDst { src: Coord(E, L), dest: Coord(A, N) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.6068133094042564;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
        let chooser: f64 = 0.4387184302387479;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(AU, X), step: Coord(AU, C), dest: Coord(IA, C) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.5389973894130426;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(A, X), step: Coord(E, C), dest: Coord(A, X) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.326059639621062;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(U, N), step: Coord(O, N), planned_direction: Coord(U, N) }));
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
        let chooser = 0.35744135533004573;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: None };
        let chooser2 = 0.7344065507402054;
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

// Turn #138
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(E, K), step: Coord(E, N), planned_direction: Coord(E, L) }));
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
        let chooser = 0.18752409695989614;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: Some(Coord(E, L)) };
        let chooser2 = 0.27657840021941094;
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

// Turn #139
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(IA, T), step: Coord(AU, Z), dest: Coord(AU, T) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.31529044226537284;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(E, N), step: Coord(E, T), planned_direction: Coord(A, T) }));
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
        let chooser = 0.42059263424273796;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: None };
        let chooser2 = 0.4698533255173025;
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

// Turn #141
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(AI, Z), step: Coord(Y, T), planned_direction: Coord(AI, N) }));
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
        let chooser = 0.7092081475010826;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: Some(Coord(AI, N)) };
        let chooser2 = 0.3508804642141732;
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
let pure_move = Some(NormalMove(TamMoveStepsDuringFormer { src: Coord(O, P), step: Coord(U, M), first_dest: Coord(O, C), second_dest: Coord(Y, C) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.8066524724005044;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(U, Z), step: Coord(I, Z), planned_direction: Coord(A, Z) }));
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
        let chooser = 0.44413031964455874;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: Some(Coord(E, Z)) };
        let chooser2 = 0.6514166823940725;
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

// Turn #144
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(TamMoveStepsDuringLatter { src: Coord(Y, C), step: Coord(Y, P), first_dest: Coord(AI, M), second_dest: Coord(AI, P) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.5777179744948715;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(U, L), step: Coord(U, N), planned_direction: Coord(O, N) }));
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
        let chooser = 0.10533832549307676;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: None };
        let chooser2 = 0.059983816470293494;
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

// Turn #146
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(E, T), step: Coord(E, Z), planned_direction: Coord(E, T) }));
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
        let chooser = 0.6731398304501627;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: Some(Coord(E, T)) };
        let chooser2 = 0.3543816419954626;
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(U, N), step: Coord(O, N), planned_direction: Coord(Y, N) }));
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
        let chooser = 0.625608693640888;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: None };
        let chooser2 = 0.3330582568989825;
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

// Turn #148
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(TamMoveStepsDuringFormer { src: Coord(AI, P), step: Coord(Y, P), first_dest: Coord(AI, M), second_dest: Coord(AU, M) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.26962696975165446;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(IA, C), step: Coord(AU, C), planned_direction: Coord(O, C) }));
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
        let chooser = 0.6712198376912866;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: None };
        let chooser2 = 0.7116555974990362;
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

// Turn #150
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(TamMoveStepsDuringLatter { src: Coord(AU, M), step: Coord(AU, C), first_dest: Coord(AI, M), second_dest: Coord(AU, M) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.26176985969925937;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(IA, C), step: Coord(IA, M), planned_direction: Coord(IA, C) }));
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
        let chooser = 0.12864600868312293;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: Some(Coord(IA, C)) };
        let chooser2 = 0.16352920046546138;
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

// Turn #152
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(TamMoveNoStep { src: Coord(AU, M), first_dest: Coord(AU, P), second_dest: Coord(AU, M) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.7378597136956196;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(AU, C), step: Coord(Y, P), planned_direction: Coord(AU, P) }));
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
        let chooser = 0.8795282191348238;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: Some(Coord(AU, P)) };
        let chooser2 = 0.018768857201709954;
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
let pure_move = Some(NormalMove(TamMoveStepsDuringLatter { src: Coord(AU, M), step: Coord(IA, M), first_dest: Coord(AU, C), second_dest: Coord(AU, M) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.5268939596089285;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(IA, C), step: Coord(IA, M), planned_direction: Coord(IA, C) }));
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
        let chooser = 0.6796525408135716;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: None };
        let chooser2 = 0.9434288209536422;
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

// Turn #156
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveSrcDst { src: Coord(E, X), dest: Coord(E, Z) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.8244809004238991;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
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
let pure_move = Some(NormalMove(TamMoveStepsDuringFormer { src: Coord(AU, M), step: Coord(IA, M), first_dest: Coord(AU, C), second_dest: Coord(AI, M) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.14074666097065547;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveFromHopZuo { color: Huok2, prof: Kua2, dest: Coord(U, C) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.8017636467271838;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(TamMoveStepsDuringLatter { src: Coord(AI, M), step: Coord(AI, C), first_dest: Coord(AU, C), second_dest: Coord(AI, M) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.8918318750284364;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(I, Z), step: Coord(E, Z), dest: Coord(A, T) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.5648509896090291;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(TamMoveStepsDuringLatter { src: Coord(AI, M), step: Coord(O, X), first_dest: Coord(Y, C), second_dest: Coord(O, C) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.014998060179377548;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(E, Z), step: Coord(A, X), dest: Coord(E, C) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.8573606152726808;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(TamMoveStepsDuringLatter { src: Coord(O, C), step: Coord(Y, M), first_dest: Coord(O, M), second_dest: Coord(Y, C) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.8234024028786828;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveFromHopZuo { color: Kok1, prof: Kauk2, dest: Coord(AI, P) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.7317861596586129;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(IA, M), step: Coord(AI, X), planned_direction: Coord(IA, M) }));
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
        let chooser = 0.5715450908325048;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: None };
        let chooser2 = 0.28949331414788315;
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

// Turn #166
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(A, C), step: Coord(I, C), dest: Coord(O, P) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.395800467349381;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(AI, X), step: Coord(O, X), dest: Coord(Y, X) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.5405609589063327;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
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
let pure_move = Some(NormalMove(NonTamMoveSrcDst { src: Coord(AI, P), dest: Coord(AU, P) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.938596067246472;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcDst { src: Coord(O, X), dest: Coord(O, T) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.28863686555237655;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveFromHopZuo { color: Kok1, prof: Gua2, dest: Coord(AI, X) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.7037459422874109;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(IA, X), step: Coord(IA, Z), dest: Coord(IA, X) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.7087215785548112;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(E, N), step: Coord(I, N), planned_direction: Coord(O, Z) }));
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
        let chooser = 0.9865048758548057;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: Some(Coord(Y, X)) };
        let chooser2 = 0.3605658121843701;
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
let pure_move = Some(NormalMove(TamMoveStepsDuringLatter { src: Coord(Y, C), step: Coord(AI, C), first_dest: Coord(AI, M), second_dest: Coord(AU, X) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.5264737964204216;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveFromHopZuo { color: Kok1, prof: Kauk2, dest: Coord(AU, C) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.469425506437926;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(Y, Z), step: Coord(AI, X), dest: Coord(AU, C) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.3478495794859997;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(E, C), step: Coord(I, X), dest: Coord(U, Z) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.2887612137530816;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveFromHopZuo { color: Kok1, prof: Kauk2, dest: Coord(O, X) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.03935828846383249;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(Y, P), step: Coord(AU, P), dest: Coord(AU, M) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.5554930250232729;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(AU, Z), step: Coord(AU, X), planned_direction: Coord(O, N) }));
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
        let chooser = 0.8314630890619529;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: Some(Coord(AI, Z)) };
        let chooser2 = 0.8934785217974083;
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

// Turn #180
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(A, X), step: Coord(E, C), dest: Coord(A, X) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.9396082911580215;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(IA, X), step: Coord(AU, C), dest: Coord(AU, M) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.10803285581709621;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(U, C), step: Coord(U, M), dest: Coord(U, C) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.33966008892235855;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
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
let pure_move = Some(NormalMove(NonTamMoveFromHopZuo { color: Kok1, prof: Kua2, dest: Coord(E, P) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.05546450824066884;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(A, X), step: Coord(E, C), dest: Coord(A, X) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.1186612099021438;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(U, L), step: Coord(U, N), planned_direction: Coord(U, K) }));
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
        let chooser = 0.026541020523345726;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: None };
        let chooser2 = 0.5048047513536904;
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

// Turn #186
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(TamMoveStepsDuringFormer { src: Coord(AU, X), step: Coord(AU, C), first_dest: Coord(AI, M), second_dest: Coord(AI, P) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.9348461706237914;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(AI, N), step: Coord(Y, T), planned_direction: Coord(O, N) }));
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
        let chooser = 0.6385378204659776;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: Some(Coord(O, N)) };
        let chooser2 = 0.7831633106404581;
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(A, L), step: Coord(I, T), dest: Coord(O, L) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.01775624390087438;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveFromHopZuo { color: Huok2, prof: Kauk2, dest: Coord(E, Z) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.34133441871032055;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveFromHopZuo { color: Kok1, prof: Kauk2, dest: Coord(Y, L) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.7665036816731507;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
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
let pure_move = Some(NormalMove(NonTamMoveSrcDst { src: Coord(IA, L), dest: Coord(AI, T) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.3868774794769321;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(AU, P), step: Coord(AI, P), dest: Coord(IA, P) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.7392345586140214;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
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
let pure_move = Some(NormalMove(TamMoveStepsDuringLatter { src: Coord(AI, P), step: Coord(AU, M), first_dest: Coord(AI, M), second_dest: Coord(AI, M) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.9057913166998706;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveFromHopZuo { color: Huok2, prof: Kua2, dest: Coord(AU, K) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.9811004350339598;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(AU, T), step: Coord(AI, Z), dest: Coord(Y, X) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.2347409171843876;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(U, C), step: Coord(AI, C), planned_direction: Coord(O, C) }));
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
        let chooser = 0.6849865904916063;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: Some(Coord(U, C)) };
        let chooser2 = 0.015296156949361372;
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

// Turn #197
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveFromHopZuo { color: Kok1, prof: Gua2, dest: Coord(U, Z) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.36299518424131993;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(A, T), step: Coord(E, T), dest: Coord(A, T) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.36395506236232844;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(AU, C), step: Coord(IA, M), dest: Coord(AU, C) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.7218137971100955;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(TamMoveStepsDuringLatter { src: Coord(AI, M), step: Coord(Y, M), first_dest: Coord(Y, C), second_dest: Coord(Y, C) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.6099585323331617;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
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
let pure_move = Some(NormalMove(NonTamMoveSrcDst { src: Coord(O, X), dest: Coord(U, X) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.2021913636104138;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(E, C), step: Coord(A, X), dest: Coord(E, X) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.13055525706624183;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
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
let pure_move = Some(NormalMove(NonTamMoveSrcDst { src: Coord(IA, C), dest: Coord(IA, X) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.9591382320050275;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(TamMoveStepsDuringFormer { src: Coord(Y, C), step: Coord(AI, C), first_dest: Coord(Y, C), second_dest: Coord(O, M) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.08740640756018381;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(Y, M), step: Coord(O, M), dest: Coord(O, X) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.7118687756397936;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcDst { src: Coord(I, T), dest: Coord(U, T) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.41955360182688495;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(TamMoveStepsDuringLatter { src: Coord(O, M), step: Coord(I, P), first_dest: Coord(U, P), second_dest: Coord(E, M) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.7734292628878524;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcDst { src: Coord(O, L), dest: Coord(AI, T) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.2787448234051485;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(U, Z), step: Coord(U, T), planned_direction: Coord(E, X) }));
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
        let chooser = 0.8155017169452142;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: None };
        let chooser2 = 0.1934652259810986;
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
let pure_move = Some(NormalMove(NonTamMoveFromHopZuo { color: Kok1, prof: Maun1, dest: Coord(I, Z) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.3610551721003129;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
        let chooser: f64 = 0.9915662767639672;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcDst { src: Coord(U, T), dest: Coord(I, T) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.29529134646975774;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
        let chooser: f64 = 0.1610583593935797;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(TamMoveNoStep { src: Coord(O, M), first_dest: Coord(O, C), second_dest: Coord(Y, C) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.2542352195333658;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(O, X), step: Coord(Y, X), dest: Coord(Y, Z) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.5930158523970893;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
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
let pure_move = Some(NormalMove(TamMoveStepsDuringFormer { src: Coord(Y, C), step: Coord(AI, C), first_dest: Coord(AI, M), second_dest: Coord(AU, P) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.02626726126755441;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcDst { src: Coord(U, X), dest: Coord(I, X) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.2645687741327448;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(E, L), step: Coord(E, T), planned_direction: Coord(E, Z) }));
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
        let chooser = 0.642912235879091;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: None };
        let chooser2 = 0.46691326167188196;
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

// Turn #219
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveFromHopZuo { color: Kok1, prof: Kauk2, dest: Coord(IA, C) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.43674579532859614;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(O, P), step: Coord(I, P), dest: Coord(O, P) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.23276491314092984;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
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
let pure_move = Some(NormalMove(NonTamMoveSrcDst { src: Coord(AU, C), dest: Coord(AI, X) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.29678994690889005;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
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
let pure_move = Some(NormalMove(NonTamMoveSrcDst { src: Coord(I, Z), dest: Coord(O, C) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.022510990699528666;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
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
let pure_move = Some(NormalMove(NonTamMoveFromHopZuo { color: Kok1, prof: Gua2, dest: Coord(A, K) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.1265908446776436;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(E, L), step: Coord(E, T), planned_direction: Coord(E, Z) }));
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
        let chooser = 0.7538374045654774;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: Some(Coord(E, Z)) };
        let chooser2 = 0.12960086934893456;
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

// Turn #225
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(IA, X), step: Coord(IA, Z), planned_direction: Coord(IA, T) }));
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
        let chooser = 0.23339801299703655;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: None };
        let chooser2 = 0.4442882127680967;
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

// Turn #226
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveFromHopZuo { color: Huok2, prof: Kauk2, dest: Coord(AU, Z) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.01824092099722896;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(Y, X), step: Coord(AI, C), dest: Coord(AI, M) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.7450670470406665;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcDst { src: Coord(E, Z), dest: Coord(I, Z) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.1811571778497253;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(U, N), step: Coord(U, L), planned_direction: Coord(I, L) }));
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
        let chooser = 0.7935787248703695;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: None };
        let chooser2 = 0.15063672579959686;
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

// Turn #230
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(O, C), step: Coord(I, P), dest: Coord(O, C) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.38646590571059525;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(AI, Z), step: Coord(Y, Z), planned_direction: Coord(Y, C) }));
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
        let chooser = 0.4107208414853789;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: None };
        let chooser2 = 0.4010448757238474;
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

// Turn #232
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(AI, K), step: Coord(Y, L), dest: Coord(O, N) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.08644821650915269;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(AI, X), step: Coord(Y, Z), dest: Coord(O, X) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.5556404109106945;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(E, X), step: Coord(E, Z), dest: Coord(E, X) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.8689654158675475;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(U, N), step: Coord(U, L), planned_direction: Coord(U, K) }));
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
        let chooser = 0.7042480502805596;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: None };
        let chooser2 = 0.9488004599993493;
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

// Turn #236
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(A, X), step: Coord(E, Z), dest: Coord(I, X) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.08222382829181951;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcDst { src: Coord(U, N), dest: Coord(U, T) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.07389397274528575;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveFromHopZuo { color: Kok1, prof: Kauk2, dest: Coord(AU, N) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.3565823711509435;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(U, Z), step: Coord(U, C), planned_direction: Coord(U, M) }));
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
        let chooser = 0.3412378156243234;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: Some(Coord(U, M)) };
        let chooser2 = 0.3517743282154675;
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
let pure_move = Some(NormalMove(NonTamMoveSrcDst { src: Coord(I, C), dest: Coord(E, C) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.24092667042093763;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveFromHopZuo { color: Kok1, prof: Kauk2, dest: Coord(U, K) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.6158782904560706;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(AI, T), step: Coord(O, X), dest: Coord(AI, M) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.15174206529326562;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(TamMoveStepsDuringLatter { src: Coord(AU, P), step: Coord(AI, M), first_dest: Coord(AI, P), second_dest: Coord(AI, P) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.033643704690413;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveFromHopZuo { color: Kok1, prof: Uai1, dest: Coord(AU, X) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.9027283639484833;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(TamMoveStepsDuringLatter { src: Coord(AI, P), step: Coord(AI, M), first_dest: Coord(Y, P), second_dest: Coord(Y, M) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.9612804388882022;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(E, X), step: Coord(E, C), dest: Coord(A, M) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.5916074083751368;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(U, T), step: Coord(I, N), planned_direction: Coord(O, Z) }));
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
        let chooser = 0.6821302005564767;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: Some(Coord(O, Z)) };
        let chooser2 = 0.6190203208425232;
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(I, X), step: Coord(U, C), dest: Coord(O, X) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.9756715053825616;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(TamMoveStepsDuringFormer { src: Coord(Y, M), step: Coord(AI, C), first_dest: Coord(Y, M), second_dest: Coord(Y, C) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.8322032023134882;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveFromHopZuo { color: Huok2, prof: Uai1, dest: Coord(Y, X) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.44395550961976504;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(U, M), step: Coord(I, M), planned_direction: Coord(U, M) }));
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
        let chooser = 0.894193317601979;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: Some(Coord(AI, M)) };
        let chooser2 = 0.5341010279209883;
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

// Turn #252
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(TamMoveStepsDuringLatter { src: Coord(Y, C), step: Coord(AI, Z), first_dest: Coord(AI, X), second_dest: Coord(AI, T) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.029785580372328924;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveFromHopZuo { color: Huok2, prof: Maun1, dest: Coord(A, P) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.7470296438454299;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
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
let pure_move = Some(NormalMove(TamMoveStepsDuringLatter { src: Coord(AI, T), step: Coord(AU, N), first_dest: Coord(AI, N), second_dest: Coord(AI, T) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.988971533326834;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(O, Z), step: Coord(Y, T), planned_direction: Coord(U, X) }));
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
        let chooser = 0.34220350050701076;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: None };
        let chooser2 = 0.0662437175249061;
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

// Turn #256
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(A, T), step: Coord(E, T), dest: Coord(A, T) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.12139182966024131;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(IA, X), step: Coord(IA, Z), planned_direction: Coord(IA, T) }));
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
        let chooser = 0.2893316252555458;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: Some(Coord(IA, T)) };
        let chooser2 = 0.5343165488431832;
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

// Turn #258
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(E, Z), step: Coord(E, C), planned_direction: Coord(E, M) }));
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
        let chooser = 0.14020852894989555;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: None };
        let chooser2 = 0.02709932654751135;
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

// Turn #259
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(TamMoveStepsDuringLatter { src: Coord(AI, T), step: Coord(AI, L), first_dest: Coord(Y, N), second_dest: Coord(AI, N) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.7126119622165259;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(Y, L), step: Coord(AI, L), dest: Coord(Y, L) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.4608511162237582;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
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
let pure_move = Some(NormalMove(TamMoveStepsDuringLatter { src: Coord(AI, N), step: Coord(O, N), first_dest: Coord(Y, N), second_dest: Coord(U, T) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.8674390791675152;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
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
let pure_move = Some(NormalMove(NonTamMoveSrcDst { src: Coord(U, C), dest: Coord(U, X) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.8318006248001538;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(AI, Z), step: Coord(AI, C), planned_direction: Coord(E, C) }));
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
        let chooser = 0.5766094871561492;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: Some(Coord(U, C)) };
        let chooser2 = 0.6663672576702927;
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

// Turn #264
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(Y, X), step: Coord(O, X), dest: Coord(Y, X) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.8017564948708521;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(O, T), step: Coord(U, T), dest: Coord(U, X) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.7043821577248587;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcDst { src: Coord(Y, X), dest: Coord(AI, X) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.7729859582594995;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveFromHopZuo { color: Huok2, prof: Kua2, dest: Coord(A, X) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.7575763885391151;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(AI, X), step: Coord(AU, Z), dest: Coord(AI, X) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.6170791004665949;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(I, N), step: Coord(A, K), dest: Coord(I, K) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.7665190945140136;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
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
let pure_move = Some(NormalMove(NonTamMoveSrcDst { src: Coord(A, Z), dest: Coord(A, N) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.22497540490178347;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveFromHopZuo { color: Huok2, prof: Kauk2, dest: Coord(O, M) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.9367288419623022;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveFromHopZuo { color: Huok2, prof: Dau2, dest: Coord(AU, L) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.14206021545364866;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(IA, T), step: Coord(IA, Z), planned_direction: Coord(IA, X) }));
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
        let chooser = 0.32935388538507504;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: None };
        let chooser2 = 0.462311685145224;
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

// Turn #274
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveSrcDst { src: Coord(A, N), dest: Coord(I, N) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.45148868562803834;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(AU, M), step: Coord(AI, M), dest: Coord(AU, M) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.1956440768969493;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(TamMoveStepsDuringFormer { src: Coord(U, T), step: Coord(I, T), first_dest: Coord(I, Z), second_dest: Coord(U, Z) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.7892597637715465;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(U, L), step: Coord(I, L), planned_direction: Coord(Y, L) }));
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
        let chooser = 0.10176951275111279;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: None };
        let chooser2 = 0.019134815788013904;
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

// Turn #278
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(TamMoveStepsDuringFormer { src: Coord(U, Z), step: Coord(U, X), first_dest: Coord(I, X), second_dest: Coord(I, Z) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.9931986833995782;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(U, C), step: Coord(U, X), planned_direction: Coord(Y, X) }));
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
        let chooser = 0.331932617043354;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: Some(Coord(O, X)) };
        let chooser2 = 0.4368171641759966;
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(E, Z), step: Coord(I, T), planned_direction: Coord(AI, M) }));
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
        let chooser = 0.705856107777171;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: Some(Coord(Y, C)) };
        let chooser2 = 0.37442871086086926;
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

// Turn #281
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(O, Z), step: Coord(AI, C), planned_direction: Coord(O, P) }));
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
        let chooser = 0.5151973907020634;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: Some(Coord(Y, M)) };
        let chooser2 = 0.986729658209964;
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

// Turn #282
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveSrcDst { src: Coord(O, X), dest: Coord(Y, Z) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.000650497364814262;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(A, X), step: Coord(U, X), planned_direction: Coord(O, X) }));
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
        let chooser = 0.21212855620977566;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: None };
        let chooser2 = 0.56238761755222;
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
let pure_move = Some(NormalMove(NonTamMoveFromHopZuo { color: Kok1, prof: Nuak1, dest: Coord(AU, C) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.7110458358538533;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(AU, M), step: Coord(IA, C), dest: Coord(AU, X) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.9104564701917767;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(E, T), step: Coord(I, N), planned_direction: Coord(U, L) }));
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
        let chooser = 0.09888544475120498;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: Some(Coord(U, L)) };
        let chooser2 = 0.9612583305773865;
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

// Turn #287
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveFromHopZuo { color: Kok1, prof: Uai1, dest: Coord(IA, L) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.18314117804123908;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveFromHopZuo { color: Huok2, prof: Gua2, dest: Coord(E, Z) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.46315915023852094;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(IA, L), step: Coord(AU, L), dest: Coord(AI, L) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.9488671136067165;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(I, T), step: Coord(I, Z), dest: Coord(U, Z) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.15440029212920048;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveFromHopZuo { color: Kok1, prof: Maun1, dest: Coord(E, T) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.41177324701918905;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(TamMoveStepsDuringFormer { src: Coord(I, Z), step: Coord(I, T), first_dest: Coord(U, T), second_dest: Coord(O, Z) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.8518750170894942;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(I, K), step: Coord(I, N), dest: Coord(O, K) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.2877620613176992;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(U, L), step: Coord(U, X), planned_direction: Coord(A, X) }));
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
        let chooser = 0.6361673725823563;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: None };
        let chooser2 = 0.06620676705260387;
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

// Turn #295
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveFromHopZuo { color: Kok1, prof: Kauk2, dest: Coord(Y, P) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.24931488509715805;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(Y, C), step: Coord(Y, Z), planned_direction: Coord(AI, T) }));
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
        let chooser = 0.47107043380412905;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: None };
        let chooser2 = 0.3088175917826792;
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(Y, T), step: Coord(Y, Z), dest: Coord(Y, X) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.9561486747051117;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(A, T), step: Coord(E, Z), dest: Coord(A, Z) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.855939967421005;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(AI, L), step: Coord(Y, L), dest: Coord(AI, N) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.7930423281786735;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcDst { src: Coord(Y, C), dest: Coord(Y, M) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.774267366689316;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(TamMoveStepsDuringLatter { src: Coord(O, Z), step: Coord(O, N), first_dest: Coord(O, T), second_dest: Coord(U, T) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.5714342843250309;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveFromHopZuo { color: Huok2, prof: Gua2, dest: Coord(AU, T) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.8095748927731015;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(O, K), step: Coord(AI, K), dest: Coord(O, K) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.8034683939213714;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(E, Z), step: Coord(A, Z), planned_direction: Coord(A, X) }));
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
        let chooser = 0.7438607795277739;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: Some(Coord(A, X)) };
        let chooser2 = 0.2215133631072801;
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(Y, X), step: Coord(Y, Z), dest: Coord(O, Z) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.7426361869673709;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveFromHopZuo { color: Huok2, prof: Kua2, dest: Coord(A, N) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.9637700456689403;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
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
let pure_move = Some(NormalMove(TamMoveStepsDuringFormer { src: Coord(U, T), step: Coord(O, Z), first_dest: Coord(O, T), second_dest: Coord(U, Z) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.5229106215390703;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
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
let pure_move = Some(NormalMove(NonTamMoveSrcDst { src: Coord(A, X), dest: Coord(U, X) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.4447735564247449;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(AI, N), step: Coord(Y, L), dest: Coord(O, L) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.10178845057902719;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveFromHopZuo { color: Huok2, prof: Nuak1, dest: Coord(E, X) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.8030930591358799;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcDst { src: Coord(E, P), dest: Coord(I, P) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.2183915001429877;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(U, L), step: Coord(O, L), planned_direction: Coord(O, K) }));
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
        let chooser = 0.17818355389401563;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: None };
        let chooser2 = 0.07859291020808723;
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

// Turn #313
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveFromHopZuo { color: Huok2, prof: Kauk2, dest: Coord(A, T) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.8576708066226528;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(TamMoveNoStep { src: Coord(U, Z), first_dest: Coord(O, T), second_dest: Coord(Y, N) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.42670227428026075;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcDst { src: Coord(AI, M), dest: Coord(AI, P) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.8879261817351165;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(AI, X), step: Coord(AU, C), dest: Coord(AU, X) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.3042662365097917;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::HandExists { if_tymok, if_taxot } => {
        


        match if_taxot {
            IfTaxot::NextSeason(ps) => {
                let chooser = 0.5562162985295133;
                state = ps.clone().choose_by_uniform_random_variable(chooser).0
            },
            IfTaxot::VictoriousSide(v) => {
                panic!("Expected IfTaxot::VictoriousSide")
            }
        }
    },
    _ => { panic!("Expected HandResolved::HandExists") }
}
                                
// Turn #317
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(TamMoveNoStep { src: Coord(O, Z), first_dest: Coord(Y, X), second_dest: Coord(O, C) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.12467710417379951;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
        let chooser: f64 = 0.4620762718058673;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
        let chooser: f64 = 0.8550551152667998;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
        let chooser: f64 = 0.9445589392950716;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
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
let pure_move = Some(NormalMove(TamMoveStepsDuringLatter { src: Coord(O, C), step: Coord(O, M), first_dest: Coord(Y, M), second_dest: Coord(U, M) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.617171667342479;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcDst { src: Coord(IA, T), dest: Coord(AU, N) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.31957074272567876;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(A, L), step: Coord(I, T), dest: Coord(A, L) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.7257015201065793;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(TamMoveStepsDuringFormer { src: Coord(U, M), step: Coord(I, C), first_dest: Coord(E, C), second_dest: Coord(A, C) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.8771795460173976;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(A, L), step: Coord(I, T), dest: Coord(A, L) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.4700487545355556;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(IA, C), step: Coord(AI, C), dest: Coord(O, P) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.4282338232439601;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(O, Z), step: Coord(AI, N), dest: Coord(O, K) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.9711240718754718;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(AI, Z), step: Coord(I, Z), planned_direction: Coord(E, Z) }));
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
        let chooser = 0.9246392300482263;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: Some(Coord(E, Z)) };
        let chooser2 = 0.655967750167864;
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

// Turn #329
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(TamMoveStepsDuringFormer { src: Coord(A, C), step: Coord(E, M), first_dest: Coord(E, C), second_dest: Coord(A, C) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.829737799160569;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(AU, N), step: Coord(AU, T), dest: Coord(AI, Z) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.6336397876841136;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(E, X), step: Coord(I, C), planned_direction: Coord(AI, N) }));
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
        let chooser = 0.7342580359149811;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: None };
        let chooser2 = 0.4250052208845857;
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(Y, K), step: Coord(O, K), dest: Coord(Y, K) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.7850967071989527;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(TamMoveStepsDuringFormer { src: Coord(A, C), step: Coord(A, X), first_dest: Coord(A, C), second_dest: Coord(E, C) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.02832304316568146;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(O, M), step: Coord(I, X), planned_direction: Coord(Y, N) }));
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
        let chooser = 0.32700659448042935;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: Some(Coord(O, T)) };
        let chooser2 = 0.9397090724519208;
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

// Turn #335
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(TamMoveStepsDuringFormer { src: Coord(E, C), step: Coord(I, X), first_dest: Coord(U, X), second_dest: Coord(O, X) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.3104274932172787;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(IA, X), step: Coord(IA, Z), dest: Coord(IA, T) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.30127421640772145;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(TamMoveNoStep { src: Coord(O, X), first_dest: Coord(U, X), second_dest: Coord(U, C) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.7707161214340523;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(AI, Z), step: Coord(AI, X), dest: Coord(AU, Z) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.7619423259780836;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::NeitherTymokNorTaxot(s) => {
        state = s.clone();
    },
    _ => { panic!("Expected HandResolved::NeitherTymokNorTaxot") }
}

// Turn #339
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
        let chooser: f64 = 0.45849218271985237;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(TamMoveStepsDuringLatter { src: Coord(U, C), step: Coord(O, P), first_dest: Coord(U, M), second_dest: Coord(U, M) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.4493459507056592;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
