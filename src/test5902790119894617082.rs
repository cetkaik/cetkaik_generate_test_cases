
#[test]
fn test5902790119894617082() {
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
let chooser = 0.39391790800033755;
let mut state = initial_state().choose_by_uniform_random_variable(chooser).0;

// Turn #1
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(IA, Z), step: Coord(AU, X), dest: Coord(AU, C) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.1410499832674793;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcDst { src: Coord(I, C), dest: Coord(O, C) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.5989356727454095;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(TamMoveNoStep { src: Coord(O, Z), first_dest: Coord(Y, Z), second_dest: Coord(O, T) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.26141095636256106;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(E, M), step: Coord(E, P), planned_direction: Coord(E, M) }));
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
        let chooser = 0.5501399548596732;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: Some(Coord(E, M)) };
        let chooser2 = 0.14421340859763698;
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

// Turn #5
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(AU, X), step: Coord(AI, C), planned_direction: Coord(O, Z) }));
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
        let chooser = 0.6216927909291781;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: None };
        let chooser2 = 0.6527082366430321;
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
let pure_move = Some(NormalMove(TamMoveNoStep { src: Coord(O, T), first_dest: Coord(O, N), second_dest: Coord(Y, N) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.45716096067546574;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
        let chooser: f64 = 0.13686898076284804;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(TamMoveStepsDuringLatter { src: Coord(Y, N), step: Coord(AI, T), first_dest: Coord(Y, T), second_dest: Coord(Y, N) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.8418804124732461;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcDst { src: Coord(AI, X), dest: Coord(Y, X) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.7365041739440447;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(TamMoveStepsDuringFormer { src: Coord(Y, N), step: Coord(AI, T), first_dest: Coord(Y, T), second_dest: Coord(AI, Z) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.3843906570263712;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
        let chooser = 0.17497297535668488;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: Some(Coord(Y, L)) };
        let chooser2 = 0.47451428118579697;
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

// Turn #12
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(TamMoveStepsDuringFormer { src: Coord(AI, Z), step: Coord(AU, X), first_dest: Coord(AI, Z), second_dest: Coord(Y, Z) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.04021911533506173;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
        let chooser: f64 = 0.12969906975975454;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(TamMoveStepsDuringFormer { src: Coord(Y, Z), step: Coord(AI, T), first_dest: Coord(AU, Z), second_dest: Coord(IA, Z) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.06031319991717243;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
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
let pure_move = Some(NormalMove(NonTamMoveSrcDst { src: Coord(AI, N), dest: Coord(Y, N) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.025576455942355247;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(E, L), step: Coord(I, L), planned_direction: Coord(Y, L) }));
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
        let chooser = 0.07662109717200971;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: None };
        let chooser2 = 0.7038378396102397;
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
let pure_move = Some(NormalMove(TamMoveStepsDuringFormer { src: Coord(IA, Z), step: Coord(IA, T), first_dest: Coord(AU, N), second_dest: Coord(AI, N) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.6252163929929224;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(E, L), step: Coord(I, L), planned_direction: Coord(E, L) }));
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
        let chooser = 0.1488538985435146;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: Some(Coord(E, L)) };
        let chooser2 = 0.8477884381142953;
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(Y, L), step: Coord(AI, N), planned_direction: Coord(U, X) }));
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
        let chooser = 0.2020229027261733;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: Some(Coord(Y, T)) };
        let chooser2 = 0.6094810535016733;
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

// Turn #20
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveSrcDst { src: Coord(I, K), dest: Coord(U, K) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.7059307383420835;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(TamMoveStepsDuringFormer { src: Coord(AI, N), step: Coord(Y, N), first_dest: Coord(O, L), second_dest: Coord(O, N) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.855060638831481;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(E, M), step: Coord(I, M), planned_direction: Coord(Y, M) }));
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
        let chooser = 0.24532696342360005;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: Some(Coord(U, M)) };
        let chooser2 = 0.8558919011619912;
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

// Turn #23
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(Y, N), step: Coord(O, N), dest: Coord(Y, N) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.17178356656733562;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(U, M), step: Coord(U, Z), planned_direction: Coord(U, X) }));
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
        let chooser = 0.5811635383972781;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: Some(Coord(U, C)) };
        let chooser2 = 0.737296517316934;
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

// Turn #25
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(TamMoveStepsDuringLatter { src: Coord(O, N), step: Coord(U, Z), first_dest: Coord(U, T), second_dest: Coord(U, X) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.10471236576795884;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(A, T), step: Coord(E, T), dest: Coord(E, Z) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.19019504510192997;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(TamMoveStepsDuringLatter { src: Coord(U, X), step: Coord(U, C), first_dest: Coord(O, X), second_dest: Coord(I, C) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.45916959740189633;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(A, M), step: Coord(I, X), planned_direction: Coord(A, T) }));
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
        let chooser = 0.9447748287117913;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: None };
        let chooser2 = 0.5281464665116559;
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
let pure_move = Some(NormalMove(TamMoveStepsDuringLatter { src: Coord(I, C), step: Coord(O, C), first_dest: Coord(U, M), second_dest: Coord(O, M) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.5383427404712317;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(E, Z), step: Coord(I, T), dest: Coord(E, N) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.91616442615832;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
        let chooser: f64 = 0.29276573790540505;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(U, C), step: Coord(AI, T), planned_direction: Coord(AU, T) }));
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
        let chooser = 0.581920940840061;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: Some(Coord(AU, T)) };
        let chooser2 = 0.4592266245785821;
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

// Turn #33
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(TamMoveStepsDuringLatter { src: Coord(O, M), step: Coord(AI, M), first_dest: Coord(Y, C), second_dest: Coord(Y, M) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.6818522396732772;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(A, X), step: Coord(A, C), dest: Coord(E, C) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.6868264526374883;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
        let chooser: f64 = 0.260497041860806;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(E, N), step: Coord(I, N), dest: Coord(E, N) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.4670664593767926;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(TamMoveNoStep { src: Coord(Y, C), first_dest: Coord(O, M), second_dest: Coord(U, P) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.2858305991261273;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(AU, T), step: Coord(AU, X), planned_direction: Coord(AI, X) }));
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
        let chooser = 0.4518649274344453;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: Some(Coord(Y, X)) };
        let chooser2 = 0.07205044800098725;
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
        let chooser = 0.08601760986295037;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: None };
        let chooser2 = 0.7106494206532353;
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

