
#[test]
fn test12868732931022964117() {
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
let chooser = 0.406065992210641;
let mut state = initial_state().choose_by_uniform_random_variable(chooser).0;

// Turn #1
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(AU, M), step: Coord(AI, M), planned_direction: Coord(U, M) }));
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
        let chooser = 0.08146157910603569;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: None };
        let chooser2 = 0.4659320858687481;
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

// Turn #2
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(TamMoveStepsDuringLatter { src: Coord(O, Z), step: Coord(I, C), first_dest: Coord(U, X), second_dest: Coord(U, C) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.4976293286789467;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(AU, L), step: Coord(AU, K), planned_direction: Coord(AU, N) }));
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
        let chooser = 0.4492731873711462;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: Some(Coord(AU, L)) };
        let chooser2 = 0.569779940933758;
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(I, C), step: Coord(U, C), dest: Coord(O, C) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.5631390517159719;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(TamMoveStepsDuringFormer { src: Coord(U, C), step: Coord(I, M), first_dest: Coord(U, P), second_dest: Coord(U, M) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.42042018670885806;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(O, C), step: Coord(AI, C), dest: Coord(AI, M) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.8882989077482456;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(IA, T), step: Coord(IA, Z), dest: Coord(AU, Z) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.26709004571981865;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
        let chooser: f64 = 0.463359477014355;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(TamMoveNoStep { src: Coord(U, M), first_dest: Coord(U, C), second_dest: Coord(I, C) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.16306649751485724;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcDst { src: Coord(I, P), dest: Coord(U, P) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.5239309580504707;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(IA, Z), step: Coord(IA, T), dest: Coord(IA, Z) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.2172213291813887;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(TamMoveNoStep { src: Coord(I, C), first_dest: Coord(U, C), second_dest: Coord(O, M) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.4013007600713513;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(AU, L), step: Coord(AU, T), planned_direction: Coord(AU, N) }));
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
        let chooser = 0.5555454634936208;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: Some(Coord(AU, N)) };
        let chooser2 = 0.7882795311135399;
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

// Turn #14
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(TamMoveStepsDuringLatter { src: Coord(O, M), step: Coord(AI, C), first_dest: Coord(Y, M), second_dest: Coord(Y, M) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.22588958590562214;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
        let chooser: f64 = 0.42367493026021685;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
        let chooser = 0.11092590558000537;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: Some(Coord(E, M)) };
        let chooser2 = 0.9536192863581939;
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(AU, M), step: Coord(AI, M), planned_direction: Coord(I, T) }));
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
        let chooser = 0.8514215288583202;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: Some(Coord(Y, C)) };
        let chooser2 = 0.3022801875885489;
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

