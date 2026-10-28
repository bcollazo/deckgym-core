use super::*;

#[test]
fn blues_provocation_requires_a_named_regular_pokemon_and_lets_its_player_choose() {
    for supporter in [CardId::C1126BluesProvocation, CardId::C1166BluesProvocation] {
        let mut g = board(vec![pokemon("A1 001"), pokemon("A1 003")]);
        play(&mut g, supporter);
        let (actor, actions) = g.get_state_clone().generate_possible_actions();
        assert_eq!(actor, 0);
        assert_eq!(actions.len(), 2);
        assert!(actions
            .iter()
            .all(|a| matches!(a.action, SimpleAction::Activate { player: 1, .. })));
        let choice = actions
            .into_iter()
            .find(|a| matches!(a.action, SimpleAction::Activate { in_play_idx: 2, .. }))
            .unwrap();
        g.apply_action(&choice);
        assert_eq!(g.get_state_clone().get_active(1).get_name(), "Charmander");
    }
}
#[test]
fn blues_provocation_rejects_ex_names_and_an_empty_opponent_bench() {
    for own in ["A1 001", "A1 004"] {
        let mut g = board(vec![pokemon(own)]);
        assert!(!offered(&mut g, CardId::C1126BluesProvocation));
    }
    let mut g = board(vec![pokemon("A1 003")]);
    let mut s = g.get_state_clone();
    s.in_play_pokemon[1][1] = None;
    s.in_play_pokemon[1][2] = None;
    g.set_state(s);
    assert!(!offered(&mut g, CardId::C1126BluesProvocation));
}