// Turn #40
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveFromHopZuo { color: Kok1, prof: Kauk2, dest: Coord(Y, C) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.012009908052916907;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcDst { src: Coord(IA, M), dest: Coord(AI, X) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.8269629281927512;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(E, N), step: Coord(I, N), dest: Coord(E, N) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.2615363273161294;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(TamMoveStepsDuringLatter { src: Coord(U, P), step: Coord(O, C), first_dest: Coord(U, M), second_dest: Coord(U, C) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.8697483571715642;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(E, L), step: Coord(I, L), planned_direction: Coord(U, L) }));
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
        let chooser = 0.9455541382860277;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: Some(Coord(Y, L)) };
        let chooser2 = 0.8376934539905299;
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

// Turn #45
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(TamMoveStepsDuringFormer { src: Coord(U, C), step: Coord(I, X), first_dest: Coord(I, C), second_dest: Coord(U, M) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.9204945013508611;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(A, L), step: Coord(I, T), dest: Coord(A, X) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.6650121801537953;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(AU, X), step: Coord(AI, C), planned_direction: Coord(O, P) }));
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
        let chooser = 0.7754804781850109;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: Some(Coord(O, P)) };
        let chooser2 = 0.2474195211341763;
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

// Turn #48
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(I, M), step: Coord(U, M), dest: Coord(Y, M) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.210483967616795;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(TamMoveStepsDuringFormer { src: Coord(U, M), step: Coord(I, P), first_dest: Coord(U, M), second_dest: Coord(U, P) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.051240087246744115;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(I, Z), step: Coord(U, Z), planned_direction: Coord(O, Z) }));
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
        let chooser = 0.18529197857073132;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: Some(Coord(O, Z)) };
        let chooser2 = 0.0196459070792927;
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

// Turn #51
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(IA, L), step: Coord(AI, T), dest: Coord(O, L) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.6138514451698683;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcDst { src: Coord(O, Z), dest: Coord(AI, Z) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.6718307915573557;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(TamMoveNoStep { src: Coord(U, P), first_dest: Coord(O, M), second_dest: Coord(U, C) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.1907265413219238;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(A, X), step: Coord(I, T), dest: Coord(A, X) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.26893500545215976;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(TamMoveNoStep { src: Coord(U, C), first_dest: Coord(I, C), second_dest: Coord(U, C) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.05343766442797182;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(Y, X), step: Coord(O, C), planned_direction: Coord(I, Z) }));
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
        let chooser = 0.23917365857225203;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: None };
        let chooser2 = 0.1801517112851273;
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
let pure_move = Some(NormalMove(TamMoveStepsDuringLatter { src: Coord(U, C), step: Coord(U, Z), first_dest: Coord(O, X), second_dest: Coord(O, X) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.9274203107287017;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(Y, L), step: Coord(O, L), planned_direction: Coord(O, N) }));
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
        let chooser = 0.26728180746833163;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: Some(Coord(O, T)) };
        let chooser2 = 0.6943770703659609;
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