// Turn #18
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveSrcDst { src: Coord(E, X), dest: Coord(I, C) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.05779751576981895;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcDst { src: Coord(AU, T), dest: Coord(AI, N) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.3283065678700817;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(A, X), step: Coord(A, Z), dest: Coord(A, X) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.47678218886258084;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(TamMoveStepsDuringFormer { src: Coord(Y, M), step: Coord(Y, C), first_dest: Coord(O, X), second_dest: Coord(U, C) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.34581269751013144;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcDst { src: Coord(A, T), dest: Coord(E, Z) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.5912215217936033;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(IA, Z), step: Coord(AU, X), dest: Coord(IA, Z) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.3235742576764348;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(TamMoveNoStep { src: Coord(U, C), first_dest: Coord(O, M), second_dest: Coord(O, P) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.5304421765949133;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(AU, N), step: Coord(AI, N), planned_direction: Coord(I, C) }));
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
        let chooser = 0.9131788207434107;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: Some(Coord(O, Z)) };
        let chooser2 = 0.5656508875024575;
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
let pure_move = Some(NormalMove(TamMoveStepsDuringFormer { src: Coord(O, P), step: Coord(U, P), first_dest: Coord(O, M), second_dest: Coord(Y, P) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.1132497325337346;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(Y, C), step: Coord(AI, C), planned_direction: Coord(AU, M) }));
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
        let chooser = 0.7686682369788174;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: Some(Coord(AU, M)) };
        let chooser2 = 0.9785372971300847;
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

// Turn #28
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
        let chooser: f64 = 0.7361547640005269;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(TamMoveStepsDuringLatter { src: Coord(Y, P), step: Coord(AI, M), first_dest: Coord(Y, M), second_dest: Coord(Y, C) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.017471724726814464;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(E, Z), step: Coord(I, T), dest: Coord(E, Z) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.8369908655155751;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(IA, K), step: Coord(AU, K), planned_direction: Coord(IA, K) }));
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
        let chooser = 0.7177217264275062;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: Some(Coord(IA, K)) };
        let chooser2 = 0.7777512100986529;
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
let pure_move = Some(NormalMove(NonTamMoveSrcDst { src: Coord(U, P), dest: Coord(O, P) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.6972920831895751;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(TamMoveNoStep { src: Coord(Y, C), first_dest: Coord(O, C), second_dest: Coord(U, M) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.2704642930198965;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
        let chooser: f64 = 0.4576211928659917;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(AI, Z), step: Coord(O, Z), dest: Coord(O, N) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.3877236111776503;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(TamMoveStepsDuringFormer { src: Coord(U, M), step: Coord(I, M), first_dest: Coord(U, C), second_dest: Coord(O, X) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.575659655266286;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcDst { src: Coord(O, Z), dest: Coord(Y, X) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.7740317320811937;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(TamMoveStepsDuringFormer { src: Coord(O, X), step: Coord(O, C), first_dest: Coord(U, M), second_dest: Coord(I, P) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.30015907908928663;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(IA, Z), step: Coord(IA, X), dest: Coord(IA, Z) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.38291343021329616;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
        let chooser: f64 = 0.982670934468188;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(AU, M), step: Coord(IA, M), planned_direction: Coord(AI, M) }));
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
        let chooser = 0.4861708632361159;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: Some(Coord(AI, M)) };
        let chooser2 = 0.21759903797740132;
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(E, C), step: Coord(I, X), dest: Coord(I, Z) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.36607617768662626;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
        let chooser: f64 = 0.6889426085007943;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(A, Z), step: Coord(A, C), dest: Coord(A, Z) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.13928027706224178;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(O, N), step: Coord(I, N), dest: Coord(I, L) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.03838199355207328;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
        let chooser = 0.8577525663689715;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: None };
        let chooser2 = 0.7792527494840183;
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

// Turn #47
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveFromHopZuo { color: Kok1, prof: Kauk2, dest: Coord(E, N) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.8975110975690783;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(E, Z), step: Coord(I, T), dest: Coord(U, T) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.191049364229113;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(TamMoveStepsDuringLatter { src: Coord(I, P), step: Coord(O, P), first_dest: Coord(U, P), second_dest: Coord(O, M) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.31167642159557263;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcDst { src: Coord(I, C), dest: Coord(Y, T) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.25570196945165347;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcDst { src: Coord(Y, X), dest: Coord(AI, Z) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.4089666956493504;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(TamMoveStepsDuringLatter { src: Coord(O, M), step: Coord(U, M), first_dest: Coord(U, C), second_dest: Coord(U, C) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.7385262549066579;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
        let chooser: f64 = 0.2153094411256713;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcDst { src: Coord(O, C), dest: Coord(Y, C) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.13335620520394853;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
        let chooser = 0.585568743086203;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: None };
        let chooser2 = 0.5397344544504109;
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

// Turn #56
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(TamMoveStepsDuringFormer { src: Coord(U, C), step: Coord(I, X), first_dest: Coord(U, Z), second_dest: Coord(I, Z) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.19571452527480648;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(AU, X), step: Coord(AI, C), planned_direction: Coord(Y, M) }));
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
        let chooser = 0.37005017809815677;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: Some(Coord(Y, M)) };
        let chooser2 = 0.7707525395749993;
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
let pure_move = Some(NormalMove(TamMoveStepsDuringLatter { src: Coord(I, Z), step: Coord(I, X), first_dest: Coord(E, X), second_dest: Coord(U, Z) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.37461620030305176;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
        let chooser: f64 = 0.8291288661382872;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(TamMoveStepsDuringLatter { src: Coord(U, Z), step: Coord(Y, N), first_dest: Coord(O, T), second_dest: Coord(O, L) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.7400586365124826;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcDst { src: Coord(AU, P), dest: Coord(AU, M) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.35875507952583785;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(TamMoveStepsDuringLatter { src: Coord(O, L), step: Coord(Y, T), first_dest: Coord(O, N), second_dest: Coord(O, N) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.08609862368440868;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(AI, N), step: Coord(IA, Z), dest: Coord(AU, X) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.15470166928599083;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(Y, T), step: Coord(IA, K), dest: Coord(AU, L) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.6177061024700269;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(IA, M), step: Coord(AI, X), dest: Coord(O, T) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.8298044855036196;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(E, C), step: Coord(I, X), dest: Coord(E, X) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.5601870370990494;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(AU, Z), step: Coord(AI, X), dest: Coord(Y, Z) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.0918364784634097;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
        let chooser: f64 = 0.2853486078047952;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(TamMoveStepsDuringFormer { src: Coord(O, N), step: Coord(Y, L), first_dest: Coord(AI, L), second_dest: Coord(AU, N) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.32129304688794447;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(E, M), step: Coord(E, T), planned_direction: Coord(E, M) }));
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
        let chooser = 0.2985353062259387;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: Some(Coord(E, Z)) };
        let chooser2 = 0.09844161076771807;
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

// Turn #71
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(TamMoveStepsDuringLatter { src: Coord(AU, N), step: Coord(AI, T), first_dest: Coord(AU, T), second_dest: Coord(AI, Z) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.7743076183198764;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(Y, C), step: Coord(AI, C), planned_direction: Coord(Y, C) }));
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
        let chooser = 0.5832524788940681;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: None };
        let chooser2 = 0.5565441839014451;
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

// Turn #73
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(Y, M), step: Coord(AI, C), planned_direction: Coord(Y, M) }));
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
        let chooser = 0.7310147499073301;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: Some(Coord(O, P)) };
        let chooser2 = 0.09242818323366775;
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

// Turn #74
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(TamMoveStepsDuringLatter { src: Coord(AI, Z), step: Coord(Y, N), first_dest: Coord(Y, T), second_dest: Coord(AI, N) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.8421059867969003;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveFromHopZuo { color: Huok2, prof: Kauk2, dest: Coord(IA, M) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.21171340934696448;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(I, M), step: Coord(U, M), dest: Coord(O, M) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.5789879276011349;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(IA, L), step: Coord(AI, T), planned_direction: Coord(I, M) }));
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
        let chooser = 0.735557962310871;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: Some(Coord(O, X)) };
        let chooser2 = 0.4427724414152743;
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
let pure_move = Some(NormalMove(TamMoveStepsDuringFormer { src: Coord(AI, N), step: Coord(AU, L), first_dest: Coord(AI, N), second_dest: Coord(AI, L) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.8576112224521162;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcDst { src: Coord(E, N), dest: Coord(A, N) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.981322260772722;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcDst { src: Coord(U, T), dest: Coord(O, N) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.7069032245677044;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(TamMoveStepsDuringFormer { src: Coord(AI, L), step: Coord(Y, N), first_dest: Coord(O, L), second_dest: Coord(U, L) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.36922799077088175;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(E, Z), step: Coord(AU, Z), planned_direction: Coord(I, Z) }));
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
        let chooser = 0.6675106071025642;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: Some(Coord(O, Z)) };
        let chooser2 = 0.18057380999932704;
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(AU, X), step: Coord(AI, C), planned_direction: Coord(AU, X) }));
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
        let chooser = 0.0580621398096538;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: None };
        let chooser2 = 0.3113165852391343;
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
        let chooser: f64 = 0.4560640053102105;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(AI, M), step: Coord(AI, P), planned_direction: Coord(AU, P) }));
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
        let chooser = 0.7680106068930846;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: None };
        let chooser2 = 0.8490645624698581;
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(E, P), step: Coord(E, Z), planned_direction: Coord(E, P) }));
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
        let chooser = 0.417561054720755;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: None };
        let chooser2 = 0.3192959344951948;
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

