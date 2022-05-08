use crate::GameState;

#[test]
fn it_works() {
    let result = 2 + 2;
    assert_eq!(result, 4);
}

enum RPSAction {
    Rock,
    Paper,
    Scissors,
}
struct RPSState;

impl GameState<RPSAction, usize, usize> for RPSState {
    fn advance(self, action: RPSAction) -> Self {
        todo!()
    }

    fn is_terminal(&self) -> bool {
        todo!()
    }

    fn actions(&self) -> Box<dyn Iterator<Item = RPSAction>> {
        todo!()
    }

    fn information_set(&self) -> &usize {
        todo!()
    }

    fn player(&self) -> &usize {
        todo!()
    }
}