// Turn #59
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(TamMoveStepsDuringFormer { src: Coord(O, X), step: Coord(O, C), first_dest: Coord(U, X), second_dest: Coord(I, C) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.011542075597004597;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
        let chooser: f64 = 0.1828093246606548;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(TamMoveNoStep { src: Coord(I, C), first_dest: Coord(U, M), second_dest: Coord(O, M) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.7631725743594693;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(Y, M), step: Coord(O, M), dest: Coord(Y, M) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.4226565187216583;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(AI, X), step: Coord(O, T), dest: Coord(AI, X) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.8536608717363633;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(TamMoveStepsDuringFormer { src: Coord(O, M), step: Coord(Y, C), first_dest: Coord(O, M), second_dest: Coord(U, M) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.19562304895130678;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(AI, C), step: Coord(Y, C), dest: Coord(O, C) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.1620149467777663;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(TamMoveStepsDuringLatter { src: Coord(U, M), step: Coord(I, P), first_dest: Coord(I, M), second_dest: Coord(U, P) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.03358073239830628;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(O, P), step: Coord(E, X), dest: Coord(A, C) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.09006616151890212;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(O, T), step: Coord(O, C), planned_direction: Coord(O, T) }));
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
        let chooser = 0.4711784904179679;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: Some(Coord(O, Z)) };
        let chooser2 = 0.8467019235511896;
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

// Turn #69
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveFromHopZuo { color: Kok1, prof: Kaun1, dest: Coord(AU, Z) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.44706232099387555;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
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
let pure_move = Some(NormalMove(TamMoveStepsDuringLatter { src: Coord(U, P), step: Coord(O, C), first_dest: Coord(O, M), second_dest: Coord(O, M) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.4628598248562331;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(Y, T), step: Coord(O, Z), planned_direction: Coord(A, P) }));
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
        let chooser = 0.469685357940552;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: None };
        let chooser2 = 0.0990654760028914;
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
let pure_move = Some(NormalMove(TamMoveNoStep { src: Coord(O, M), first_dest: Coord(U, M), second_dest: Coord(U, P) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.06435637651838533;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(IA, T), step: Coord(AU, Z), dest: Coord(IA, T) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.9512513187265974;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcDst { src: Coord(O, Z), dest: Coord(E, M) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.9372773454702387;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(TamMoveStepsDuringFormer { src: Coord(U, P), step: Coord(I, P), first_dest: Coord(U, M), second_dest: Coord(U, C) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.6311718888420065;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcDst { src: Coord(E, K), dest: Coord(E, L) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.6360514635204592;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(TamMoveNoStep { src: Coord(U, C), first_dest: Coord(O, X), second_dest: Coord(O, Z) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.39043840995766277;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(E, N), step: Coord(I, T), dest: Coord(U, Z) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.2769275338516305;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
        let chooser: f64 = 0.598230289965167;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
        let chooser: f64 = 0.23365969928300823;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(TamMoveStepsDuringFormer { src: Coord(O, Z), step: Coord(U, Z), first_dest: Coord(O, T), second_dest: Coord(U, T) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.9914340330060174;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(E, L), step: Coord(I, L), dest: Coord(U, L) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.6576072252839379;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(TamMoveStepsDuringLatter { src: Coord(U, T), step: Coord(U, L), first_dest: Coord(O, N), second_dest: Coord(O, N) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.9811183014636888;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(E, M), step: Coord(A, M), planned_direction: Coord(O, M) }));
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
        let chooser = 0.8173663567156179;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: None };
        let chooser2 = 0.4485491984448642;
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

// Turn #85
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(AU, L), step: Coord(AU, Z), planned_direction: Coord(AU, T) }));
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
        let chooser = 0.49236322989298575;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: None };
        let chooser2 = 0.05244109975152467;
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

// Turn #86
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(TamMoveStepsDuringFormer { src: Coord(O, N), step: Coord(Y, T), first_dest: Coord(O, N), second_dest: Coord(U, N) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.7713283661449527;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(AI, X), step: Coord(IA, T), dest: Coord(AI, X) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.17077610284901668;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(TamMoveStepsDuringFormer { src: Coord(U, N), step: Coord(I, N), first_dest: Coord(U, T), second_dest: Coord(O, N) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.6576872519911855;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(AU, P), step: Coord(AU, M), dest: Coord(IA, M) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.021631737039800703;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(TamMoveStepsDuringLatter { src: Coord(O, N), step: Coord(Y, N), first_dest: Coord(O, T), second_dest: Coord(Y, L) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.646199863714275;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
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
let pure_move = Some(NormalMove(NonTamMoveSrcDst { src: Coord(A, C), dest: Coord(E, X) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.552406504501207;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(A, Z), step: Coord(E, T), dest: Coord(A, T) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.11939790778119619;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveFromHopZuo { color: Huok2, prof: Dau2, dest: Coord(I, Z) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.7894826006148908;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcDst { src: Coord(AI, Z), dest: Coord(AU, Z) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.3715097180912228;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(AU, C), step: Coord(IA, X), dest: Coord(AU, C) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.10408254869399658;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveFromHopZuo { color: Kok1, prof: Kaun1, dest: Coord(U, C) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.8589090995172284;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
        let chooser: f64 = 0.6576305733196982;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(U, L), step: Coord(U, Z), dest: Coord(I, Z) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.9705762661994802;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveFromHopZuo { color: Kok1, prof: Kauk2, dest: Coord(E, Z) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.8400947558468659;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveFromHopZuo { color: Huok2, prof: Dau2, dest: Coord(A, Z) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.5492030713227334;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(O, L), step: Coord(I, T), dest: Coord(O, L) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.5526177172217482;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(I, Z), step: Coord(I, T), planned_direction: Coord(I, Z) }));
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
        let chooser = 0.6169241929920141;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: Some(Coord(I, Z)) };
        let chooser2 = 0.421187630848635;
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

// Turn #103
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(Y, T), step: Coord(E, M), dest: Coord(I, P) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.5800567173828766;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcDst { src: Coord(E, N), dest: Coord(A, L) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.04051855103432622;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveFromHopZuo { color: Huok2, prof: Kauk2, dest: Coord(U, M) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.37006760904190195;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(TamMoveStepsDuringFormer { src: Coord(Y, L), step: Coord(AI, L), first_dest: Coord(AI, N), second_dest: Coord(Y, T) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.1731240662424609;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(IA, M), step: Coord(IA, P), planned_direction: Coord(IA, M) }));
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
        let chooser = 0.26017102983053986;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: None };
        let chooser2 = 0.5180295927421505;
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

// Turn #108
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
        let chooser: f64 = 0.17380300500481383;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcDst { src: Coord(AI, X), dest: Coord(O, T) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.5340219609889703;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(TamMoveStepsDuringLatter { src: Coord(Y, T), step: Coord(O, T), first_dest: Coord(O, Z), second_dest: Coord(Y, Z) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.5876663873923476;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(O, L), step: Coord(AI, T), planned_direction: Coord(U, C) }));
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
        let chooser = 0.6378276192895681;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: None };
        let chooser2 = 0.943971825001932;
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
let pure_move = Some(NormalMove(TamMoveStepsDuringFormer { src: Coord(Y, Z), step: Coord(O, T), first_dest: Coord(Y, T), second_dest: Coord(AI, Z) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.04727937294828766;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(AU, C), step: Coord(IA, X), dest: Coord(AU, X) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.4619658813300743;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(TamMoveStepsDuringFormer { src: Coord(AI, Z), step: Coord(AU, X), first_dest: Coord(AI, Z), second_dest: Coord(AU, T) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.7339650249703316;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(IA, P), step: Coord(AI, P), planned_direction: Coord(AU, P) }));
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
        let chooser = 0.03632354506412394;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: Some(Coord(AU, P)) };
        let chooser2 = 0.8021820169884819;
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

// Turn #116
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(TamMoveStepsDuringFormer { src: Coord(AU, T), step: Coord(IA, T), first_dest: Coord(IA, Z), second_dest: Coord(AU, T) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.2038494024072226;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
        let chooser: f64 = 0.9502286020589662;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(TamMoveStepsDuringFormer { src: Coord(AU, T), step: Coord(AI, T), first_dest: Coord(AU, N), second_dest: Coord(AU, T) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.9510561872551965;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(AU, P), step: Coord(AU, M), dest: Coord(AU, P) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.8640160803620385;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(Y, X), step: Coord(AI, C), planned_direction: Coord(Y, X) }));
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
        let chooser = 0.8293138919767965;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: None };
        let chooser2 = 0.4899507562387435;
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
let pure_move = Some(NormalMove(TamMoveStepsDuringFormer { src: Coord(AU, T), step: Coord(AU, Z), first_dest: Coord(AU, T), second_dest: Coord(IA, Z) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.21371374266246868;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(A, Z), step: Coord(E, T), dest: Coord(A, Z) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.956191084748608;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
        let chooser: f64 = 0.49763656143576185;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(I, Z), step: Coord(U, Z), planned_direction: Coord(U, T) }));
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
        let chooser = 0.5951112395602549;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: Some(Coord(U, T)) };
        let chooser2 = 0.8406988653800843;
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
let pure_move = Some(NormalMove(TamMoveNoStep { src: Coord(IA, Z), first_dest: Coord(AU, T), second_dest: Coord(IA, Z) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.8692691750088719;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(E, P), step: Coord(E, M), dest: Coord(I, M) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.08263661919231324;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(I, P), step: Coord(E, M), dest: Coord(A, P) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.6284546476441871;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcDst { src: Coord(U, T), dest: Coord(I, Z) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.5414442319491695;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveFromHopZuo { color: Kok1, prof: Kua2, dest: Coord(IA, L) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.6772420372793736;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcDst { src: Coord(Y, X), dest: Coord(AU, T) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.8280392526117778;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(AU, X), step: Coord(IA, Z), dest: Coord(AU, Z) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.8768580476770347;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(I, X), step: Coord(U, X), dest: Coord(U, Z) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.6016614066488353;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveFromHopZuo { color: Kok1, prof: Nuak1, dest: Coord(U, N) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.511751322543714;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveFromHopZuo { color: Huok2, prof: Nuak1, dest: Coord(Y, Z) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.8221237230279456;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(TamMoveStepsDuringFormer { src: Coord(IA, Z), step: Coord(AU, Z), first_dest: Coord(IA, Z), second_dest: Coord(AU, X) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.6167830267279898;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(I, N), step: Coord(U, N), dest: Coord(O, N) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.7516752336828944;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcDst { src: Coord(AI, L), dest: Coord(Y, L) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.8138653574436374;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcDst { src: Coord(I, Z), dest: Coord(E, Z) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.34623727624604494;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcDst { src: Coord(IA, N), dest: Coord(AI, N) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.4789096415057563;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveFromHopZuo { color: Kok1, prof: Kauk2, dest: Coord(U, L) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.5737160371843252;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(TamMoveStepsDuringLatter { src: Coord(AU, X), step: Coord(IA, C), first_dest: Coord(AU, C), second_dest: Coord(AU, X) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.00817059248041696;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(E, C), step: Coord(A, M), dest: Coord(E, C) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.3451346846136465;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(TamMoveStepsDuringLatter { src: Coord(AU, X), step: Coord(AU, M), first_dest: Coord(AU, C), second_dest: Coord(IA, P) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.6903634868277055;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(E, M), step: Coord(I, M), planned_direction: Coord(I, X) }));
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
        let chooser = 0.9461818944654126;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: Some(Coord(I, X)) };
        let chooser2 = 0.5519901661958971;
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

// Turn #145
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveSrcDst { src: Coord(U, X), dest: Coord(O, C) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.8224173585708774;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcDst { src: Coord(O, N), dest: Coord(Y, N) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.13056199162336446;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
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
let pure_move = Some(NormalMove(NonTamMoveFromHopZuo { color: Huok2, prof: Kauk2, dest: Coord(Y, X) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.24705181549319222;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveFromHopZuo { color: Huok2, prof: Kauk2, dest: Coord(O, X) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.5186155210964772;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcDst { src: Coord(AI, N), dest: Coord(O, Z) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.4899157970462541;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(AU, T), step: Coord(AU, Z), planned_direction: Coord(AU, T) }));
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
        let chooser = 0.20068731422690111;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: None };
        let chooser2 = 0.1821674340446281;
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

// Turn #151
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(AU, P), step: Coord(IA, P), planned_direction: Coord(AU, P) }));
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
        let chooser = 0.7484813829751296;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: Some(Coord(AU, P)) };
        let chooser2 = 0.8323482389670417;
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
let pure_move = Some(NormalMove(NonTamMoveSrcDst { src: Coord(E, C), dest: Coord(E, X) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.11053231662190532;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(AU, P), step: Coord(AU, M), planned_direction: Coord(AU, C) }));
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
        let chooser = 0.716672393296078;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: Some(Coord(AU, X)) };
        let chooser2 = 0.03456770802022624;
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
let pure_move = Some(NormalMove(TamMoveStepsDuringLatter { src: Coord(IA, P), step: Coord(AU, M), first_dest: Coord(AU, P), second_dest: Coord(IA, P) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.6675881173161712;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(AU, K), step: Coord(AU, L), planned_direction: Coord(AU, T) }));
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
        let chooser = 0.9106183649003888;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: Some(Coord(AU, T)) };
        let chooser2 = 0.9779402870137098;
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(A, L), step: Coord(A, N), dest: Coord(E, L) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.6631260087492612;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveFromHopZuo { color: Huok2, prof: Gua2, dest: Coord(O, N) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.9503351925212691;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(E, Z), step: Coord(E, X), planned_direction: Coord(E, P) }));
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
        let chooser = 0.7304758555169519;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: Some(Coord(E, M)) };
        let chooser2 = 0.08463310855403883;
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

// Turn #159
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(AU, X), step: Coord(IA, X), planned_direction: Coord(AI, X) }));
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
        let chooser = 0.9700396235178871;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: None };
        let chooser2 = 0.18851980100489818;
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(A, T), step: Coord(A, N), dest: Coord(A, L) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.9287440033707671;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(O, C), step: Coord(Y, M), dest: Coord(O, P) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.9872110414070718;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcDst { src: Coord(I, M), dest: Coord(I, P) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.6219955780047983;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(TamMoveStepsDuringLatter { src: Coord(IA, P), step: Coord(IA, M), first_dest: Coord(AU, P), second_dest: Coord(AU, C) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.5302681877918088;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(A, N), step: Coord(A, K), dest: Coord(I, K) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.7536565331864246;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(TamMoveStepsDuringFormer { src: Coord(AU, C), step: Coord(AU, M), first_dest: Coord(AU, C), second_dest: Coord(AI, M) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.16876931591723032;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(I, X), step: Coord(I, P), planned_direction: Coord(I, M) }));
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
        let chooser = 0.6087687189777666;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: Some(Coord(I, C)) };
        let chooser2 = 0.7325166511707675;
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

// Turn #167
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(IA, L), step: Coord(IA, K), planned_direction: Coord(AU, K) }));
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
        let chooser = 0.4209284089180245;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: Some(Coord(AI, K)) };
        let chooser2 = 0.07954461644865318;
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(A, L), step: Coord(A, K), dest: Coord(A, L) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.11146932630334172;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(AU, T), step: Coord(IA, T), planned_direction: Coord(IA, Z) }));
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
        let chooser = 0.0023169525643492728;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: None };
        let chooser2 = 0.43375712908168373;
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

// Turn #170
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveSrcDst { src: Coord(E, T), dest: Coord(A, N) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.9866510635376723;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(AI, K), step: Coord(Y, K), planned_direction: Coord(AI, K) }));
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
        let chooser = 0.37405289119721397;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: None };
        let chooser2 = 0.5207117364253357;
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

// Turn #172
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(TamMoveStepsDuringLatter { src: Coord(AI, M), step: Coord(AU, M), first_dest: Coord(AU, P), second_dest: Coord(AI, M) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.7927172315662653;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
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
let pure_move = Some(NormalMove(NonTamMoveSrcDst { src: Coord(IA, M), dest: Coord(IA, P) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.18271216877329055;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(TamMoveStepsDuringFormer { src: Coord(AI, M), step: Coord(Y, C), first_dest: Coord(O, M), second_dest: Coord(Y, P) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.19856658726819076;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(IA, T), step: Coord(AU, Z), dest: Coord(AI, Z) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.07138621830818248;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(A, L), step: Coord(A, K), dest: Coord(E, K) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.577223831586461;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(AU, T), step: Coord(AI, T), dest: Coord(AU, T) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.9447260324447552;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(TamMoveStepsDuringLatter { src: Coord(Y, P), step: Coord(O, P), first_dest: Coord(O, M), second_dest: Coord(Y, P) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.5565882324680496;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(AU, T), step: Coord(AU, L), planned_direction: Coord(AU, K) }));
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
        let chooser = 0.4161719421302469;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: Some(Coord(AU, K)) };
        let chooser2 = 0.17683471091655878;
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
        let chooser: f64 = 0.7516085027428405;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(IA, C), step: Coord(IA, P), dest: Coord(IA, C) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.5352539761335988;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveFromHopZuo { color: Kok1, prof: Kauk2, dest: Coord(AI, X) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.03925314092466736;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(AU, L), step: Coord(AU, Z), planned_direction: Coord(Y, Z) }));
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
        let chooser = 0.5792913819992287;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: Some(Coord(Y, Z)) };
        let chooser2 = 0.585283927683626;
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

// Turn #184
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(I, P), step: Coord(I, C), planned_direction: Coord(I, P) }));
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
        let chooser = 0.029843499173932697;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: None };
        let chooser2 = 0.7759992920875288;
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

// Turn #185
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveFromHopZuo { color: Huok2, prof: Nuak1, dest: Coord(E, N) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.4617143329331793;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(I, P), step: Coord(I, C), planned_direction: Coord(I, Z) }));
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
        let chooser = 0.8511503925606239;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: Some(Coord(I, Z)) };
        let chooser2 = 0.49695626933286785;
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

// Turn #187
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(AI, K), step: Coord(Y, K), planned_direction: Coord(U, K) }));
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
        let chooser = 0.6826753118829745;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: None };
        let chooser2 = 0.5616156644430188;
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(I, P), step: Coord(I, C), planned_direction: Coord(A, P) }));
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
        let chooser = 0.6152006849228061;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: Some(Coord(A, P)) };
        let chooser2 = 0.4855307603369905;
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