// Turn #87
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveSrcDst { src: Coord(IA, Z), dest: Coord(IA, T) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.580576312262981;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
        let chooser: f64 = 0.7848203394145796;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveFromHopZuo { color: Kok1, prof: Kauk2, dest: Coord(Y, T) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.1277236869462549;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(TamMoveStepsDuringLatter { src: Coord(U, L), step: Coord(I, L), first_dest: Coord(I, K), second_dest: Coord(U, N) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.2400850116649519;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(O, T), step: Coord(I, L), planned_direction: Coord(Y, Z) }));
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
        let chooser = 0.9512514401923472;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: Some(Coord(Y, Z)) };
        let chooser2 = 0.21486069042936973;
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

// Turn #92
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(TamMoveStepsDuringLatter { src: Coord(U, N), step: Coord(Y, N), first_dest: Coord(O, T), second_dest: Coord(AI, L) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.40824329891519073;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(Y, X), step: Coord(U, M), planned_direction: Coord(E, M) }));
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
        let chooser = 0.03884142281531944;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: None };
        let chooser2 = 0.04877827424908521;
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

// Turn #94
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(TamMoveNoStep { src: Coord(AI, L), first_dest: Coord(AI, N), second_dest: Coord(AU, T) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.521093054930962;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(AI, M), step: Coord(O, M), planned_direction: Coord(Y, M) }));
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
        let chooser = 0.4659240787021808;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: Some(Coord(AI, M)) };
        let chooser2 = 0.023595653003251726;
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

// Turn #96
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(A, C), step: Coord(A, P), dest: Coord(I, P) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.5791039933323051;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(TamMoveNoStep { src: Coord(AU, T), first_dest: Coord(AU, N), second_dest: Coord(AI, N) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.9083522784301583;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(E, L), step: Coord(E, T), planned_direction: Coord(E, N) }));
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
        let chooser = 0.6712821088007969;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: None };
        let chooser2 = 0.23011902881750346;
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

// Turn #99
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
        let chooser: f64 = 0.11314723480535083;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
        let chooser: f64 = 0.6737393992244288;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
        let chooser: f64 = 0.1300625968246114;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(A, X), step: Coord(A, Z), dest: Coord(E, Z) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.048680934037193535;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
        let chooser: f64 = 0.0035557987419364556;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(O, N), step: Coord(Y, T), dest: Coord(Y, N) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.12307602788590266;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
        let chooser: f64 = 0.13897573551114806;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(TamMoveStepsDuringFormer { src: Coord(AI, N), step: Coord(Y, N), first_dest: Coord(O, N), second_dest: Coord(U, N) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.30575291875287325;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(Y, X), step: Coord(AI, C), planned_direction: Coord(O, Z) }));
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
        let chooser = 0.5494170701133263;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: Some(Coord(U, T)) };
        let chooser2 = 0.6150734606579086;
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
let pure_move = Some(NormalMove(NonTamMoveSrcDst { src: Coord(Y, N), dest: Coord(AI, T) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.5125823989534749;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(TamMoveStepsDuringFormer { src: Coord(U, N), step: Coord(U, T), first_dest: Coord(O, T), second_dest: Coord(O, Z) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.8993030617627287;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
        let chooser: f64 = 0.8505781536921779;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(TamMoveStepsDuringLatter { src: Coord(O, Z), step: Coord(Y, Z), first_dest: Coord(O, T), second_dest: Coord(O, T) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.4713707661404134;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(E, T), step: Coord(A, Z), dest: Coord(E, T) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.083867783774067;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(AU, X), step: Coord(AI, C), planned_direction: Coord(Y, X) }));
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
        let chooser = 0.43624030665816094;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: Some(Coord(Y, X)) };
        let chooser2 = 0.3566370869598362;
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

// Turn #114
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(TamMoveStepsDuringLatter { src: Coord(O, T), step: Coord(I, X), first_dest: Coord(U, Z), second_dest: Coord(U, X) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.5892398472455798;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(IA, T), step: Coord(AU, N), dest: Coord(AI, T) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.5241148789268189;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
        let chooser: f64 = 0.5290003793708467;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveFromHopZuo { color: Huok2, prof: Uai1, dest: Coord(U, P) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.08882307011434698;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(AU, L), step: Coord(IA, K), dest: Coord(AU, L) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.7593717839343262;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
        let chooser = 0.40746629615118746;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: None };
        let chooser2 = 0.39875523349060715;
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
let pure_move = Some(NormalMove(TamMoveStepsDuringLatter { src: Coord(U, X), step: Coord(U, M), first_dest: Coord(U, C), second_dest: Coord(U, C) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.8931452651094055;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(Y, X), step: Coord(IA, N), dest: Coord(AU, T) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.5463918742029665;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
        let chooser: f64 = 0.4469281143133431;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(AI, M), step: Coord(AI, C), planned_direction: Coord(O, Z) }));
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
        let chooser = 0.738806348026476;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: None };
        let chooser2 = 0.8900869113356497;
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

// Turn #124
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(TamMoveNoStep { src: Coord(U, C), first_dest: Coord(O, X), second_dest: Coord(U, C) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.659380342497647;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(I, L), step: Coord(E, L), planned_direction: Coord(A, L) }));
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
        let chooser = 0.1099017179087054;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: Some(Coord(A, L)) };
        let chooser2 = 0.2950926340146246;
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(E, N), step: Coord(I, T), dest: Coord(I, Z) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.8867875216779577;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveFromHopZuo { color: Huok2, prof: Maun1, dest: Coord(I, C) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.1814779849900071;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(E, L), step: Coord(A, L), planned_direction: Coord(Y, L) }));
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
        let chooser = 0.3708095470132563;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: Some(Coord(I, L)) };
        let chooser2 = 0.755611489122478;
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
let pure_move = Some(NormalMove(NonTamMoveSrcDst { src: Coord(U, T), dest: Coord(O, Z) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.3120511707775552;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(I, Z), step: Coord(I, X), dest: Coord(U, Z) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.6424905856426699;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(Y, Z), step: Coord(U, C), planned_direction: Coord(E, Z) }));
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
        let chooser = 0.6364887372211341;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: None };
        let chooser2 = 0.8830285419265352;
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

