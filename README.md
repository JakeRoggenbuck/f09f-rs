# f09f-rs ![Rust](https://img.shields.io/github/workflow/status/jakeroggenbuck/f09f-rs/Rust?style=for-the-badge)
 pronounced fonf; is a statically typed language with simple but powerful syntax, and an interpreter written in rust 

# Install
```
cargo install --path .
```

# Usage
| Shorthand | Flag        | Description                                                |
|-----------|-------------|------------------------------------------------------------|
| -v        |             | Verbose output of tokens                                   |

## Syntax
### Vars
```rb
int fact = 0;
prec dec = 0.2;
string this = "a string";
bool is_it = true;
```

### Functions
```rb
fun square(int n) returns int {
	~ Return the square of n ~
	return n ^ n;
}
```

### Control flow
```rb
if (n == 0) { n = 1; }

if (this and not that) {
	x = 1;
}

if (this or that) {
	x = 1;
}

if (this) {
	that = false;
} else {
	that = true;
}
```