// Turn #189
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(Y, Z), step: Coord(AU, Z), planned_direction: Coord(O, Z) }));
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
        let chooser = 0.7604547189980415;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: Some(Coord(Y, Z)) };
        let chooser2 = 0.49885721686142137;
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

// Turn #190
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
        let chooser: f64 = 0.0727412714935739;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(AU, M), step: Coord(AU, X), planned_direction: Coord(AU, M) }));
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
        let chooser = 0.5847413812059308;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: Some(Coord(AU, M)) };
        let chooser2 = 0.47949293655149994;
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

// Turn #192
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(Y, C), step: Coord(AI, C), dest: Coord(Y, C) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.930580524173454;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(TamMoveStepsDuringFormer { src: Coord(Y, P), step: Coord(Y, M), first_dest: Coord(AI, M), second_dest: Coord(AU, C) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.7021235360179082;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcDst { src: Coord(I, K), dest: Coord(O, K) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.6346313313074359;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(AI, C), step: Coord(AI, X), dest: Coord(AI, Z) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.2665671057008355;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(I, C), step: Coord(AI, N), planned_direction: Coord(AU, T) }));
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
        let chooser = 0.2343579031261107;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: Some(Coord(IA, Z)) };
        let chooser2 = 0.5556787684180835;
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
let pure_move = Some(NormalMove(NonTamMoveSrcDst { src: Coord(O, T), dest: Coord(AI, X) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.33088020224328774;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(Y, C), step: Coord(AI, C), dest: Coord(AI, M) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.49507266728758803;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveFromHopZuo { color: Kok1, prof: Kauk2, dest: Coord(Y, C) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.5148902900904196;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(Y, N), step: Coord(AI, N), dest: Coord(AU, N) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.48555901451757366;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
        let chooser: f64 = 0.2754863732980426;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(A, K), step: Coord(E, K), planned_direction: Coord(I, K) }));
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
        let chooser = 0.15760403655231547;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: Some(Coord(I, K)) };
        let chooser2 = 0.1885656958229791;
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
let pure_move = Some(NormalMove(NonTamMoveFromHopZuo { color: Huok2, prof: Kauk2, dest: Coord(A, L) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.8884269905981353;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(E, M), step: Coord(E, X), planned_direction: Coord(E, P) }));
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
        let chooser = 0.37097326003696085;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: Some(Coord(E, C)) };
        let chooser2 = 0.617899891818906;
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