// Turn #132
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveSrcDst { src: Coord(O, M), dest: Coord(AI, M) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.7575692881836528;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(AI, T), step: Coord(AU, Z), dest: Coord(AU, X) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.9379221464943582;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveFromHopZuo { color: Kok1, prof: Gua2, dest: Coord(AU, P) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.39339160524384575;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(TamMoveStepsDuringLatter { src: Coord(U, C), step: Coord(I, C), first_dest: Coord(U, X), second_dest: Coord(E, M) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.21147767822310448;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(E, P), step: Coord(U, P), dest: Coord(O, P) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.4816421564652179;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(U, P), step: Coord(U, M), dest: Coord(I, P) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.18279256432875346;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveFromHopZuo { color: Kok1, prof: Dau2, dest: Coord(Y, L) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.5939750326083754;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
        let chooser: f64 = 0.5283040698453942;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(TamMoveStepsDuringLatter { src: Coord(E, M), step: Coord(E, X), first_dest: Coord(E, C), second_dest: Coord(I, Z) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.05180928321568301;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(I, P), step: Coord(U, M), dest: Coord(I, P) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.9526845313485489;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(Y, L), step: Coord(O, K), dest: Coord(Y, L) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.009281859115957913;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(TamMoveStepsDuringFormer { src: Coord(I, Z), step: Coord(I, T), first_dest: Coord(U, N), second_dest: Coord(O, L) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.07688556378245737;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(O, K), step: Coord(O, L), dest: Coord(U, L) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.9451750424200837;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(TamMoveStepsDuringFormer { src: Coord(O, L), step: Coord(Y, L), first_dest: Coord(AI, N), second_dest: Coord(AI, T) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.48476524600336635;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcDst { src: Coord(AU, P), dest: Coord(AI, P) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.08250351457813421;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(TamMoveStepsDuringFormer { src: Coord(AI, T), step: Coord(Y, T), first_dest: Coord(Y, N), second_dest: Coord(O, L) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.4906167551634636;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveFromHopZuo { color: Huok2, prof: Kauk2, dest: Coord(Y, P) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.621913151306066;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(TamMoveStepsDuringFormer { src: Coord(O, L), step: Coord(U, L), first_dest: Coord(O, K), second_dest: Coord(U, K) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.9680754325173879;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(U, L), step: Coord(Y, L), dest: Coord(AI, L) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.21210506425655984;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(TamMoveStepsDuringFormer { src: Coord(U, K), step: Coord(I, L), first_dest: Coord(U, K), second_dest: Coord(O, K) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.7328325701200875;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveFromHopZuo { color: Kok1, prof: Kauk2, dest: Coord(Y, M) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.30396964244722635;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
        let chooser: f64 = 0.9243411571435178;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(I, L), step: Coord(Y, L), planned_direction: Coord(AI, K) }));
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
        let chooser = 0.7049986852446785;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: Some(Coord(AI, K)) };
        let chooser2 = 0.7509168803235032;
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

// Turn #155
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
        let chooser: f64 = 0.09564845052954285;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(U, Z), step: Coord(I, X), dest: Coord(U, Z) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.7049930141629039;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(O, X), step: Coord(I, X), dest: Coord(O, X) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.18702914137857973;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(E, Z), step: Coord(I, X), dest: Coord(U, X) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.8626716598119436;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(TamMoveStepsDuringFormer { src: Coord(O, K), step: Coord(Y, K), first_dest: Coord(O, L), second_dest: Coord(O, K) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.12433869557129262;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(E, X), step: Coord(E, T), planned_direction: Coord(E, P) }));
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
        let chooser = 0.5971227939649053;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: Some(Coord(E, Z)) };
        let chooser2 = 0.7337438110785979;
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

// Turn #161
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveSrcDst { src: Coord(IA, N), dest: Coord(IA, Z) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.28989574915403393;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(E, Z), step: Coord(U, Z), planned_direction: Coord(Y, Z) }));
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
        let chooser = 0.3198303651419382;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: Some(Coord(Y, Z)) };
        let chooser2 = 0.5260807722701333;
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

// Turn #163
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(I, C), step: Coord(Y, T), planned_direction: Coord(AU, L) }));
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
        let chooser = 0.7610625442310501;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: Some(Coord(AU, L)) };
        let chooser2 = 0.7809019419936608;
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

// Turn #164
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveFromHopZuo { color: Huok2, prof: Maun1, dest: Coord(E, L) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.2107809296412454;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveFromHopZuo { color: Huok2, prof: Dau2, dest: Coord(E, X) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.18077306872558074;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(U, X), step: Coord(O, X), dest: Coord(O, Z) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.9737038747361131;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(U, T), step: Coord(O, Z), planned_direction: Coord(E, M) }));
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
        let chooser = 0.03143138117659883;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: None };
        let chooser2 = 0.9402654090058362;
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(A, P), step: Coord(A, M), planned_direction: Coord(I, M) }));
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
        let chooser = 0.8291669286936344;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: None };
        let chooser2 = 0.5773276313170976;
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

// Turn #169
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(U, T), step: Coord(I, N), planned_direction: Coord(U, T) }));
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
        let chooser = 0.5215650812829081;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: None };
        let chooser2 = 0.07143331639577843;
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(O, Z), step: Coord(U, Z), dest: Coord(O, T) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.7547055541635589;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(O, X), step: Coord(AI, X), dest: Coord(AI, T) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.8880423234124653;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(O, P), step: Coord(Y, P), dest: Coord(O, P) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.7118862346073156;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(AU, Z), step: Coord(AI, T), dest: Coord(Y, N) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.4822333794408977;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(O, P), step: Coord(O, T), planned_direction: Coord(O, M) }));
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
        let chooser = 0.8433837052604573;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: Some(Coord(O, X)) };
        let chooser2 = 0.41536364824703376;
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

