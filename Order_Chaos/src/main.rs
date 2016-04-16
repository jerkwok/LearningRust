extern crate rand;

use std::io;
use std::cmp::*;
use rand::Rng;


fn displayboard(board: [u8;36]){
	print!(" ");
	for x in 0..6{
		print!("|{}|",x);
	}
	println!("");

	for x in 0..6{
		print!("{:?}",x);
		for y in 0..6{
			print!("|{}|",board[x*6+y]);
	    }
	    println!("");
	}
    println!("");
}

fn displayturn(turn:u8){
	println!("Current Turn:");
    if turn % 2 == 1 {
	    println!("Player One");
    }else{
	    println!("Player Two");
    }
    println!("");
}

fn checkwinner(board: [u8;36]) -> u8{
	return 1;
}

fn valiadateinput(col:&str, row:&str, sym:&str){

}

fn main() {
	let mut board = [0u8; 36];
	let mut turn:u8;
	for x in 0..6{
		for y in 0..6{
	    	board[x * 6 + y] = 0;
	    }
	}

	displayboard(board);

    println!("Welcome to Order//Chaos.");

    let mut turn = rand::thread_rng().gen_range(1,3);


    loop{
	    let mut pmove = String::new();

	    displayturn(turn);

    	println!("Enter your move:");
    	println!("[Column] [Row] [1|2]");
    	println!("Example: 2 5 1");
    	io::stdin().read_line(&mut pmove)
    		.expect("Failed to read line");
    	let pmovecol = &pmove[0..1];
    	let pmoverow = &pmove[2..3];
    	let pmovesym = &pmove[4..5];

    	// println!("{}", pmovecol);
    	// println!("{}", pmoverow);
    	// println!("{}", pmovesym);

    	valiadateinput(pmovecol,pmoverow, pmovesym);

    	if board[(pmoverow.parse::<u8>().unwrap()*6 + pmovecol.parse::<u8>().unwrap()) as usize] == 0{
	    	board[(pmoverow.parse::<u8>().unwrap()*6 + pmovecol.parse::<u8>().unwrap()) as usize]=pmovesym.parse::<u8>().unwrap();

	    	if (checkwinner(board) == 1){
	    		println!("board 1");
	    	}

	    	turn+=1;
	    	displayboard(board);
    	}else{
    		println!("Spot taken");
    	}

    }
}