// Turn #205
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(Y, Z), step: Coord(Y, X), planned_direction: Coord(O, C) }));
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
        let chooser = 0.6651576927124072;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: Some(Coord(O, C)) };
        let chooser2 = 0.2510726819421736;
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

// Turn #206
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(I, N), step: Coord(Y, X), planned_direction: Coord(I, N) }));
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
        let chooser = 0.979369932488212;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: Some(Coord(U, T)) };
        let chooser2 = 0.21221718290452307;
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

// Turn #207
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(AU, M), step: Coord(AI, P), planned_direction: Coord(AI, M) }));
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
        let chooser = 0.8780844807056525;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: None };
        let chooser2 = 0.8805543594183131;
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

// Turn #208
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(O, K), step: Coord(I, K), dest: Coord(A, K) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.07440884383885416;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(O, N), step: Coord(U, N), planned_direction: Coord(U, L) }));
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
        let chooser = 0.6099277312325807;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: Some(Coord(U, K)) };
        let chooser2 = 0.677969681951184;
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
let pure_move = Some(NormalMove(TamMoveStepsDuringFormer { src: Coord(AU, C), step: Coord(AI, M), first_dest: Coord(AU, P), second_dest: Coord(IA, M) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.11092404259072708;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveFromHopZuo { color: Huok2, prof: Kauk2, dest: Coord(AU, T) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.5138336068621406;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(TamMoveStepsDuringLatter { src: Coord(IA, M), step: Coord(IA, C), first_dest: Coord(AU, C), second_dest: Coord(IA, M) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.9149922375840929;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(O, C), step: Coord(Y, C), planned_direction: Coord(U, C) }));
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
        let chooser = 0.17824869831453105;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: Some(Coord(O, C)) };
        let chooser2 = 0.07983574557529982;
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

// Turn #214
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(IA, Z), step: Coord(IA, X), planned_direction: Coord(IA, C) }));
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
        let chooser = 0.416592267638649;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: None };
        let chooser2 = 0.765596364706298;
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