// Turn #175
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(AI, T), step: Coord(AI, X), dest: Coord(AI, M) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.31500802704136766;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcDst { src: Coord(O, T), dest: Coord(Y, N) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.3903061446611682;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveFromHopZuo { color: Kok1, prof: Kauk2, dest: Coord(O, M) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.5861522160589516;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveFromHopZuo { color: Kok1, prof: Uai1, dest: Coord(Y, X) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.2598104587944403;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(I, P), step: Coord(U, M), dest: Coord(I, C) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.44362531182604925;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(Y, Z), step: Coord(IA, Z), planned_direction: Coord(Y, Z) }));
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
        let chooser = 0.6498797574653293;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: Some(Coord(AI, Z)) };
        let chooser2 = 0.8452011796210639;
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

// Turn #181
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(U, T), step: Coord(E, X), planned_direction: Coord(E, Z) }));
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
        let chooser = 0.6056962081511192;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: Some(Coord(E, T)) };
        let chooser2 = 0.4953069507290432;
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

// Turn #182
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(AI, Z), step: Coord(U, Z), planned_direction: Coord(U, X) }));
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
        let chooser = 0.5568455815401631;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: None };
        let chooser2 = 0.054507683822408004;
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
        let chooser: f64 = 0.685639618969926;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(AI, Z), step: Coord(AI, X), planned_direction: Coord(AI, Z) }));
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
        let chooser = 0.30822098576691515;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: None };
        let chooser2 = 0.34953375393462405;
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(AI, N), step: Coord(Y, L), planned_direction: Coord(O, N) }));
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
        let chooser = 0.08612179602271774;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: None };
        let chooser2 = 0.47779500539080155;
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
let pure_move = Some(NormalMove(TamMoveStepsDuringLatter { src: Coord(O, K), step: Coord(I, N), first_dest: Coord(U, L), second_dest: Coord(I, L) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.41088389553640914;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(AU, X), step: Coord(IA, C), dest: Coord(AU, C) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.11943085221907579;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(E, K), step: Coord(E, T), dest: Coord(A, T) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.05890703733690816;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveFromHopZuo { color: Kok1, prof: Kauk2, dest: Coord(U, C) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.15093782358999352;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(TamMoveStepsDuringFormer { src: Coord(I, L), step: Coord(I, N), first_dest: Coord(U, N), second_dest: Coord(O, N) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.5453758728487265;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(I, C), step: Coord(U, C), dest: Coord(U, X) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.12010252650390496;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
        let chooser = 0.988678288101736;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: Some(Coord(I, K)) };
        let chooser2 = 0.058393864585787214;
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
        let chooser: f64 = 0.8540381338994572;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcDst { src: Coord(AI, Z), dest: Coord(IA, Z) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.6456435709053149;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(TamMoveStepsDuringLatter { src: Coord(O, N), step: Coord(U, Z), first_dest: Coord(U, T), second_dest: Coord(O, Z) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.7305838938604899;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveFromHopZuo { color: Kok1, prof: Kaun1, dest: Coord(IA, P) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.3640353511845603;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(AU, M), step: Coord(AI, M), dest: Coord(Y, M) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.8277675836174683;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(Y, L), step: Coord(AI, N), planned_direction: Coord(Y, T) }));
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
        let chooser = 0.19774478346794178;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: None };
        let chooser2 = 0.6210784238756467;
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

// Turn #199
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveFromHopZuo { color: Kok1, prof: Kauk2, dest: Coord(AU, X) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.5572591917144871;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(I, K), step: Coord(Y, K), planned_direction: Coord(O, K) }));
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
        let chooser = 0.6341648801832034;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: Some(Coord(U, K)) };
        let chooser2 = 0.10461179184339586;
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(E, T), step: Coord(E, X), planned_direction: Coord(E, P) }));
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
        let chooser = 0.6416928230996936;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: Some(Coord(E, M)) };
        let chooser2 = 0.8872368255210835;
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

// Turn #202
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
        let chooser: f64 = 0.6292834946587824;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(Y, M), step: Coord(Y, C), planned_direction: Coord(Y, P) }));
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
        let chooser = 0.92810129164566;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: Some(Coord(Y, P)) };
        let chooser2 = 0.45341315597757226;
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

