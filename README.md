# term_cursor

A pure-rust crate for manipulating the position of the terminal cursor!
Also allows for clearing the screen!

## Usage

```rust
    extern crate term_cursor as cursor;

    fn main() {
        // Clear the screen. Does not reset the cursor position!
        print!("{}", cursor::Clear);
        // Position the cursor at column 5 and row 10 and print "Hello world!".
        print!("{}Hello world!", cursor::Goto(5, 10));
        // Go up a line. Does not reset the column of the cursor!
        print!("{}I'm above", cursor::Up(1));

        // Let's do the same thing again, with the second API.
        cursor::clear().expect("Clear failed");
        cursor::set_cursor_pos(5, 10).expect("Setting the cursor position failed");
        print!("Hello world!");
        let (x, _y) = cursor::get_cursor_pos().expect("Getting the cursor position failed");
        cursor::set_cursor_pos(x, 9).expect("Set failed again");
        print!("I'm above");

        // To finish off the example, move the cursor down 2 lines.
        // That's where the command prompt will return once the program finishes.
        // We don't the command prompt to overprint our stuff!
        print!("{}", cursor::Goto(0, 12));
    }
```

## Caveats

The 2D coordinate system of term_cursor is in the range: `x in 0..WIDTH` and `y in 0..HEIGHT`, where `WIDTH` and `HEIGHT` are the dimensions of the terminal buffer in characters.

Positioning the cursor and printing text out of bounds are **UNDEFINED BEHAVIOUR!** Your text might wrap, negative indicies might get turned into positive indicies, or you program might just crash. It is completely platform dependent.

To cope with this, it is your duty to ensure that all drawing happens within bounds. A possible mitigation is to clamp all coordinates, which should eliminate most UB - this feature will come in a future release.