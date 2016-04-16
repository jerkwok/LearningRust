extern crate rand;

use std::io;
//use std::cmp::*;
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
	// Is there a mathematical way to do this!?

	//check Chaos
	let mut chaoswin = 0;
	for x in 0..6{
		for y in 0..6{
	    	if board[x * 6 + y] != 0{
	    		// println!("{:?}",chaoswin );
	    		chaoswin = chaoswin + 1;
	    	}
	    }
	}

	//Chaos victory
	if chaoswin == 36{
		return 2
	}

	//check Order
	//check horizaontals
	for x in 0..6{
		for y in 0..2{
			if board[x * 6 + y] != 0 &&
				board[x * 6 + (y + 0)] == board[x * 6 + (y + 1)] &&
				board[x * 6 + (y + 1)] == board[x * 6 + (y + 2)] &&
				board[x * 6 + (y + 2)] == board[x * 6 + (y + 3)] &&
				board[x * 6 + (y + 3)] == board[x * 6 + (y + 4)]
				{
					return 1;
			}
		}
	}

	//check verticals
	for y in 0..6{
		for x in 0..2{
			if board[x * 6 + y] != 0 &&
				board[(x + 0) * 6 + y] == board[(x + 1) * 6 + y] &&
				board[(x + 1) * 6 + y] == board[(x + 2) * 6 + y] &&
				board[(x + 2) * 6 + y] == board[(x + 3) * 6 + y] &&
				board[(x + 3) * 6 + y] == board[(x + 4) * 6 + y]
				{
					return 1;
			}
		}
	}

	//check \ diagonals
	for y in 0..2{
		for x in 0..2{
			if board[x * 6 + y] != 0 &&
				board[(x + 0) * 6 + (y + 0)] == board[(x + 1) * 6 + (y + 1)] &&
				board[(x + 1) * 6 + (y + 1)] == board[(x + 2) * 6 + (y + 2)] &&
				board[(x + 2) * 6 + (y + 2)] == board[(x + 3) * 6 + (y + 3)] &&
				board[(x + 3) * 6 + (y + 3)] == board[(x + 4) * 6 + (y + 4)]
				{
					return 1;
			}
		}
	}

	//check / diagonals
	for y in 4..6{
		for x in 0..2{
			if board[x * 6 + y] != 0 &&
				board[(x + 0) * 6 + (y - 0)] == board[(x + 1) * 6 + (y - 1)] &&
				board[(x + 1) * 6 + (y - 1)] == board[(x + 2) * 6 + (y - 2)] &&
				board[(x + 2) * 6 + (y - 2)] == board[(x + 3) * 6 + (y - 3)] &&
				board[(x + 3) * 6 + (y - 3)] == board[(x + 4) * 6 + (y - 4)]
				{
					return 1;
			}
		}
	}

	return 0;
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

    	if board[(pmoverow.parse::<u8>().unwrap()*6 + pmovecol.parse::<u8>().unwrap()) as usize] == 0{
	    	board[(pmoverow.parse::<u8>().unwrap()*6 + pmovecol.parse::<u8>().unwrap()) as usize]=pmovesym.parse::<u8>().unwrap();

	    	let winner = checkwinner(board);
	    	// println!("winner: {:?}",winner );
	    	if winner == 1{
	    		println!("Order Wins!");
	    		break;
	    	}else if winner == 2{
	    		println!("Chaos Wins!");
	    		break;
	    	}

	    	turn+=1;
	    	displayboard(board);
    	}else{
    		println!("Spot taken");
    	}

    }
}