// Turn #204
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(TamMoveStepsDuringFormer { src: Coord(O, Z), step: Coord(U, X), first_dest: Coord(O, Z), second_dest: Coord(Y, Z) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.5322587679862096;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveFromHopZuo { color: Huok2, prof: Kauk2, dest: Coord(I, Z) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.8558872631354867;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
        let chooser: f64 = 0.504998548590242;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
        let chooser = 0.42993977152411145;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: Some(Coord(O, N)) };
        let chooser2 = 0.001991772184646301;
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
let pure_move = Some(NormalMove(NonTamMoveSrcDst { src: Coord(O, P), dest: Coord(O, M) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.43535318723510974;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(TamMoveStepsDuringLatter { src: Coord(Y, Z), step: Coord(AU, X), first_dest: Coord(AI, Z), second_dest: Coord(AU, Z) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.9908067317506957;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
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
let pure_move = Some(NormalMove(NonTamMoveFromHopZuo { color: Kok1, prof: Kauk2, dest: Coord(E, P) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.5567730917215031;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(E, X), step: Coord(A, Z), dest: Coord(E, X) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.41592840058002745;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(TamMoveStepsDuringLatter { src: Coord(AU, Z), step: Coord(Y, T), first_dest: Coord(AI, T), second_dest: Coord(AI, T) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.36400548783403675;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(AU, C), step: Coord(AI, X), dest: Coord(AI, Z) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.038675455972523864;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(U, K), step: Coord(A, K), planned_direction: Coord(Y, K) }));
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
        let chooser = 0.9948156458814396;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: None };
        let chooser2 = 0.7188077442630645;
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(E, M), step: Coord(E, X), planned_direction: Coord(E, C) }));
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
        let chooser = 0.12590767011589077;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: Some(Coord(E, C)) };
        let chooser2 = 0.6601495641705561;
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(A, K), step: Coord(A, L), dest: Coord(A, K) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.24414856608235203;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(TamMoveStepsDuringLatter { src: Coord(AI, T), step: Coord(AU, T), first_dest: Coord(AI, Z), second_dest: Coord(AI, Z) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.3638767745303003;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(A, K), step: Coord(U, K), dest: Coord(U, L) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.772192345053053;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(TamMoveStepsDuringFormer { src: Coord(AI, Z), step: Coord(AI, X), first_dest: Coord(AI, Z), second_dest: Coord(Y, Z) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.9269053849573801;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcDst { src: Coord(O, M), dest: Coord(U, M) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.7300512169993627;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(U, X), step: Coord(O, X), dest: Coord(O, C) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.7548483024343797;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveFromHopZuo { color: Huok2, prof: Kauk2, dest: Coord(I, L) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.17132320881092944;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(AI, N), step: Coord(Y, T), planned_direction: Coord(U, L) }));
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
        let chooser = 0.8132256111685902;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: Some(Coord(U, L)) };
        let chooser2 = 0.08868679342525598;
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

// Turn #224
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(TamMoveStepsDuringLatter { src: Coord(Y, Z), step: Coord(AU, X), first_dest: Coord(AI, Z), second_dest: Coord(AU, Z) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.7197405369762212;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveFromHopZuo { color: Huok2, prof: Kua2, dest: Coord(AU, M) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.339810146528285;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(E, L), step: Coord(U, T), planned_direction: Coord(E, X) }));
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
        let chooser = 0.730977299970642;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: None };
        let chooser2 = 0.8856076485009279;
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

// Turn #227
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(AU, T), step: Coord(Y, L), dest: Coord(AI, N) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.8150630856169827;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcDst { src: Coord(Y, C), dest: Coord(AI, C) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.7996351358705203;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(AU, X), step: Coord(AU, Z), dest: Coord(AU, T) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.030650731765955008;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
        let chooser: f64 = 0.06717961952554097;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(TamMoveNoStep { src: Coord(AU, Z), first_dest: Coord(AU, X), second_dest: Coord(IA, X) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.07924517362230188;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(Y, N), step: Coord(Y, T), dest: Coord(AI, T) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.8557456343973374;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(TamMoveStepsDuringLatter { src: Coord(IA, X), step: Coord(AU, C), first_dest: Coord(AU, X), second_dest: Coord(AU, X) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.13592497068259235;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(U, Z), step: Coord(U, T), dest: Coord(O, Z) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.7966243409138772;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcDst { src: Coord(Y, P), dest: Coord(O, P) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.6310975694835756;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(TamMoveStepsDuringLatter { src: Coord(AU, X), step: Coord(Y, X), first_dest: Coord(AI, Z), second_dest: Coord(AI, Z) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.02870685693939934;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
        let chooser: f64 = 0.5363489433907491;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(TamMoveNoStep { src: Coord(AI, Z), first_dest: Coord(AU, Z), second_dest: Coord(AU, X) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.27105237503263113;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(AU, N), step: Coord(AI, N), dest: Coord(AI, L) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.6713707783141211;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
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
let pure_move = Some(NormalMove(TamMoveStepsDuringLatter { src: Coord(AU, X), step: Coord(AI, T), first_dest: Coord(AU, Z), second_dest: Coord(Y, N) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.49245489956354926;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
        let chooser: f64 = 0.7744882408738895;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(IA, Z), step: Coord(IA, T), planned_direction: Coord(IA, C) }));
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
        let chooser = 0.6635600220604784;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: None };
        let chooser2 = 0.596965037904764;
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

// Turn #243
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(AU, M), step: Coord(AI, M), planned_direction: Coord(AU, M) }));
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
        let chooser = 0.3691458881035301;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: Some(Coord(AU, M)) };
        let chooser2 = 0.4951672500353065;
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

// Turn #244
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(TamMoveStepsDuringFormer { src: Coord(Y, N), step: Coord(Y, T), first_dest: Coord(O, N), second_dest: Coord(O, T) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.9492362724280683;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
        let chooser: f64 = 0.7697220617528492;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(TamMoveStepsDuringLatter { src: Coord(O, T), step: Coord(I, Z), first_dest: Coord(U, Z), second_dest: Coord(U, X) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.6710302416624878;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveFromHopZuo { color: Huok2, prof: Kauk2, dest: Coord(IA, N) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.3205524051233991;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(O, Z), step: Coord(U, T), dest: Coord(I, T) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.08769534641438215;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(AU, M), step: Coord(AI, M), dest: Coord(AI, C) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.0332531404233235;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(TamMoveStepsDuringLatter { src: Coord(U, X), step: Coord(I, T), first_dest: Coord(U, Z), second_dest: Coord(U, N) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.38030986584563053;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveFromHopZuo { color: Kok1, prof: Nuak1, dest: Coord(A, K) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.858166445286841;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(I, T), step: Coord(U, N), dest: Coord(U, L) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.879584915410516;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcDst { src: Coord(U, C), dest: Coord(I, C) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.8932764533348002;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveFromHopZuo { color: Kok1, prof: Dau2, dest: Coord(A, C) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.4904284413638266;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(AI, C), step: Coord(AU, C), planned_direction: Coord(Y, C) }));
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
        let chooser = 0.5896395594717334;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: None };
        let chooser2 = 0.5775840348835095;
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
let pure_move = Some(NormalMove(NonTamMoveSrcDst { src: Coord(AI, T), dest: Coord(AU, T) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.45431250401833834;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcDst { src: Coord(Y, T), dest: Coord(Y, N) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.8112308828227961;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveFromHopZuo { color: Kok1, prof: Kauk2, dest: Coord(U, Z) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.017480142228219142;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(IA, C), step: Coord(AI, C), dest: Coord(IA, P) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.6275351271360081;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(TamMoveStepsDuringFormer { src: Coord(U, N), step: Coord(U, L), first_dest: Coord(U, N), second_dest: Coord(O, N) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.8323369224483295;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveFromHopZuo { color: Kok1, prof: Kaun1, dest: Coord(I, T) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.4573629847489824;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(A, P), step: Coord(A, M), planned_direction: Coord(I, M) }));
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
        let chooser = 0.9162927968481673;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: Some(Coord(I, M)) };
        let chooser2 = 0.6905275975542412;
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

// Turn #263
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
        let chooser: f64 = 0.8590822978382704;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(TamMoveStepsDuringLatter { src: Coord(O, N), step: Coord(U, L), first_dest: Coord(U, N), second_dest: Coord(U, N) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.36436255490257174;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(O, C), step: Coord(U, X), dest: Coord(U, Z) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.7345502890637226;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(AU, T), step: Coord(IA, Z), dest: Coord(IA, T) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.19145079494127604;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveFromHopZuo { color: Kok1, prof: Kauk2, dest: Coord(O, Z) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.21150285242256694;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
        let chooser: f64 = 0.16473199168125596;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(TamMoveStepsDuringLatter { src: Coord(U, N), step: Coord(Y, L), first_dest: Coord(O, L), second_dest: Coord(O, N) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.9507238936132678;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(IA, Z), step: Coord(IA, M), planned_direction: Coord(AI, M) }));
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
        let chooser = 0.34473761506660083;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: Some(Coord(AU, M)) };
        let chooser2 = 0.312201759006688;
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

// Turn #271
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(U, Z), step: Coord(I, T), dest: Coord(U, Z) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.999330385530108;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(O, T), step: Coord(O, N), dest: Coord(O, T) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.5404811081836424;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(AU, C), step: Coord(AI, M), dest: Coord(Y, M) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.7285349241242312;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(O, X), step: Coord(Y, X), dest: Coord(O, X) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.8207165824593706;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(TamMoveNoStep { src: Coord(O, N), first_dest: Coord(Y, T), second_dest: Coord(Y, Z) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.21086555079338454;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(U, K), step: Coord(U, L), planned_direction: Coord(U, N) }));
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
        let chooser = 0.5818675944439743;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: None };
        let chooser2 = 0.7832252496586614;
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(Y, M), step: Coord(AI, P), dest: Coord(AU, M) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.0765160957544756;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(O, X), step: Coord(U, X), dest: Coord(U, Z) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.46784829305958775;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveFromHopZuo { color: Huok2, prof: Gua2, dest: Coord(O, M) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.7739850071660419;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveFromHopZuo { color: Huok2, prof: Uai1, dest: Coord(E, K) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.7086090507240872;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(AI, C), step: Coord(I, C), planned_direction: Coord(AU, C) }));
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
        let chooser = 0.21307144835789127;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: None };
        let chooser2 = 0.8315224103582797;
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(I, M), step: Coord(I, C), planned_direction: Coord(E, C) }));
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
        let chooser = 0.4924057920685504;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: Some(Coord(E, C)) };
        let chooser2 = 0.1630536758161848;
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(AI, N), step: Coord(Y, L), dest: Coord(O, N) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.11616564184131128;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
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
let pure_move = Some(NormalMove(NonTamMoveFromHopZuo { color: Huok2, prof: Gua2, dest: Coord(AI, X) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.4731035440345329;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(AU, M), step: Coord(AI, C), dest: Coord(AU, C) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.9594091752500723;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(AI, P), step: Coord(O, P), planned_direction: Coord(AU, P) }));
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
        let chooser = 0.9112532283836237;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: Some(Coord(AU, P)) };
        let chooser2 = 0.0646688569444479;
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(U, X), step: Coord(I, Z), dest: Coord(E, X) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.3212813710540451;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveFromHopZuo { color: Huok2, prof: Kua2, dest: Coord(AI, K) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.7908189761904939;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(O, P), step: Coord(O, M), planned_direction: Coord(O, P) }));
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
        let chooser = 0.26050413696017594;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: Some(Coord(O, P)) };
        let chooser2 = 0.36646068740142335;
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

// Turn #290
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(AU, P), step: Coord(O, P), planned_direction: Coord(Y, P) }));
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
        let chooser = 0.12726197029663722;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: Some(Coord(Y, P)) };
        let chooser2 = 0.1512359603863832;
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

// Turn #291
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(TamMoveStepsDuringFormer { src: Coord(Y, Z), step: Coord(Y, X), first_dest: Coord(Y, C), second_dest: Coord(Y, M) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.026669782371549267;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(U, Z), step: Coord(O, T), dest: Coord(O, N) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.11378077018987465;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(O, P), step: Coord(I, C), planned_direction: Coord(O, P) }));
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
        let chooser = 0.00906504791857543;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: None };
        let chooser2 = 0.9200555210675903;
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(Y, P), step: Coord(AI, M), planned_direction: Coord(I, T) }));
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
        let chooser = 0.9878739383570435;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: Some(Coord(Y, C)) };
        let chooser2 = 0.7024087507044956;
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(O, P), step: Coord(Y, M), planned_direction: Coord(O, C) }));
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
        let chooser = 0.33485692879303774;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: None };
        let chooser2 = 0.16207051794951255;
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

