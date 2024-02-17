# guess-the-number-rust
Guess the number game in Rust created to learn the ropes.

What better way to learn a new programming language than to, you guessed it (pun intended!), make a number guessing game!

## How to run
Ensure you have Rust and Cargo on your system.
Clone the repo and move into the directory and run the command:

```
cargo run
```

## Usage
The program will ask you to enter a number as a guess

```
Please input your guess.
<Enter your guess here>
```

If your guess is too big, the code will inform you and ask you to input another number.
If your guess is too small, the code will inform you and ask you to input another number.

Once you guess the correct number, the code will display

```
after {number of guesses you took to guess correctly} guesses,
You win!
```

The program will quit. You can play again using ```cargo run``` command

