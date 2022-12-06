
#[test]
fn test3551368881874913695() {
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
let chooser = 0.35416760937583813;
let mut state = initial_state().choose_by_uniform_random_variable(chooser).0;

// Turn #1
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
        let chooser: f64 = 0.3852482477297696;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(TamMoveNoStep { src: Coord(O, Z), first_dest: Coord(O, T), second_dest: Coord(Y, N) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.8453532158187032;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(AU, T), step: Coord(AI, N), planned_direction: Coord(I, C) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::InfAfterStep(m) => {
        let chooser = 0.32252890225721187;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: None };
        let chooser2 = 0.13111966139673026;
        let hnr_state = apply_after_half_acceptance(&ext_state, aha_move, config)
            .unwrap()
            .choose_by_uniform_random_variable(chooser2)
            .0;
        
        hnr_state
    },
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
let pure_move = Some(NormalMove(TamMoveNoStep { src: Coord(Y, N), first_dest: Coord(O, T), second_dest: Coord(Y, T) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.6528927318500347;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
        let chooser: f64 = 0.4781422032205238;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
        let chooser: f64 = 0.025359524517724608;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(TamMoveNoStep { src: Coord(Y, T), first_dest: Coord(O, T), second_dest: Coord(O, Z) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.17654187244050545;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
        let chooser: f64 = 0.7585827724119204;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(AU, M), step: Coord(AU, X), planned_direction: Coord(AU, C) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::InfAfterStep(m) => {
        let chooser = 0.7929719667922858;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: Some(Coord(AU, C)) };
        let chooser2 = 0.07677921234280327;
        let hnr_state = apply_after_half_acceptance(&ext_state, aha_move, config)
            .unwrap()
            .choose_by_uniform_random_variable(chooser2)
            .0;
        
        hnr_state
    },
    _ => { panic!("Expected PureMove::InfAfterStep") }
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
let pure_move = Some(NormalMove(TamMoveStepsDuringLatter { src: Coord(O, Z), step: Coord(O, T), first_dest: Coord(Y, T), second_dest: Coord(U, T) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.13249834810739491;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(AI, Z), step: Coord(I, Z), dest: Coord(I, X) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.2547571740916428;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(TamMoveStepsDuringLatter { src: Coord(U, T), step: Coord(I, Z), first_dest: Coord(U, Z), second_dest: Coord(E, Z) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.5080478571291075;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
        let chooser: f64 = 0.09750425011487451;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(E, X), step: Coord(I, Z), planned_direction: Coord(Y, M) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::InfAfterStep(m) => {
        let chooser = 0.03825946443701167;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: None };
        let chooser2 = 0.43272185917339223;
        let hnr_state = apply_after_half_acceptance(&ext_state, aha_move, config)
            .unwrap()
            .choose_by_uniform_random_variable(chooser2)
            .0;
        
        hnr_state
    },
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
        let chooser: f64 = 0.34901168382027126;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(I, T), step: Coord(I, N), dest: Coord(U, N) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.385668184962912;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
    _ => { panic!("expected PureMove::NormalMove") }
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
        let chooser: f64 = 0.24234087088492373;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(TamMoveStepsDuringFormer { src: Coord(E, Z), step: Coord(I, X), first_dest: Coord(U, C), second_dest: Coord(O, M) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.11434193217884536;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
        let chooser = 0.24882520066034475;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: Some(Coord(IA, P)) };
        let chooser2 = 0.27771367060782803;
        let hnr_state = apply_after_half_acceptance(&ext_state, aha_move, config)
            .unwrap()
            .choose_by_uniform_random_variable(chooser2)
            .0;
        
        hnr_state
    },
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
let pure_move = Some(NormalMove(TamMoveStepsDuringLatter { src: Coord(O, M), step: Coord(I, M), first_dest: Coord(U, C), second_dest: Coord(U, M) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.7872212425025895;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(IA, L), step: Coord(AI, T), dest: Coord(IA, L) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.8623416192979512;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(TamMoveStepsDuringFormer { src: Coord(U, M), step: Coord(I, P), first_dest: Coord(U, M), second_dest: Coord(O, C) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.17536163423568163;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(AU, L), step: Coord(Y, L), planned_direction: Coord(I, L) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::InfAfterStep(m) => {
        let chooser = 0.06124775438912655;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: None };
        let chooser2 = 0.33720750797266597;
        let hnr_state = apply_after_half_acceptance(&ext_state, aha_move, config)
            .unwrap()
            .choose_by_uniform_random_variable(chooser2)
            .0;
        
        hnr_state
    },
    _ => { panic!("Expected PureMove::InfAfterStep") }
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
let pure_move = Some(NormalMove(TamMoveStepsDuringLatter { src: Coord(O, C), step: Coord(I, X), first_dest: Coord(U, C), second_dest: Coord(E, C) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.22787225706522718;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
        let chooser: f64 = 0.4703324234661356;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(TamMoveStepsDuringFormer { src: Coord(E, C), step: Coord(I, C), first_dest: Coord(U, M), second_dest: Coord(O, M) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.5575805942654171;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(AU, C), step: Coord(AI, C), planned_direction: Coord(O, P) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::InfAfterStep(m) => {
        let chooser = 0.5488944390788897;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: Some(Coord(Y, M)) };
        let chooser2 = 0.416935844559877;
        let hnr_state = apply_after_half_acceptance(&ext_state, aha_move, config)
            .unwrap()
            .choose_by_uniform_random_variable(chooser2)
            .0;
        
        hnr_state
    },
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
let pure_move = Some(NormalMove(TamMoveStepsDuringLatter { src: Coord(O, M), step: Coord(I, X), first_dest: Coord(U, C), second_dest: Coord(E, Z) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.3398986811652299;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
        let chooser: f64 = 0.6837191655827362;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(A, T), step: Coord(E, N), dest: Coord(A, T) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.8077263797317416;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(Y, M), step: Coord(Y, L), planned_direction: Coord(Y, K) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::InfAfterStep(m) => {
        let chooser = 0.587714883793764;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: None };
        let chooser2 = 0.45754593838916013;
        let hnr_state = apply_after_half_acceptance(&ext_state, aha_move, config)
            .unwrap()
            .choose_by_uniform_random_variable(chooser2)
            .0;
        
        hnr_state
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(A, T), step: Coord(E, N), dest: Coord(I, T) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.24864421364323652;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(Y, M), step: Coord(AI, M), planned_direction: Coord(I, M) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::InfAfterStep(m) => {
        let chooser = 0.35607418211385444;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: Some(Coord(O, M)) };
        let chooser2 = 0.2017160136578725;
        let hnr_state = apply_after_half_acceptance(&ext_state, aha_move, config)
            .unwrap()
            .choose_by_uniform_random_variable(chooser2)
            .0;
        
        hnr_state
    },
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
let pure_move = Some(NormalMove(TamMoveStepsDuringLatter { src: Coord(E, Z), step: Coord(A, N), first_dest: Coord(A, T), second_dest: Coord(A, T) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.7804344199524331;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
        let chooser: f64 = 0.06444280325191454;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
        let chooser: f64 = 0.4837340770860973;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcDst { src: Coord(O, M), dest: Coord(O, Z) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.18562162349847133;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(TamMoveStepsDuringLatter { src: Coord(A, T), step: Coord(A, Z), first_dest: Coord(E, Z), second_dest: Coord(E, Z) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.1499190714675267;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(AU, P), step: Coord(AU, X), planned_direction: Coord(AU, M) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::InfAfterStep(m) => {
        let chooser = 0.8178081083976431;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: Some(Coord(AU, P)) };
        let chooser2 = 0.6438224454810676;
        let hnr_state = apply_after_half_acceptance(&ext_state, aha_move, config)
            .unwrap()
            .choose_by_uniform_random_variable(chooser2)
            .0;
        
        hnr_state
    },
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
let pure_move = Some(NormalMove(TamMoveStepsDuringFormer { src: Coord(E, Z), step: Coord(I, T), first_dest: Coord(U, T), second_dest: Coord(O, N) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.0023398927667200864;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcDst { src: Coord(AI, C), dest: Coord(O, C) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.44209103839312125;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(TamMoveStepsDuringFormer { src: Coord(O, N), step: Coord(O, T), first_dest: Coord(Y, Z), second_dest: Coord(O, X) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.193006679117816;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(IA, Z), step: Coord(AU, T), dest: Coord(AU, N) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.9422468928925221;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(TamMoveNoStep { src: Coord(O, X), first_dest: Coord(U, C), second_dest: Coord(O, X) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.6729108985066614;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcDst { src: Coord(I, X), dest: Coord(E, X) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.17437538541856035;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(TamMoveStepsDuringLatter { src: Coord(O, X), step: Coord(O, T), first_dest: Coord(Y, Z), second_dest: Coord(Y, N) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.6840979054995986;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveFromHopZuo { color: Huok2, prof: Dau2, dest: Coord(IA, Z) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.5169090468580382;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(TamMoveNoStep { src: Coord(Y, N), first_dest: Coord(O, L), second_dest: Coord(U, L) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.8436596997991634;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
        let chooser: f64 = 0.6115124878200572;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(TamMoveStepsDuringLatter { src: Coord(U, L), step: Coord(I, K), first_dest: Coord(U, K), second_dest: Coord(U, L) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.8174901033481861;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(O, M), step: Coord(AI, M), planned_direction: Coord(O, M) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::InfAfterStep(m) => {
        let chooser = 0.8264407573093393;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: Some(Coord(U, M)) };
        let chooser2 = 0.6589832408520828;
        let hnr_state = apply_after_half_acceptance(&ext_state, aha_move, config)
            .unwrap()
            .choose_by_uniform_random_variable(chooser2)
            .0;
        
        hnr_state
    },
    _ => { panic!("Expected PureMove::InfAfterStep") }
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(U, N), step: Coord(U, L), dest: Coord(Y, L) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.9254349286356827;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(AU, L), step: Coord(Y, L), planned_direction: Coord(Y, N) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::InfAfterStep(m) => {
        let chooser = 0.7069222417013569;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: None };
        let chooser2 = 0.29562572847120605;
        let hnr_state = apply_after_half_acceptance(&ext_state, aha_move, config)
            .unwrap()
            .choose_by_uniform_random_variable(chooser2)
            .0;
        
        hnr_state
    },
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
        let chooser: f64 = 0.02529036798438078;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(Y, P), step: Coord(Y, L), dest: Coord(AI, L) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.12062831224804282;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(TamMoveStepsDuringFormer { src: Coord(U, L), step: Coord(I, N), first_dest: Coord(U, L), second_dest: Coord(O, N) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.13784053459552048;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
        let chooser: f64 = 0.6050189284927249;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(TamMoveStepsDuringLatter { src: Coord(O, N), step: Coord(U, T), first_dest: Coord(U, N), second_dest: Coord(O, Z) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.7776969294693288;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(AU, Z), step: Coord(IA, X), dest: Coord(AU, Z) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.30039388721278637;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(TamMoveStepsDuringFormer { src: Coord(O, Z), step: Coord(U, T), first_dest: Coord(O, Z), second_dest: Coord(O, X) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.21206361761037484;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(AI, L), step: Coord(AI, N), planned_direction: Coord(E, M) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::InfAfterStep(m) => {
        let chooser = 0.05767284781728277;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: Some(Coord(Y, T)) };
        let chooser2 = 0.9112731067032563;
        let hnr_state = apply_after_half_acceptance(&ext_state, aha_move, config)
            .unwrap()
            .choose_by_uniform_random_variable(chooser2)
            .0;
        
        hnr_state
    },
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
let pure_move = Some(NormalMove(TamMoveNoStep { src: Coord(O, X), first_dest: Coord(O, Z), second_dest: Coord(O, X) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.18224144508851836;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(AU, N), step: Coord(IA, N), dest: Coord(IA, T) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.0076998107794243875;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(TamMoveStepsDuringLatter { src: Coord(O, X), step: Coord(Y, T), first_dest: Coord(O, Z), second_dest: Coord(Y, N) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.05211183442242717;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
        let chooser: f64 = 0.603767485104352;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcDst { src: Coord(A, X), dest: Coord(E, C) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.554764378124943;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(IA, T), step: Coord(AU, Z), dest: Coord(AU, X) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.6148101360364318;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcDst { src: Coord(I, T), dest: Coord(E, Z) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.15836031662414352;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(TamMoveStepsDuringFormer { src: Coord(Y, N), step: Coord(AI, N), first_dest: Coord(Y, N), second_dest: Coord(O, N) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.511673908193431;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(A, N), step: Coord(I, N), dest: Coord(O, Z) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.7215660019306626;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcDst { src: Coord(Y, T), dest: Coord(I, C) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.4001127131127312;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(TamMoveNoStep { src: Coord(O, N), first_dest: Coord(U, L), second_dest: Coord(O, N) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.21269833705446728;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveFromHopZuo { color: Huok2, prof: Kauk2, dest: Coord(A, M) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.7328486480464484;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(TamMoveStepsDuringFormer { src: Coord(O, N), step: Coord(U, T), first_dest: Coord(U, Z), second_dest: Coord(I, X) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.5585139249194357;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(I, C), step: Coord(E, X), planned_direction: Coord(E, Z) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::InfAfterStep(m) => {
        let chooser = 0.9945308761784482;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: None };
        let chooser2 = 0.46991108906553214;
        let hnr_state = apply_after_half_acceptance(&ext_state, aha_move, config)
            .unwrap()
            .choose_by_uniform_random_variable(chooser2)
            .0;
        
        hnr_state
    },
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
let pure_move = Some(NormalMove(TamMoveStepsDuringLatter { src: Coord(I, X), step: Coord(I, Z), first_dest: Coord(U, Z), second_dest: Coord(I, T) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.7128304055099584;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
        let chooser: f64 = 0.5279589474817777;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(TamMoveStepsDuringFormer { src: Coord(I, T), step: Coord(U, T), first_dest: Coord(O, N), second_dest: Coord(Y, T) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.9656177101484367;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
        let chooser: f64 = 0.7507052708163942;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcDst { src: Coord(O, T), dest: Coord(I, X) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.07389388344763959;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(IA, P), step: Coord(IA, M), dest: Coord(IA, P) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.9640992541054839;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
        let chooser = 0.04480355916713419;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: None };
        let chooser2 = 0.15830438833346816;
        let hnr_state = apply_after_half_acceptance(&ext_state, aha_move, config)
            .unwrap()
            .choose_by_uniform_random_variable(chooser2)
            .0;
        
        hnr_state
    },
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(AU, L), step: Coord(AU, K), planned_direction: Coord(AU, L) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::InfAfterStep(m) => {
        let chooser = 0.5225200560949131;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: Some(Coord(AU, L)) };
        let chooser2 = 0.4786089734518576;
        let hnr_state = apply_after_half_acceptance(&ext_state, aha_move, config)
            .unwrap()
            .choose_by_uniform_random_variable(chooser2)
            .0;
        
        hnr_state
    },
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
        let chooser: f64 = 0.13231042387053837;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcDst { src: Coord(O, P), dest: Coord(I, P) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.650819754776546;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcDst { src: Coord(O, Z), dest: Coord(I, N) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.672586247471031;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveFromHopZuo { color: Huok2, prof: Kauk2, dest: Coord(U, C) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.1262784009158867;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
        let chooser: f64 = 0.4507257476684313;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(I, C), step: Coord(I, P), dest: Coord(E, P) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.0890440130318928;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveFromHopZuo { color: Kok1, prof: Kauk2, dest: Coord(A, X) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.9407567216440604;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveFromHopZuo { color: Huok2, prof: Tuk2, dest: Coord(O, K) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.9866494651340905;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(TamMoveStepsDuringLatter { src: Coord(Y, T), step: Coord(AI, Z), first_dest: Coord(Y, Z), second_dest: Coord(Y, X) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.8093937674987624;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
        let chooser: f64 = 0.7926224344906708;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcDst { src: Coord(I, Z), dest: Coord(AI, Z) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.8825644569154611;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveFromHopZuo { color: Kok1, prof: Kauk2, dest: Coord(U, P) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.9182117608573586;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveFromHopZuo { color: Kok1, prof: Dau2, dest: Coord(AU, N) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.6904268109132211;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(TamMoveNoStep { src: Coord(Y, X), first_dest: Coord(O, C), second_dest: Coord(Y, M) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.8497855716120399;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(E, M), step: Coord(I, M), planned_direction: Coord(A, M) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::InfAfterStep(m) => {
        let chooser = 0.7295399558566512;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: Some(Coord(A, M)) };
        let chooser2 = 0.023830082749818793;
        let hnr_state = apply_after_half_acceptance(&ext_state, aha_move, config)
            .unwrap()
            .choose_by_uniform_random_variable(chooser2)
            .0;
        
        hnr_state
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcDst { src: Coord(O, K), dest: Coord(U, K) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.669026242322707;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveFromHopZuo { color: Huok2, prof: Kauk2, dest: Coord(U, Z) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.3788163663387125;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(TamMoveStepsDuringFormer { src: Coord(Y, M), step: Coord(AI, C), first_dest: Coord(Y, M), second_dest: Coord(O, P) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.5576906586100578;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
        let chooser = 0.7244447055704539;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: None };
        let chooser2 = 0.10055395488658148;
        let hnr_state = apply_after_half_acceptance(&ext_state, aha_move, config)
            .unwrap()
            .choose_by_uniform_random_variable(chooser2)
            .0;
        
        hnr_state
    },
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(I, M), step: Coord(U, M), planned_direction: Coord(I, C) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::InfAfterStep(m) => {
        let chooser = 0.45305490449095986;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: Some(Coord(I, C)) };
        let chooser2 = 0.9031922804315405;
        let hnr_state = apply_after_half_acceptance(&ext_state, aha_move, config)
            .unwrap()
            .choose_by_uniform_random_variable(chooser2)
            .0;
        
        hnr_state
    },
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
let pure_move = Some(NormalMove(TamMoveNoStep { src: Coord(O, P), first_dest: Coord(O, M), second_dest: Coord(O, P) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.6426460930989001;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(I, C), step: Coord(A, C), planned_direction: Coord(A, M) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::InfAfterStep(m) => {
        let chooser = 0.8822517362061298;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: Some(Coord(A, M)) };
        let chooser2 = 0.49685649349115557;
        let hnr_state = apply_after_half_acceptance(&ext_state, aha_move, config)
            .unwrap()
            .choose_by_uniform_random_variable(chooser2)
            .0;
        
        hnr_state
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcDst { src: Coord(E, T), dest: Coord(I, Z) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.2526587635253267;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveFromHopZuo { color: Huok2, prof: Gua2, dest: Coord(Y, T) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.6444198594433668;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
        let chooser: f64 = 0.7005729872382283;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
        let chooser: f64 = 0.9515133459985545;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveFromHopZuo { color: Huok2, prof: Nuak1, dest: Coord(O, M) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.7684234218249328;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(AU, Z), step: Coord(AI, X), dest: Coord(AU, C) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.7815216976246485;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
        let chooser: f64 = 0.22923160758440553;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
        let chooser: f64 = 0.4475829251457546;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveFromHopZuo { color: Huok2, prof: Tuk2, dest: Coord(IA, L) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.7982682547792995;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(Y, T), step: Coord(AI, Z), planned_direction: Coord(O, Z) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::InfAfterStep(m) => {
        let chooser = 0.2457993315462581;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: Some(Coord(O, Z)) };
        let chooser2 = 0.2799746281287103;
        let hnr_state = apply_after_half_acceptance(&ext_state, aha_move, config)
            .unwrap()
            .choose_by_uniform_random_variable(chooser2)
            .0;
        
        hnr_state
    },
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
let pure_move = Some(NormalMove(TamMoveNoStep { src: Coord(O, P), first_dest: Coord(Y, P), second_dest: Coord(O, P) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.8695588280938173;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(IA, X), step: Coord(IA, Z), dest: Coord(AU, T) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.7916631497203923;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(A, P), step: Coord(A, M), planned_direction: Coord(U, M) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::InfAfterStep(m) => {
        let chooser = 0.9143065933153531;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: Some(Coord(U, M)) };
        let chooser2 = 0.02687625051583642;
        let hnr_state = apply_after_half_acceptance(&ext_state, aha_move, config)
            .unwrap()
            .choose_by_uniform_random_variable(chooser2)
            .0;
        
        hnr_state
    },
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
        let chooser = 0.6779290353166255;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: None };
        let chooser2 = 0.9259651231353923;
        let hnr_state = apply_after_half_acceptance(&ext_state, aha_move, config)
            .unwrap()
            .choose_by_uniform_random_variable(chooser2)
            .0;
        
        hnr_state
    },
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
let pure_move = Some(NormalMove(NonTamMoveFromHopZuo { color: Kok1, prof: Gua2, dest: Coord(A, P) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.17539956477603003;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcDst { src: Coord(AU, T), dest: Coord(AI, T) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.657076873516164;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(TamMoveStepsDuringLatter { src: Coord(O, P), step: Coord(O, M), first_dest: Coord(Y, P), second_dest: Coord(Y, C) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.2157479127089209;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveFromHopZuo { color: Kok1, prof: Kauk2, dest: Coord(I, Z) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.5805018637307302;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcDst { src: Coord(E, T), dest: Coord(I, Z) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.37164633942229064;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(AU, C), step: Coord(AU, X), dest: Coord(IA, C) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.8096289415786274;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(E, C), step: Coord(I, X), dest: Coord(E, Z) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.38797547943842203;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(TamMoveStepsDuringLatter { src: Coord(Y, C), step: Coord(AI, M), first_dest: Coord(Y, M), second_dest: Coord(AU, C) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.05609085865977359;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
        let chooser: f64 = 0.49540779520343536;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
        let chooser: f64 = 0.239746360479955;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcDst { src: Coord(U, M), dest: Coord(E, M) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.2518780358043621;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(AI, C), step: Coord(AI, M), dest: Coord(AU, M) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.6449305703532286;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(E, L), step: Coord(I, L), planned_direction: Coord(I, K) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::InfAfterStep(m) => {
        let chooser = 0.8447531603738629;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: Some(Coord(I, K)) };
        let chooser2 = 0.9031798622538344;
        let hnr_state = apply_after_half_acceptance(&ext_state, aha_move, config)
            .unwrap()
            .choose_by_uniform_random_variable(chooser2)
            .0;
        
        hnr_state
    },
    _ => { panic!("Expected PureMove::InfAfterStep") }
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(O, X), step: Coord(AI, M), planned_direction: Coord(U, Z) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::InfAfterStep(m) => {
        let chooser = 0.48987017113043796;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: Some(Coord(O, X)) };
        let chooser2 = 0.01588830849265077;
        let hnr_state = apply_after_half_acceptance(&ext_state, aha_move, config)
            .unwrap()
            .choose_by_uniform_random_variable(chooser2)
            .0;
        
        hnr_state
    },
    _ => { panic!("Expected PureMove::InfAfterStep") }
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(E, T), step: Coord(A, N), dest: Coord(E, L) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.19438123147412056;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(TamMoveStepsDuringLatter { src: Coord(AU, C), step: Coord(AI, X), first_dest: Coord(AI, C), second_dest: Coord(AU, C) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.4479985382768221;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(O, M), step: Coord(AI, M), dest: Coord(AI, X) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.006575940036787631;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(AI, T), step: Coord(Y, T), dest: Coord(O, Z) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.4839184073270206;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveFromHopZuo { color: Kok1, prof: Kauk2, dest: Coord(AI, L) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.6075857505464589;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(TamMoveStepsDuringLatter { src: Coord(AU, C), step: Coord(AU, X), first_dest: Coord(IA, X), second_dest: Coord(AI, C) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.5351866865897968;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(AI, X), step: Coord(AI, Z), planned_direction: Coord(AU, Z) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::InfAfterStep(m) => {
        let chooser = 0.9321825703915559;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: Some(Coord(IA, Z)) };
        let chooser2 = 0.605626616147329;
        let hnr_state = apply_after_half_acceptance(&ext_state, aha_move, config)
            .unwrap()
            .choose_by_uniform_random_variable(chooser2)
            .0;
        
        hnr_state
    },
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
let pure_move = Some(NormalMove(TamMoveStepsDuringLatter { src: Coord(AI, C), step: Coord(AU, X), first_dest: Coord(AI, X), second_dest: Coord(AU, Z) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.7141046105152038;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveFromHopZuo { color: Huok2, prof: Dau2, dest: Coord(O, Z) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.7070486062657921;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(TamMoveNoStep { src: Coord(AU, Z), first_dest: Coord(IA, X), second_dest: Coord(AU, C) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.29786492324230673;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(E, M), step: Coord(A, M), planned_direction: Coord(E, M) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::InfAfterStep(m) => {
        let chooser = 0.5489605105229063;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: None };
        let chooser2 = 0.5851711808775005;
        let hnr_state = apply_after_half_acceptance(&ext_state, aha_move, config)
            .unwrap()
            .choose_by_uniform_random_variable(chooser2)
            .0;
        
        hnr_state
    },
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
let pure_move = Some(NormalMove(TamMoveStepsDuringLatter { src: Coord(AU, C), step: Coord(AI, M), first_dest: Coord(AI, C), second_dest: Coord(Y, P) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.12558998705591795;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(E, M), step: Coord(AI, M), planned_direction: Coord(U, M) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::InfAfterStep(m) => {
        let chooser = 0.13263589149841393;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: Some(Coord(Y, M)) };
        let chooser2 = 0.3172480262397943;
        let hnr_state = apply_after_half_acceptance(&ext_state, aha_move, config)
            .unwrap()
            .choose_by_uniform_random_variable(chooser2)
            .0;
        
        hnr_state
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(I, P), step: Coord(A, P), dest: Coord(I, P) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.967326270898443;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
        let chooser: f64 = 0.17107478256335362;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(TamMoveNoStep { src: Coord(Y, P), first_dest: Coord(O, M), second_dest: Coord(Y, C) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.19035745041775975;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(E, K), step: Coord(E, L), planned_direction: Coord(E, N) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::InfAfterStep(m) => {
        let chooser = 0.5450467118328975;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: Some(Coord(E, T)) };
        let chooser2 = 0.3266788691917091;
        let hnr_state = apply_after_half_acceptance(&ext_state, aha_move, config)
            .unwrap()
            .choose_by_uniform_random_variable(chooser2)
            .0;
        
        hnr_state
    },
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
let pure_move = Some(NormalMove(TamMoveStepsDuringLatter { src: Coord(Y, C), step: Coord(AU, M), first_dest: Coord(AI, C), second_dest: Coord(AI, C) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.004163753607295373;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcDst { src: Coord(Y, M), dest: Coord(Y, C) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.9622744093811497;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(TamMoveNoStep { src: Coord(AI, C), first_dest: Coord(AI, X), second_dest: Coord(Y, Z) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.2650802155432138;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(A, Z), step: Coord(E, X), dest: Coord(I, C) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.47850879351389797;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(Y, T), step: Coord(AI, Z), planned_direction: Coord(O, N) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::InfAfterStep(m) => {
        let chooser = 0.48586531222336005;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: None };
        let chooser2 = 0.34926930770920905;
        let hnr_state = apply_after_half_acceptance(&ext_state, aha_move, config)
            .unwrap()
            .choose_by_uniform_random_variable(chooser2)
            .0;
        
        hnr_state
    },
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(IA, L), step: Coord(IA, N), planned_direction: Coord(IA, T) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::InfAfterStep(m) => {
        let chooser = 0.1026692429218099;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: Some(Coord(IA, T)) };
        let chooser2 = 0.2459041983153456;
        let hnr_state = apply_after_half_acceptance(&ext_state, aha_move, config)
            .unwrap()
            .choose_by_uniform_random_variable(chooser2)
            .0;
        
        hnr_state
    },
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
let pure_move = Some(NormalMove(TamMoveStepsDuringFormer { src: Coord(Y, Z), step: Coord(Y, T), first_dest: Coord(O, T), second_dest: Coord(O, N) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.26520612504114516;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(O, Z), step: Coord(I, N), planned_direction: Coord(U, T) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::InfAfterStep(m) => {
        let chooser = 0.24842324153615847;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: Some(Coord(U, T)) };
        let chooser2 = 0.6382627524031508;
        let hnr_state = apply_after_half_acceptance(&ext_state, aha_move, config)
            .unwrap()
            .choose_by_uniform_random_variable(chooser2)
            .0;
        
        hnr_state
    },
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
let pure_move = Some(NormalMove(TamMoveStepsDuringFormer { src: Coord(O, N), step: Coord(U, T), first_dest: Coord(O, Z), second_dest: Coord(U, X) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.3711240083143509;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(Y, C), step: Coord(U, C), planned_direction: Coord(IA, C) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::InfAfterStep(m) => {
        let chooser = 0.7735421851932052;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: None };
        let chooser2 = 0.09512462039172442;
        let hnr_state = apply_after_half_acceptance(&ext_state, aha_move, config)
            .unwrap()
            .choose_by_uniform_random_variable(chooser2)
            .0;
        
        hnr_state
    },
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
let pure_move = Some(NormalMove(TamMoveStepsDuringLatter { src: Coord(U, X), step: Coord(O, X), first_dest: Coord(O, C), second_dest: Coord(Y, Z) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.7003769607458458;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(E, X), step: Coord(I, C), dest: Coord(U, X) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.8984161276126792;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(AU, L), step: Coord(AI, L), planned_direction: Coord(AU, L) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::InfAfterStep(m) => {
        let chooser = 0.9165234860267586;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: None };
        let chooser2 = 0.10782541217783925;
        let hnr_state = apply_after_half_acceptance(&ext_state, aha_move, config)
            .unwrap()
            .choose_by_uniform_random_variable(chooser2)
            .0;
        
        hnr_state
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(I, C), step: Coord(U, X), dest: Coord(I, C) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.755039566459115;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(AI, T), step: Coord(AI, N), dest: Coord(AU, T) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.0633379301971485;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(IA, T), step: Coord(AU, T), dest: Coord(AI, T) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.5454988608365254;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(AU, T), step: Coord(AI, T), dest: Coord(AI, Z) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.4904908155784231;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(U, X), step: Coord(I, Z), dest: Coord(I, T) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.3424532860687204;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(TamMoveStepsDuringLatter { src: Coord(Y, Z), step: Coord(U, Z), first_dest: Coord(O, Z), second_dest: Coord(O, T) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.5742966352173076;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(I, T), step: Coord(I, Z), dest: Coord(E, X) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.37985091951070105;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(TamMoveStepsDuringFormer { src: Coord(O, T), step: Coord(U, Z), first_dest: Coord(O, T), second_dest: Coord(Y, Z) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.7144732000748032;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcDst { src: Coord(Y, C), dest: Coord(U, C) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.09907424851199287;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(TamMoveNoStep { src: Coord(Y, Z), first_dest: Coord(AI, X), second_dest: Coord(AU, C) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.2173092663798788;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
        let chooser: f64 = 0.563729655769531;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcDst { src: Coord(AI, M), dest: Coord(AI, C) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.5915928788823114;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(TamMoveStepsDuringFormer { src: Coord(AU, C), step: Coord(AI, C), first_dest: Coord(Y, M), second_dest: Coord(O, M) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.9835158771599296;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcDst { src: Coord(Y, T), dest: Coord(U, L) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.7593940637219068;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(AI, T), step: Coord(AU, T), planned_direction: Coord(AU, X) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::InfAfterStep(m) => {
        let chooser = 0.5767343500171337;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: Some(Coord(AU, X)) };
        let chooser2 = 0.5416335880217614;
        let hnr_state = apply_after_half_acceptance(&ext_state, aha_move, config)
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
        


        match if_taxot {
            IfTaxot::NextSeason(ps) => {
                let chooser = 0.9365891118320441;
                state = ps.clone().choose_by_uniform_random_variable(chooser).0
            },
            IfTaxot::VictoriousSide(v) => {
                panic!("Expected IfTaxot::VictoriousSide")
            }
        }
    },
    _ => { panic!("Expected HandResolved::HandExists") }
}
                                
// Turn #179
let (hop1zuo1_candidates, candidates) = state.get_candidates(config);
let pure_move = Some(NormalMove(TamMoveNoStep { src: Coord(O, Z), first_dest: Coord(O, T), second_dest: Coord(U, Z) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.71475002552691;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(AU, M), step: Coord(AI, M), planned_direction: Coord(O, M) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::InfAfterStep(m) => {
        let chooser = 0.4755032655084489;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: None };
        let chooser2 = 0.2811738238143625;
        let hnr_state = apply_after_half_acceptance(&ext_state, aha_move, config)
            .unwrap()
            .choose_by_uniform_random_variable(chooser2)
            .0;
        
        hnr_state
    },
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(E, X), step: Coord(I, Z), planned_direction: Coord(O, C) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::InfAfterStep(m) => {
        let chooser = 0.18367035351979888;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: Some(Coord(U, X)) };
        let chooser2 = 0.32494224605704536;
        let hnr_state = apply_after_half_acceptance(&ext_state, aha_move, config)
            .unwrap()
            .choose_by_uniform_random_variable(chooser2)
            .0;
        
        hnr_state
    },
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(AU, L), step: Coord(AI, L), planned_direction: Coord(I, L) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::InfAfterStep(m) => {
        let chooser = 0.06352043546886166;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: None };
        let chooser2 = 0.013801233394899137;
        let hnr_state = apply_after_half_acceptance(&ext_state, aha_move, config)
            .unwrap()
            .choose_by_uniform_random_variable(chooser2)
            .0;
        
        hnr_state
    },
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(U, X), step: Coord(AI, N), planned_direction: Coord(U, X) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::InfAfterStep(m) => {
        let chooser = 0.5511946923843769;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: None };
        let chooser2 = 0.3066751598604557;
        let hnr_state = apply_after_half_acceptance(&ext_state, aha_move, config)
            .unwrap()
            .choose_by_uniform_random_variable(chooser2)
            .0;
        
        hnr_state
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(IA, T), step: Coord(IA, N), dest: Coord(AU, N) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.764762688749444;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
        let chooser = 0.04543059380307435;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: None };
        let chooser2 = 0.15677701639192176;
        let hnr_state = apply_after_half_acceptance(&ext_state, aha_move, config)
            .unwrap()
            .choose_by_uniform_random_variable(chooser2)
            .0;
        
        hnr_state
    },
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
        let chooser: f64 = 0.2600568898736598;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(E, T), step: Coord(I, Z), planned_direction: Coord(U, T) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::InfAfterStep(m) => {
        let chooser = 0.8692401075806275;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: Some(Coord(AI, K)) };
        let chooser2 = 0.3159953075256742;
        let hnr_state = apply_after_half_acceptance(&ext_state, aha_move, config)
            .unwrap()
            .choose_by_uniform_random_variable(chooser2)
            .0;
        
        hnr_state
    },
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
let pure_move = Some(NormalMove(TamMoveStepsDuringFormer { src: Coord(U, Z), step: Coord(I, T), first_dest: Coord(E, T), second_dest: Coord(E, N) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.5090141401642542;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
        let chooser: f64 = 0.27836491450890244;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(TamMoveStepsDuringFormer { src: Coord(E, N), step: Coord(I, T), first_dest: Coord(U, T), second_dest: Coord(O, Z) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.5921662596630293;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
        let chooser = 0.014455263510445682;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: None };
        let chooser2 = 0.2882024110455381;
        let hnr_state = apply_after_half_acceptance(&ext_state, aha_move, config)
            .unwrap()
            .choose_by_uniform_random_variable(chooser2)
            .0;
        
        hnr_state
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcDst { src: Coord(IA, T), dest: Coord(IA, Z) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.7321364892772432;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(TamMoveNoStep { src: Coord(O, Z), first_dest: Coord(O, T), second_dest: Coord(U, Z) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.14799219835859945;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
        let chooser: f64 = 0.8874688085765503;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
        let chooser: f64 = 0.858207012590707;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(TamMoveStepsDuringLatter { src: Coord(U, Z), step: Coord(O, Z), first_dest: Coord(U, X), second_dest: Coord(U, Z) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.6504543596940252;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
        let chooser: f64 = 0.9990226455551917;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(TamMoveStepsDuringLatter { src: Coord(U, Z), step: Coord(I, Z), first_dest: Coord(U, X), second_dest: Coord(E, T) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.3188145841033797;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcDst { src: Coord(I, X), dest: Coord(U, X) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.5105205898396894;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(TamMoveStepsDuringFormer { src: Coord(E, T), step: Coord(I, T), first_dest: Coord(E, Z), second_dest: Coord(E, X) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.21273939217523086;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
        let chooser: f64 = 0.8408592747137854;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(IA, K), step: Coord(AU, K), planned_direction: Coord(AI, K) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::InfAfterStep(m) => {
        let chooser = 0.8452756605418844;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: None };
        let chooser2 = 0.22561713579764198;
        let hnr_state = apply_after_half_acceptance(&ext_state, aha_move, config)
            .unwrap()
            .choose_by_uniform_random_variable(chooser2)
            .0;
        
        hnr_state
    },
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
let pure_move = Some(NormalMove(TamMoveStepsDuringFormer { src: Coord(E, X), step: Coord(I, Z), first_dest: Coord(E, T), second_dest: Coord(E, Z) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.2361966442750928;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(IA, P), step: Coord(IA, M), dest: Coord(IA, C) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.4546533410726282;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcDst { src: Coord(E, L), dest: Coord(I, L) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.36559454221133014;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(TamMoveStepsDuringLatter { src: Coord(E, Z), step: Coord(I, N), first_dest: Coord(E, T), second_dest: Coord(U, N) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.09121099748053452;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(E, K), step: Coord(E, M), planned_direction: Coord(E, C) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::InfAfterStep(m) => {
        let chooser = 0.9916649474670881;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: None };
        let chooser2 = 0.2683104127528648;
        let hnr_state = apply_after_half_acceptance(&ext_state, aha_move, config)
            .unwrap()
            .choose_by_uniform_random_variable(chooser2)
            .0;
        
        hnr_state
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(IA, Z), step: Coord(AU, T), dest: Coord(IA, T) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.5594731073114114;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(TamMoveStepsDuringLatter { src: Coord(U, N), step: Coord(U, L), first_dest: Coord(O, L), second_dest: Coord(O, L) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.5325188913604467;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(AI, N), step: Coord(AU, N), dest: Coord(AI, N) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.5834371242090368;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(E, M), step: Coord(E, K), planned_direction: Coord(E, M) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::InfAfterStep(m) => {
        let chooser = 0.4347156090155909;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: None };
        let chooser2 = 0.6969606917747205;
        let hnr_state = apply_after_half_acceptance(&ext_state, aha_move, config)
            .unwrap()
            .choose_by_uniform_random_variable(chooser2)
            .0;
        
        hnr_state
    },
    _ => { panic!("Expected PureMove::InfAfterStep") }
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(AU, N), step: Coord(AI, L), dest: Coord(AI, K) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.8776578316613428;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(A, X), step: Coord(A, C), dest: Coord(A, X) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.5438967173261816;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveFromHopZuo { color: Kok1, prof: Dau2, dest: Coord(E, Z) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.7138752332020498;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(TamMoveStepsDuringLatter { src: Coord(O, L), step: Coord(U, L), first_dest: Coord(U, N), second_dest: Coord(U, N) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.3120059409013056;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(O, Z), step: Coord(AI, N), dest: Coord(O, Z) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.8239498298867226;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(A, X), step: Coord(A, Z), dest: Coord(E, X) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.6746643593979796;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(TamMoveStepsDuringLatter { src: Coord(U, N), step: Coord(U, L), first_dest: Coord(O, L), second_dest: Coord(O, K) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.9980164312647081;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(A, N), step: Coord(I, N), dest: Coord(O, Z) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.27437102656020695;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(IA, T), step: Coord(IA, N), dest: Coord(IA, T) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.7076347753969441;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(E, X), step: Coord(E, Z), dest: Coord(E, T) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.47042337121266986;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(TamMoveStepsDuringLatter { src: Coord(O, K), step: Coord(AI, L), first_dest: Coord(Y, K), second_dest: Coord(Y, K) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.5534318030591834;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(I, Z), step: Coord(O, Z), dest: Coord(O, N) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.5812005872970726;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(TamMoveStepsDuringLatter { src: Coord(Y, K), step: Coord(O, N), first_dest: Coord(O, L), second_dest: Coord(U, T) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.020037898028889445;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(O, N), step: Coord(O, Z), planned_direction: Coord(U, Z) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::InfAfterStep(m) => {
        let chooser = 0.2272055224903894;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: Some(Coord(I, Z)) };
        let chooser2 = 0.5591535246295793;
        let hnr_state = apply_after_half_acceptance(&ext_state, aha_move, config)
            .unwrap()
            .choose_by_uniform_random_variable(chooser2)
            .0;
        
        hnr_state
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(AI, Z), step: Coord(O, Z), dest: Coord(O, T) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.809803696568085;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(I, L), step: Coord(U, L), planned_direction: Coord(I, L) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::InfAfterStep(m) => {
        let chooser = 0.31522690939864395;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: Some(Coord(I, L)) };
        let chooser2 = 0.9511999568622531;
        let hnr_state = apply_after_half_acceptance(&ext_state, aha_move, config)
            .unwrap()
            .choose_by_uniform_random_variable(chooser2)
            .0;
        
        hnr_state
    },
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
        let chooser: f64 = 0.6749314263567289;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(E, T), step: Coord(I, Z), dest: Coord(E, T) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.5299230911146852;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(TamMoveStepsDuringLatter { src: Coord(U, T), step: Coord(I, Z), first_dest: Coord(U, Z), second_dest: Coord(U, Z) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.22392801307360188;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
        let chooser: f64 = 0.5047805967280528;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
        let chooser: f64 = 0.5687296314205023;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(A, T), step: Coord(E, Z), dest: Coord(I, X) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.9738892765786036;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(AU, T), step: Coord(AI, N), planned_direction: Coord(IA, Z) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::InfAfterStep(m) => {
        let chooser = 0.7830241203140232;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: None };
        let chooser2 = 0.8057117796437911;
        let hnr_state = apply_after_half_acceptance(&ext_state, aha_move, config)
            .unwrap()
            .choose_by_uniform_random_variable(chooser2)
            .0;
        
        hnr_state
    },
    _ => { panic!("Expected PureMove::InfAfterStep") }
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
let pure_move = Some(NormalMove(NonTamMoveSrcDst { src: Coord(I, X), dest: Coord(E, Z) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.674144102260828;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(TamMoveStepsDuringFormer { src: Coord(U, Z), step: Coord(O, Z), first_dest: Coord(Y, T), second_dest: Coord(Y, Z) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.8884363149816908;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveFromHopZuo { color: Kok1, prof: Dau2, dest: Coord(I, X) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.16913802669958578;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(AI, X), step: Coord(AU, X), dest: Coord(AI, X) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.6872386651297863;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(I, L), step: Coord(U, L), planned_direction: Coord(Y, L) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::InfAfterStep(m) => {
        let chooser = 0.8807470688934932;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: Some(Coord(O, L)) };
        let chooser2 = 0.8984539629231469;
        let hnr_state = apply_after_half_acceptance(&ext_state, aha_move, config)
            .unwrap()
            .choose_by_uniform_random_variable(chooser2)
            .0;
        
        hnr_state
    },
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
let pure_move = Some(NormalMove(TamMoveStepsDuringLatter { src: Coord(Y, Z), step: Coord(O, T), first_dest: Coord(Y, T), second_dest: Coord(U, Z) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.8307211707125891;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(A, N), step: Coord(I, N), dest: Coord(O, K) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.638098672797797;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(AU, L), step: Coord(AI, L), planned_direction: Coord(O, L) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::InfAfterStep(m) => {
        let chooser = 0.5593076880876515;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: Some(Coord(Y, L)) };
        let chooser2 = 0.738060059078381;
        let hnr_state = apply_after_half_acceptance(&ext_state, aha_move, config)
            .unwrap()
            .choose_by_uniform_random_variable(chooser2)
            .0;
        
        hnr_state
    },
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
let pure_move = Some(NormalMove(TamMoveStepsDuringFormer { src: Coord(U, Z), step: Coord(O, Z), first_dest: Coord(Y, T), second_dest: Coord(Y, N) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.32740982293319687;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(AI, Z), step: Coord(AU, X), dest: Coord(IA, Z) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.8545155096653537;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(TamMoveStepsDuringFormer { src: Coord(Y, N), step: Coord(O, T), first_dest: Coord(Y, T), second_dest: Coord(O, N) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.8049235614584435;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(AI, K), step: Coord(Y, L), dest: Coord(Y, N) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.994464086398588;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(TamMoveStepsDuringLatter { src: Coord(O, N), step: Coord(I, T), first_dest: Coord(U, N), second_dest: Coord(U, T) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.3653656462407675;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
        let chooser: f64 = 0.6812827709243723;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(A, C), step: Coord(A, Z), dest: Coord(A, C) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.9974942049120956;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveFromHopZuo { color: Kok1, prof: Gua2, dest: Coord(AU, Z) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.8763802161833876;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(TamMoveStepsDuringFormer { src: Coord(U, T), step: Coord(I, N), first_dest: Coord(E, L), second_dest: Coord(I, L) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.9790153631291444;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcDst { src: Coord(IA, K), dest: Coord(IA, L) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.7676340916220045;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(O, C), step: Coord(U, X), planned_direction: Coord(O, C) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::InfAfterStep(m) => {
        let chooser = 0.6551375630702919;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: Some(Coord(AI, P)) };
        let chooser2 = 0.4379945580146817;
        let hnr_state = apply_after_half_acceptance(&ext_state, aha_move, config)
            .unwrap()
            .choose_by_uniform_random_variable(chooser2)
            .0;
        
        hnr_state
    },
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(AU, M), step: Coord(AI, M), planned_direction: Coord(O, M) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::InfAfterStep(m) => {
        let chooser = 0.10021725446043761;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: None };
        let chooser2 = 0.4457215138081817;
        let hnr_state = apply_after_half_acceptance(&ext_state, aha_move, config)
            .unwrap()
            .choose_by_uniform_random_variable(chooser2)
            .0;
        
        hnr_state
    },
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
        let chooser: f64 = 0.46490536522878456;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(O, T), step: Coord(I, T), planned_direction: Coord(E, T) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::InfAfterStep(m) => {
        let chooser = 0.031576998991783234;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: None };
        let chooser2 = 0.7025632199946352;
        let hnr_state = apply_after_half_acceptance(&ext_state, aha_move, config)
            .unwrap()
            .choose_by_uniform_random_variable(chooser2)
            .0;
        
        hnr_state
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(I, X), step: Coord(E, Z), dest: Coord(A, X) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.5574425020557983;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(TamMoveStepsDuringLatter { src: Coord(I, L), step: Coord(I, T), first_dest: Coord(E, N), second_dest: Coord(U, N) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.7191448383440481;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(E, T), step: Coord(I, Z), dest: Coord(E, T) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.9598677202291988;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(TamMoveStepsDuringFormer { src: Coord(U, N), step: Coord(O, L), first_dest: Coord(U, N), second_dest: Coord(O, N) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.04001854240333236;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
        let chooser: f64 = 0.049675469125541105;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcDst { src: Coord(Y, P), dest: Coord(Y, X) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.7209609302341821;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(TamMoveStepsDuringFormer { src: Coord(O, N), step: Coord(O, T), first_dest: Coord(U, T), second_dest: Coord(O, N) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.25654800794883714;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcDst { src: Coord(IA, N), dest: Coord(IA, K) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.1007192926450532;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(O, X), step: Coord(Y, X), dest: Coord(AI, X) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.32092527848697305;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(O, Z), step: Coord(AI, C), dest: Coord(O, Z) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.640777457023201;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveFromHopZuo { color: Kok1, prof: Kauk2, dest: Coord(E, L) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.054848668802143785;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(TamMoveStepsDuringLatter { src: Coord(O, N), step: Coord(O, T), first_dest: Coord(U, N), second_dest: Coord(U, Z) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.9921719153246659;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(I, T), step: Coord(I, Z), dest: Coord(I, T) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.719048549615752;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(Y, X), step: Coord(U, X), planned_direction: Coord(E, X) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::InfAfterStep(m) => {
        let chooser = 0.6196787702745082;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: Some(Coord(E, X)) };
        let chooser2 = 0.7474656984914038;
        let hnr_state = apply_after_half_acceptance(&ext_state, aha_move, config)
            .unwrap()
            .choose_by_uniform_random_variable(chooser2)
            .0;
        
        hnr_state
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcDst { src: Coord(U, X), dest: Coord(Y, X) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.7059756403072532;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(TamMoveStepsDuringLatter { src: Coord(U, Z), step: Coord(O, Z), first_dest: Coord(U, X), second_dest: Coord(U, X) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.5004957473253999;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(I, Z), step: Coord(I, C), dest: Coord(I, X) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.6187553587416654;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(O, Z), step: Coord(AI, N), dest: Coord(O, Z) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.8966144384472907;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(E, T), step: Coord(I, T), dest: Coord(I, Z) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.14966016983337127;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(Y, L), step: Coord(Y, N), planned_direction: Coord(Y, L) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::InfAfterStep(m) => {
        let chooser = 0.8545221457950023;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: Some(Coord(Y, L)) };
        let chooser2 = 0.3666978900296243;
        let hnr_state = apply_after_half_acceptance(&ext_state, aha_move, config)
            .unwrap()
            .choose_by_uniform_random_variable(chooser2)
            .0;
        
        hnr_state
    },
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
        let chooser = 0.16914014587952353;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: None };
        let chooser2 = 0.5967176905428868;
        let hnr_state = apply_after_half_acceptance(&ext_state, aha_move, config)
            .unwrap()
            .choose_by_uniform_random_variable(chooser2)
            .0;
        
        hnr_state
    },
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(AU, M), step: Coord(AI, M), planned_direction: Coord(O, M) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::InfAfterStep(m) => {
        let chooser = 0.5353583521410793;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: Some(Coord(O, M)) };
        let chooser2 = 0.44135652236683043;
        let hnr_state = apply_after_half_acceptance(&ext_state, aha_move, config)
            .unwrap()
            .choose_by_uniform_random_variable(chooser2)
            .0;
        
        hnr_state
    },
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
let pure_move = Some(NormalMove(TamMoveStepsDuringLatter { src: Coord(U, X), step: Coord(E, T), first_dest: Coord(I, Z), second_dest: Coord(A, N) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.9112985541983804;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(AU, Z), step: Coord(AU, X), planned_direction: Coord(AU, M) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::InfAfterStep(m) => {
        let chooser = 0.965565875230283;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: None };
        let chooser2 = 0.5156813386744754;
        let hnr_state = apply_after_half_acceptance(&ext_state, aha_move, config)
            .unwrap()
            .choose_by_uniform_random_variable(chooser2)
            .0;
        
        hnr_state
    },
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
        let chooser: f64 = 0.5058358202794581;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(TamMoveNoStep { src: Coord(A, N), first_dest: Coord(A, T), second_dest: Coord(A, N) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.8974662444922987;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(E, T), step: Coord(I, T), dest: Coord(E, N) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.3264577792088791;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
        let chooser: f64 = 0.6600087721895966;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(E, L), step: Coord(E, N), dest: Coord(E, T) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.5424100385389921;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveFromHopZuo { color: Huok2, prof: Kua2, dest: Coord(IA, X) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.6147993483376921;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(TamMoveStepsDuringFormer { src: Coord(A, N), step: Coord(E, T), first_dest: Coord(A, N), second_dest: Coord(E, L) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.6329200191539389;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(Y, N), step: Coord(O, L), dest: Coord(O, N) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.675038783012578;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(TamMoveStepsDuringLatter { src: Coord(E, L), step: Coord(I, K), first_dest: Coord(I, L), second_dest: Coord(U, K) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.5766933268402072;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(AU, Z), step: Coord(AU, X), planned_direction: Coord(AU, M) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::InfAfterStep(m) => {
        let chooser = 0.5565731469009166;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: Some(Coord(AU, C)) };
        let chooser2 = 0.14114039546287738;
        let hnr_state = apply_after_half_acceptance(&ext_state, aha_move, config)
            .unwrap()
            .choose_by_uniform_random_variable(chooser2)
            .0;
        
        hnr_state
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(E, Z), step: Coord(E, T), dest: Coord(A, N) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.5784140304563722;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(IA, T), step: Coord(AU, T), dest: Coord(IA, T) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.39964297040414176;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcDst { src: Coord(U, L), dest: Coord(I, L) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.32649299008677546;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(TamMoveStepsDuringFormer { src: Coord(U, K), step: Coord(I, L), first_dest: Coord(U, N), second_dest: Coord(U, L) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.30440789136231783;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(E, M), step: Coord(I, M), planned_direction: Coord(O, M) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::InfAfterStep(m) => {
        let chooser = 0.459721833349529;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: Some(Coord(O, M)) };
        let chooser2 = 0.3160176194454385;
        let hnr_state = apply_after_half_acceptance(&ext_state, aha_move, config)
            .unwrap()
            .choose_by_uniform_random_variable(chooser2)
            .0;
        
        hnr_state
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(O, N), step: Coord(O, L), dest: Coord(O, K) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.8553665473679533;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveFromHopZuo { color: Kok1, prof: Gua2, dest: Coord(U, C) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.2668580049466176;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveFromHopZuo { color: Huok2, prof: Kaun1, dest: Coord(A, P) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.9861951009244918;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(TamMoveNoStep { src: Coord(U, L), first_dest: Coord(O, N), second_dest: Coord(U, N) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.3338076929586008;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(AU, C), step: Coord(AI, C), planned_direction: Coord(O, P) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::InfAfterStep(m) => {
        let chooser = 0.08302194279134778;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: Some(Coord(Y, M)) };
        let chooser2 = 0.8275759949541548;
        let hnr_state = apply_after_half_acceptance(&ext_state, aha_move, config)
            .unwrap()
            .choose_by_uniform_random_variable(chooser2)
            .0;
        
        hnr_state
    },
    _ => { panic!("Expected PureMove::InfAfterStep") }
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
let pure_move = Some(NormalMove(TamMoveStepsDuringFormer { src: Coord(U, N), step: Coord(I, T), first_dest: Coord(U, N), second_dest: Coord(O, N) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.5486700331615104;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(E, X), step: Coord(I, X), planned_direction: Coord(I, T) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::InfAfterStep(m) => {
        let chooser = 0.02523584200840734;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: None };
        let chooser2 = 0.4662745379138168;
        let hnr_state = apply_after_half_acceptance(&ext_state, aha_move, config)
            .unwrap()
            .choose_by_uniform_random_variable(chooser2)
            .0;
        
        hnr_state
    },
    _ => { panic!("Expected PureMove::InfAfterStep") }
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
let pure_move = Some(NormalMove(TamMoveStepsDuringLatter { src: Coord(O, N), step: Coord(I, N), first_dest: Coord(U, T), second_dest: Coord(U, L) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.3678743178718107;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(Y, M), step: Coord(O, M), planned_direction: Coord(O, C) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::InfAfterStep(m) => {
        let chooser = 0.2866889054446913;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: Some(Coord(O, C)) };
        let chooser2 = 0.26129344832272194;
        let hnr_state = apply_after_half_acceptance(&ext_state, aha_move, config)
            .unwrap()
            .choose_by_uniform_random_variable(chooser2)
            .0;
        
        hnr_state
    },
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
let pure_move = Some(NormalMove(TamMoveStepsDuringFormer { src: Coord(U, L), step: Coord(O, L), first_dest: Coord(O, N), second_dest: Coord(U, T) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.16615163881334905;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(O, T), step: Coord(O, L), planned_direction: Coord(I, L) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::InfAfterStep(m) => {
        let chooser = 0.6886065867641801;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: None };
        let chooser2 = 0.015158656747955468;
        let hnr_state = apply_after_half_acceptance(&ext_state, aha_move, config)
            .unwrap()
            .choose_by_uniform_random_variable(chooser2)
            .0;
        
        hnr_state
    },
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
let pure_move = Some(NormalMove(TamMoveStepsDuringFormer { src: Coord(U, T), step: Coord(O, T), first_dest: Coord(O, Z), second_dest: Coord(O, X) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.07185500193448091;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(IA, T), step: Coord(AU, T), dest: Coord(IA, T) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.49470397831679214;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(E, K), step: Coord(E, N), planned_direction: Coord(E, K) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::InfAfterStep(m) => {
        let chooser = 0.3439753401284149;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: None };
        let chooser2 = 0.6655049878298689;
        let hnr_state = apply_after_half_acceptance(&ext_state, aha_move, config)
            .unwrap()
            .choose_by_uniform_random_variable(chooser2)
            .0;
        
        hnr_state
    },
    _ => { panic!("Expected PureMove::InfAfterStep") }
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
let pure_move = Some(NormalMove(TamMoveStepsDuringLatter { src: Coord(O, X), step: Coord(Y, X), first_dest: Coord(Y, C), second_dest: Coord(Y, Z) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.6695126490851382;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(I, X), step: Coord(Y, X), planned_direction: Coord(U, X) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::InfAfterStep(m) => {
        let chooser = 0.5986755288908158;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: Some(Coord(O, X)) };
        let chooser2 = 0.9478412198602276;
        let hnr_state = apply_after_half_acceptance(&ext_state, aha_move, config)
            .unwrap()
            .choose_by_uniform_random_variable(chooser2)
            .0;
        
        hnr_state
    },
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
let pure_move = Some(NormalMove(TamMoveStepsDuringLatter { src: Coord(Y, Z), step: Coord(AI, T), first_dest: Coord(AI, Z), second_dest: Coord(Y, Z) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.7353239755958314;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(Y, X), step: Coord(Y, Z), dest: Coord(Y, T) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.4779656723008424;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(O, K), step: Coord(O, L), dest: Coord(Y, N) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.48290001976557917;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcDst { src: Coord(A, Z), dest: Coord(A, T) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.9037824429578298;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
        let chooser: f64 = 0.4729076717430678;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(U, C), step: Coord(O, C), planned_direction: Coord(Y, C) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::InfAfterStep(m) => {
        let chooser = 0.11048289141290468;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: None };
        let chooser2 = 0.156331494801361;
        let hnr_state = apply_after_half_acceptance(&ext_state, aha_move, config)
            .unwrap()
            .choose_by_uniform_random_variable(chooser2)
            .0;
        
        hnr_state
    },
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
let pure_move = Some(NormalMove(TamMoveStepsDuringFormer { src: Coord(U, N), step: Coord(O, T), first_dest: Coord(U, Z), second_dest: Coord(I, X) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.1697379380887868;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
        let chooser: f64 = 0.4307432543412836;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(E, X), step: Coord(I, C), planned_direction: Coord(U, C) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::InfAfterStep(m) => {
        let chooser = 0.7382346074749359;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: None };
        let chooser2 = 0.32800900006249056;
        let hnr_state = apply_after_half_acceptance(&ext_state, aha_move, config)
            .unwrap()
            .choose_by_uniform_random_variable(chooser2)
            .0;
        
        hnr_state
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(A, N), step: Coord(E, L), dest: Coord(A, K) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.6178668688869451;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcDst { src: Coord(E, X), dest: Coord(E, M) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.4862014245389924;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveFromHopZuo { color: Huok2, prof: Kaun1, dest: Coord(I, Z) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.15930268607152076;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(Y, L), step: Coord(Y, N), planned_direction: Coord(Y, T) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::InfAfterStep(m) => {
        let chooser = 0.03976956328815373;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: None };
        let chooser2 = 0.12099133127484418;
        let hnr_state = apply_after_half_acceptance(&ext_state, aha_move, config)
            .unwrap()
            .choose_by_uniform_random_variable(chooser2)
            .0;
        
        hnr_state
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(I, P), step: Coord(U, P), dest: Coord(I, P) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.7376993039952935;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(TamMoveStepsDuringFormer { src: Coord(I, X), step: Coord(I, Z), first_dest: Coord(U, Z), second_dest: Coord(U, X) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.5738745765901496;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(I, Z), step: Coord(A, C), dest: Coord(A, Z) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.31234760516931415;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcDst { src: Coord(IA, T), dest: Coord(IA, N) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.1786561386743697;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(E, N), step: Coord(I, T), dest: Coord(U, T) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.14704123557278836;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
        let chooser: f64 = 0.3685388465760082;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(TamMoveStepsDuringLatter { src: Coord(U, X), step: Coord(U, T), first_dest: Coord(U, Z), second_dest: Coord(I, Z) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.4196209897916413;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
        let chooser: f64 = 0.7128953656145668;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(TamMoveStepsDuringLatter { src: Coord(I, Z), step: Coord(A, T), first_dest: Coord(E, Z), second_dest: Coord(E, N) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.8717123334121311;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcStepDstFinite { src: Coord(AU, C), step: Coord(IA, X), dest: Coord(AU, Z) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.7218657875713382;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
        let chooser: f64 = 0.6128615793301753;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(TamMoveStepsDuringFormer { src: Coord(E, N), step: Coord(I, T), first_dest: Coord(E, Z), second_dest: Coord(I, X) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.5311459143401192;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(NonTamMoveSrcDst { src: Coord(O, M), dest: Coord(Y, M) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.079219151826042;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(InfAfterStep(message::InfAfterStep { src: Coord(E, M), step: Coord(E, T), planned_direction: Coord(E, N) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::InfAfterStep(m) => {
        let chooser = 0.7465816313692774;
        let ext_state = apply_inf_after_step(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0;
        let aha_move = AfterHalfAcceptance { dest: None };
        let chooser2 = 0.1317003082656928;
        let hnr_state = apply_after_half_acceptance(&ext_state, aha_move, config)
            .unwrap()
            .choose_by_uniform_random_variable(chooser2)
            .0;
        
        hnr_state
    },
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
let pure_move = Some(NormalMove(NonTamMoveFromHopZuo { color: Kok1, prof: Gua2, dest: Coord(U, M) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.3693264855651215;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
let pure_move = Some(NormalMove(TamMoveNoStep { src: Coord(I, X), first_dest: Coord(I, Z), second_dest: Coord(U, X) }));
if pure_move.is_none() { return; }
let pure_move = pure_move.unwrap();

if !(hop1zuo1_candidates.contains(&pure_move) || candidates.contains(&pure_move)) {
    println!("hop1zuo1_candidates: {:?}", hop1zuo1_candidates);
    println!("candidates: {:?}", candidates);
    println!("pure_move: {:?}", pure_move);
    panic!("pure_move not contained in the candidates");
}


let hnr_state = match pure_move {
    PureMove::NormalMove(m) => {
        let chooser: f64 = 0.9021680114055503;
        apply_normal_move(&state, m, config).unwrap().choose_by_uniform_random_variable(chooser).0
    },
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