// Turn #296
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(TamMoveNoStep { src: Coord(Y, M), first_dest: Coord(O, C), second_dest: Coord(U, C) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.43476038066832867;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(O, P), step: Coord(O, M), planned_direction: Coord(O, X) }));
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
        let chooser = 0.06974433462167484;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: Some(Coord(O, C)) };
        let chooser2 = 0.8637308683872023;
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(U, M), step: Coord(O, C), planned_direction: Coord(O, Z) }));
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
        let chooser = 0.1708844103000896;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: None };
        let chooser2 = 0.9690680215874852;
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

// Turn #299
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(O, Z), step: Coord(I, Z), dest: Coord(E, Z) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.10387990508802569;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(TamMoveStepsDuringLatter { src: Coord(U, C), step: Coord(O, C), first_dest: Coord(U, X), second_dest: Coord(U, X) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.0014510583471986083;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(AU, C), step: Coord(AI, X), dest: Coord(AU, C) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.5094495060958987;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(O, N), step: Coord(U, T), dest: Coord(U, N) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.502745959294436;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(AU, C), step: Coord(AU, X), dest: Coord(AI, Z) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.07114377083221335;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(TamMoveStepsDuringLatter { src: Coord(U, X), step: Coord(O, T), first_dest: Coord(U, Z), second_dest: Coord(U, Z) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.16904825426040193;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
        let chooser: f64 = 0.8719831731629261;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(U, N), step: Coord(O, T), dest: Coord(O, N) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.06570588657896315;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveFromHopZuo { color: Kok1, prof: Dau2, dest: Coord(O, X) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.28285282125978173;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(TamMoveStepsDuringLatter { src: Coord(U, Z), step: Coord(O, X), first_dest: Coord(O, Z), second_dest: Coord(Y, Z) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.6853657055388535;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(AU, C), step: Coord(AI, M), dest: Coord(Y, C) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.48999966824057595;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(E, K), step: Coord(I, L), dest: Coord(E, K) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.06689576632392669;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveFromHopZuo { color: Kok1, prof: Gua2, dest: Coord(E, N) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.4056868312638511;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(E, K), step: Coord(I, L), dest: Coord(E, N) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.2065561820110119;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
        let chooser: f64 = 0.396085607660951;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveFromHopZuo { color: Kok1, prof: Gua2, dest: Coord(AI, Z) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.2875994616076165;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(AI, M), step: Coord(O, M), dest: Coord(AI, M) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.512624971642015;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(TamMoveNoStep { src: Coord(Y, Z), first_dest: Coord(AI, T), second_dest: Coord(Y, T) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.9777188709195515;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
        let chooser = 0.11368822543556423;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: Some(Coord(O, N)) };
        let chooser2 = 0.9530768392975687;
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(I, N), step: Coord(E, N), dest: Coord(I, N) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.9072510179769845;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(O, M), step: Coord(U, M), planned_direction: Coord(U, Z) }));
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
        let chooser = 0.010457561166499985;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: None };
        let chooser2 = 0.923495210668433;
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

// Turn #320
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(AI, Z), step: Coord(AU, X), planned_direction: Coord(AU, C) }));
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
        let chooser = 0.3336203553569428;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: Some(Coord(AU, C)) };
        let chooser2 = 0.24775748607964154;
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(O, M), step: Coord(AI, M), planned_direction: Coord(AU, M) }));
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
        let chooser = 0.06058045913058485;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: Some(Coord(AU, M)) };
        let chooser2 = 0.8514108307732966;
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