// Turn #215
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(AU, Z), step: Coord(AI, X), dest: Coord(AU, Z) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.7747969303264229;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(E, X), step: Coord(A, Z), dest: Coord(A, T) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.0021215412089850005;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(O, C), step: Coord(O, P), planned_direction: Coord(U, P) }));
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
        let chooser = 0.8604939210885475;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: Some(Coord(E, P)) };
        let chooser2 = 0.4952647186075638;
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

// Turn #218
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(I, K), step: Coord(E, K), planned_direction: Coord(U, K) }));
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
        let chooser = 0.757570798388842;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: None };
        let chooser2 = 0.2047025420852574;
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
let pure_move = Some(NormalMove(TamMoveStepsDuringFormer { src: Coord(IA, M), step: Coord(IA, C), first_dest: Coord(IA, M), second_dest: Coord(AU, C) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.6169123564150207;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(E, K), step: Coord(A, L), dest: Coord(E, K) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.8547934095158726;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(TamMoveStepsDuringFormer { src: Coord(AU, C), step: Coord(AI, X), first_dest: Coord(AU, C), second_dest: Coord(IA, M) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.5335649528297081;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(U, T), step: Coord(Y, L), dest: Coord(O, N) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.3783992749189684;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(TamMoveNoStep { src: Coord(IA, M), first_dest: Coord(AU, C), second_dest: Coord(IA, M) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.02454607947749643;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcDst { src: Coord(U, T), dest: Coord(I, Z) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.3229937119157017;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcDst { src: Coord(IA, P), dest: Coord(AU, P) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.9295521237273464;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(A, T), step: Coord(A, N), dest: Coord(E, N) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.36997100469010935;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(AU, X), step: Coord(AU, Z), planned_direction: Coord(U, Z) }));
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
        let chooser = 0.195094429494171;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: Some(Coord(AI, Z)) };
        let chooser2 = 0.4515072015960476;
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

// Turn #228
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveFromHopZuo { color: Huok2, prof: Nuak1, dest: Coord(O, T) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.5452342302826002;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(AU, Z), step: Coord(IA, X), dest: Coord(AU, C) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.6288405958930648;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcDst { src: Coord(IA, Z), dest: Coord(O, Z) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.191434763451362;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(TamMoveStepsDuringFormer { src: Coord(IA, M), step: Coord(AU, P), first_dest: Coord(IA, P), second_dest: Coord(IA, M) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.4120257381058857;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(I, K), step: Coord(U, K), dest: Coord(U, L) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.5182180482188248;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(AU, K), step: Coord(IA, K), dest: Coord(AU, K) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.2793693980639026;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(E, L), step: Coord(I, L), dest: Coord(I, N) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.24613408440676598;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(E, P), step: Coord(A, P), planned_direction: Coord(E, P) }));
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
        let chooser = 0.33635498355004967;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: Some(Coord(E, P)) };
        let chooser2 = 0.5342663777403769;
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(E, C), step: Coord(E, P), planned_direction: Coord(E, X) }));
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
        let chooser = 0.04399932089599312;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: None };
        let chooser2 = 0.11351193454470998;
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

// Turn #237
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(AU, C), step: Coord(AU, M), dest: Coord(AU, C) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.07962702338141836;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(E, C), step: Coord(E, N), planned_direction: Coord(E, M) }));
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
        let chooser = 0.3100908341938635;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: Some(Coord(E, Z)) };
        let chooser2 = 0.3558880199481175;
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

// Turn #239
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(AU, M), step: Coord(IA, C), planned_direction: Coord(AU, M) }));
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
        let chooser = 0.06947547018723532;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: Some(Coord(AU, M)) };
        let chooser2 = 0.4439542762399551;
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(U, L), step: Coord(U, K), planned_direction: Coord(I, K) }));
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
        let chooser = 0.5772071043970644;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: None };
        let chooser2 = 0.5422779729867543;
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

// Turn #241
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(IA, X), step: Coord(AU, X), dest: Coord(AI, Z) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.2017278045295835;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
        let chooser: f64 = 0.27134647035336856;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcDst { src: Coord(U, K), dest: Coord(E, K) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.11574521374843239;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
};
let resolved = resolve(&hnr_state, config);

match &resolved {
    HandResolved::HandExists { if_tymok, if_taxot } => {
        state = if_tymok.clone();
    },
    _ => { panic!("Expected HandResolved::HandExists") }
}

// Turn #244
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(A, K), step: Coord(A, N), dest: Coord(A, K) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.12568276938831613;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveFromHopZuo { color: Kok1, prof: Io, dest: Coord(I, P) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.37182805523243234;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(TamMoveNoStep { src: Coord(IA, M), first_dest: Coord(IA, P), second_dest: Coord(IA, M) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.8009286733709247;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(AI, K), step: Coord(Y, K), planned_direction: Coord(U, K) }));
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
        let chooser = 0.680192628013898;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: None };
        let chooser2 = 0.6284642982228329;
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(IA, Z), step: Coord(IA, K), planned_direction: Coord(IA, X) }));
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
        let chooser = 0.633551245718157;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: Some(Coord(IA, L)) };
        let chooser2 = 0.49483641743345186;
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

