use rchess::board::Board;

fn main() {
    let mut board = Board::new();

    board.print();

    // e4

    let e2 = board.square(2, 5);

    let plys = e2.valid_plys(&board);

    board.make_ply(plys[1]);

    board.print();

    // d5

    let d7 = board.square(7, 4);

    let plys = d7.valid_plys(&board);

    board.make_ply(plys[1]);

    board.print();

    // exd5

    let e4 = board.square(4, 5);

    let plys = e4.valid_plys(&board);

    board.make_ply(plys[1]);

    board.print();

    // Ke2

    let e1 = board.square(1, 5);

    let plys = e1.valid_plys(&board);

    assert_eq!(plys.len(), 1);

    board.make_ply(plys[0]);

    board.print();

    // Kd3

    let e2 = board.square(2, 5);

    let plys = e2.valid_plys(&board);

    assert_eq!(plys.len(), 4);

    board.make_ply(plys[3]);

    board.print();
}