// Turn #322
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(TamMoveStepsDuringLatter { src: Coord(Y, T), step: Coord(O, T), first_dest: Coord(Y, Z), second_dest: Coord(U, Z) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.8550336259356602;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(AU, K), step: Coord(AU, L), planned_direction: Coord(AU, K) }));
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
        let chooser = 0.6368457582335472;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: None };
        let chooser2 = 0.2900308499306714;
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

// Turn #324
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(U, N), step: Coord(O, N), dest: Coord(Y, L) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.3369932005147197;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
        let chooser: f64 = 0.06442642378988306;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveFromHopZuo { color: Huok2, prof: Kauk2, dest: Coord(AI, Z) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.31946405036642456;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveFromHopZuo { color: Huok2, prof: Gua2, dest: Coord(U, C) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.638355924547889;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(TamMoveStepsDuringFormer { src: Coord(U, Z), step: Coord(U, T), first_dest: Coord(O, Z), second_dest: Coord(Y, X) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.01585031959802352;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(O, C), step: Coord(I, Z), planned_direction: Coord(I, X) }));
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
        let chooser = 0.9071329469572692;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: Some(Coord(I, X)) };
        let chooser2 = 0.7390811498743317;
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
let pure_move = Some(NormalMove(TamMoveStepsDuringFormer { src: Coord(Y, X), step: Coord(Y, C), first_dest: Coord(Y, M), second_dest: Coord(AI, P) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.5908166174496456;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
        let chooser: f64 = 0.5173036196061254;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
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
let pure_move = Some(NormalMove(TamMoveStepsDuringLatter { src: Coord(AI, P), step: Coord(AI, M), first_dest: Coord(Y, M), second_dest: Coord(Y, P) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.19964976306127302;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(O, X), step: Coord(U, C), dest: Coord(O, X) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.9247018763525778;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
        let chooser: f64 = 0.63852883055755;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(I, X), step: Coord(I, C), planned_direction: Coord(U, X) }));
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
        let chooser = 0.6708712213147648;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: None };
        let chooser2 = 0.9061714271553789;
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

// Turn #336
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(Y, L), step: Coord(AI, K), dest: Coord(Y, L) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.38465977331055723;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(I, X), step: Coord(I, C), planned_direction: Coord(AI, N) }));
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
        let chooser = 0.8969056128475599;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: Some(Coord(O, Z)) };
        let chooser2 = 0.8393404820962554;
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(AI, K), step: Coord(AU, K), planned_direction: Coord(AI, K) }));
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
        let chooser = 0.9092748502314227;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: None };
        let chooser2 = 0.18523820503425292;
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

// Turn #339
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(TamMoveStepsDuringFormer { src: Coord(Y, P), step: Coord(AI, M), first_dest: Coord(Y, M), second_dest: Coord(O, M) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.6916178616560706;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(U, M), step: Coord(O, M), planned_direction: Coord(O, X) }));
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
        let chooser = 0.7630103634705268;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: Some(Coord(O, X)) };
        let chooser2 = 0.5416215012970085;
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

}