// Turn #249
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(TamMoveStepsDuringFormer { src: Coord(IA, M), step: Coord(AU, M), first_dest: Coord(IA, M), second_dest: Coord(IA, P) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.8420109200657397;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(I, N), step: Coord(U, L), dest: Coord(I, N) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.22914375817299582;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcDst { src: Coord(E, P), dest: Coord(A, P) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.30572813210352645;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(TamMoveStepsDuringLatter { src: Coord(IA, P), step: Coord(AU, M), first_dest: Coord(IA, M), second_dest: Coord(IA, P) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.05375598529790182;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveFromHopZuo { color: Huok2, prof: Tuk2, dest: Coord(Y, T) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.6397039855712713;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(O, T), step: Coord(Y, T), dest: Coord(Y, N) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.23773185085920145;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcDst { src: Coord(A, P), dest: Coord(A, M) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.2876459471463516;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(Y, N), step: Coord(AI, N), planned_direction: Coord(O, N) }));
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
        let chooser = 0.1778084173100143;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: None };
        let chooser2 = 0.2959705568600477;
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

// Turn #257
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveFromHopZuo { color: Kok1, prof: Maun1, dest: Coord(O, T) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.7426142064688185;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(I, N), step: Coord(I, T), dest: Coord(U, N) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.7894344620488838;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcDst { src: Coord(AU, N), dest: Coord(AU, L) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.32705759405723467;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveFromHopZuo { color: Kok1, prof: Nuak1, dest: Coord(U, X) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.03219807250902662;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(Y, T), step: Coord(Y, X), planned_direction: Coord(U, X) }));
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
        let chooser = 0.939381291526512;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: Some(Coord(E, X)) };
        let chooser2 = 0.8715498376872051;
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
let pure_move = Some(NormalMove(NonTamMoveSrcDst { src: Coord(A, N), dest: Coord(E, L) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.6378595664040049;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
        let chooser: f64 = 0.1893307932822852;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(IA, L), step: Coord(AU, L), planned_direction: Coord(AU, K) }));
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
        let chooser = 0.6984733310208304;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: None };
        let chooser2 = 0.5102330781459142;
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

// Turn #265
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveFromHopZuo { color: Kok1, prof: Tuk2, dest: Coord(E, T) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.8362775874367477;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
        let chooser: f64 = 0.9832346797367578;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(A, M), step: Coord(U, M), planned_direction: Coord(E, M) }));
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
        let chooser = 0.6602076967747982;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: None };
        let chooser2 = 0.571692667765386;
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

// Turn #268
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
        let chooser: f64 = 0.9546543150236154;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
        let chooser: f64 = 0.7360249628247819;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcDst { src: Coord(E, N), dest: Coord(A, L) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.8415429883270803;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcDst { src: Coord(E, K), dest: Coord(E, L) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.15391097264637577;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
        let chooser: f64 = 0.8278495130873993;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcDst { src: Coord(I, P), dest: Coord(E, M) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.8876104528498143;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(A, X), step: Coord(I, T), dest: Coord(A, X) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.9121655406873593;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcDst { src: Coord(E, T), dest: Coord(E, N) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.2937570304910423;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(I, Z), step: Coord(U, X), planned_direction: Coord(O, Z) }));
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
        let chooser = 0.09819037435620404;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: Some(Coord(O, Z)) };
        let chooser2 = 0.8737182128819547;
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

// Turn #277
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveSrcDst { src: Coord(Y, X), dest: Coord(O, X) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.32217667379747683;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(Y, N), step: Coord(AI, N), planned_direction: Coord(Y, N) }));
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
        let chooser = 0.8747350568504478;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: Some(Coord(Y, N)) };
        let chooser2 = 0.5298247486050149;
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

// Turn #279
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(AU, L), step: Coord(AI, N), dest: Coord(AI, L) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.1100776863626044;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(O, Z), step: Coord(U, X), planned_direction: Coord(I, Z) }));
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
        let chooser = 0.5065082217397547;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: Some(Coord(E, T)) };
        let chooser2 = 0.5059619745068913;
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
let pure_move = Some(NormalMove(TamMoveNoStep { src: Coord(IA, P), first_dest: Coord(IA, M), second_dest: Coord(IA, P) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.6916694257962491;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(U, X), step: Coord(U, C), planned_direction: Coord(O, C) }));
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
        let chooser = 0.052275853433463304;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: None };
        let chooser2 = 0.349846056651579;
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

// Turn #283
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(E, N), step: Coord(A, N), planned_direction: Coord(A, Z) }));
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
        let chooser = 0.09848459097314577;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: None };
        let chooser2 = 0.04667125198481625;
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(U, N), step: Coord(U, T), dest: Coord(O, T) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.18344565904087096;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(IA, K), step: Coord(AU, K), dest: Coord(AU, L) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.04109960813730984;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(A, L), step: Coord(A, N), dest: Coord(A, T) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.7166202138993535;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcDst { src: Coord(Y, X), dest: Coord(U, X) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.6897512901735487;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(U, N), step: Coord(O, L), dest: Coord(O, N) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.27369211167731033;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveFromHopZuo { color: Kok1, prof: Nuak1, dest: Coord(A, C) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.1217993795330864;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(A, N), step: Coord(E, T), dest: Coord(I, Z) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.9262160496742269;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(O, L), step: Coord(AI, T), dest: Coord(O, X) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.5321880894724597;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcDst { src: Coord(U, T), dest: Coord(AI, C) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.8807296154450547;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(E, Z), step: Coord(E, M), planned_direction: Coord(E, C) }));
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
        let chooser = 0.9282026042508155;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: Some(Coord(E, Z)) };
        let chooser2 = 0.730551091581316;
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

// Turn #294
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveFromHopZuo { color: Huok2, prof: Kauk2, dest: Coord(O, C) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.0333095432185434;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(TamMoveNoStep { src: Coord(IA, P), first_dest: Coord(AU, P), second_dest: Coord(IA, M) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.6565474502070551;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(IA, L), step: Coord(AU, L), planned_direction: Coord(AU, K) }));
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
        let chooser = 0.6648018721372744;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: Some(Coord(AU, K)) };
        let chooser2 = 0.462375876243178;
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
let pure_move = Some(NormalMove(NonTamMoveSrcDst { src: Coord(O, X), dest: Coord(I, M) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.7522493448178984;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveFromHopZuo { color: Huok2, prof: Tuk2, dest: Coord(IA, K) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.08244873938190811;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(TamMoveStepsDuringLatter { src: Coord(IA, M), step: Coord(AU, M), first_dest: Coord(IA, P), second_dest: Coord(IA, P) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.13915784142323673;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(U, N), step: Coord(U, L), dest: Coord(O, L) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.7850676679763994;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(AU, C), step: Coord(AU, M), dest: Coord(AI, M) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.2948481410052345;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcDst { src: Coord(U, L), dest: Coord(U, N) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.6316096730528084;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveFromHopZuo { color: Kok1, prof: Kauk2, dest: Coord(A, L) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.6987780388399575;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(A, X), step: Coord(I, T), dest: Coord(O, X) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.4794595728749198;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(AI, L), step: Coord(Y, L), dest: Coord(O, N) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.860487075084276;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcDst { src: Coord(IA, K), dest: Coord(IA, Z) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.746624573148551;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(AU, M), step: Coord(IA, C), planned_direction: Coord(AU, C) }));
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
        let chooser = 0.9139825195894855;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: Some(Coord(AI, C)) };
        let chooser2 = 0.7021676075518721;
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
let pure_move = Some(NormalMove(NonTamMoveSrcDst { src: Coord(E, T), dest: Coord(A, N) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.959605153351316;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveFromHopZuo { color: Huok2, prof: Dau2, dest: Coord(I, X) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.9419110267500338;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(O, L), step: Coord(Y, L), dest: Coord(AI, N) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.14178222335618507;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(E, Z), step: Coord(E, M), planned_direction: Coord(E, C) }));
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
        let chooser = 0.8326759590304302;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: Some(Coord(E, T)) };
        let chooser2 = 0.745256646684031;
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

// Turn #312
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveFromHopZuo { color: Kok1, prof: Kaun1, dest: Coord(I, C) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.8112432205610391;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(E, L), step: Coord(E, N), planned_direction: Coord(U, N) }));
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
        let chooser = 0.025573953050895937;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: None };
        let chooser2 = 0.20364725838246367;
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

// Turn #314
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(AI, N), step: Coord(AI, T), dest: Coord(AI, Z) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.7334034018467531;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(E, T), step: Coord(I, T), dest: Coord(U, T) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.9960853205666047;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(TamMoveStepsDuringLatter { src: Coord(IA, P), step: Coord(IA, C), first_dest: Coord(AU, M), second_dest: Coord(AU, M) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.4362115245450737;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(U, T), step: Coord(U, N), planned_direction: Coord(U, T) }));
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
        let chooser = 0.42291458004105564;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: Some(Coord(U, T)) };
        let chooser2 = 0.9819827841787588;
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

// Turn #318
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(U, N), step: Coord(O, N), planned_direction: Coord(E, N) }));
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
        let chooser = 0.5530619472207076;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: Some(Coord(I, N)) };
        let chooser2 = 0.7323930002318186;
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

// Turn #319
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveSrcDst { src: Coord(U, X), dest: Coord(O, X) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.20132810704576953;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(A, X), step: Coord(I, T), dest: Coord(A, L) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.640226318151034;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(TamMoveStepsDuringLatter { src: Coord(AU, M), step: Coord(IA, X), first_dest: Coord(AU, C), second_dest: Coord(AU, C) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.5089984724895912;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
        let chooser: f64 = 0.5537025942968579;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcDst { src: Coord(U, T), dest: Coord(Y, T) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.8091078694723177;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(I, N), step: Coord(I, L), planned_direction: Coord(O, L) }));
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
        let chooser = 0.919419167618408;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: Some(Coord(U, L)) };
        let chooser2 = 0.18116309523342244;
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

// Turn #325
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveSrcDst { src: Coord(I, X), dest: Coord(U, C) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.8492776286123882;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(A, L), step: Coord(I, T), dest: Coord(A, X) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.6591339106353923;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
        let chooser: f64 = 0.8795966450832855;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(A, K), step: Coord(A, N), dest: Coord(A, K) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.7039200407227294;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(Y, T), step: Coord(Y, L), dest: Coord(O, L) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.2947521820037292;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(IA, Z), step: Coord(IA, X), planned_direction: Coord(IA, N) }));
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
        let chooser = 0.6852150009078596;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: None };
        let chooser2 = 0.43141671598731235;
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(O, L), step: Coord(Y, L), planned_direction: Coord(Y, N) }));
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
        let chooser = 0.1921112487078851;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: Some(Coord(Y, N)) };
        let chooser2 = 0.06480543458682209;
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
let pure_move = Some(NormalMove(NonTamMoveSrcDst { src: Coord(I, C), dest: Coord(A, P) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.21976529488324226;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveFromHopZuo { color: Huok2, prof: Nuak1, dest: Coord(U, P) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.6663657164667794;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(AI, Z), step: Coord(AI, T), dest: Coord(Y, N) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.9417047352214826;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(TamMoveStepsDuringLatter { src: Coord(AU, C), step: Coord(IA, C), first_dest: Coord(AU, M), second_dest: Coord(AU, C) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.24236257877179013;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveFromHopZuo { color: Huok2, prof: Tuk2, dest: Coord(IA, N) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.44887407689922276;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(AI, C), step: Coord(E, L), planned_direction: Coord(A, L) }));
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
        let chooser = 0.44341477357796766;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: Some(Coord(A, L)) };
        let chooser2 = 0.42028363008751235;
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

// Turn #338
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(TamMoveStepsDuringLatter { src: Coord(AU, C), step: Coord(AI, M), first_dest: Coord(AU, M), second_dest: Coord(AU, C) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.90661341143824;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(E, L), step: Coord(E, N), planned_direction: Coord(U, N) }));
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
        let chooser = 0.39861400800776736;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: Some(Coord(I, N)) };
        let chooser2 = 0.9165563084387048;
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

// Turn #340
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(TamMoveStepsDuringFormer { src: Coord(AU, C), step: Coord(AI, M), first_dest: Coord(AU, M), second_dest: Coord(AU, P) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.3672063839420041;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
