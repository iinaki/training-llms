# Establishing System Safety with Panic and Unrecoverable Errors in Rust

## Use Case

In a global banking firm, the application monitoring system was frequently reporting system crashes leading to misinformation and loss of transactions. Developers use Rust's capabilities of panics as safety precautions to quit the current thread immediately when the system encounters situations that it doesn't know how to handle. They also manage unrecoverable errors where if the main function returns the error value, the program will automatically clean up, print the error and its backtrace, and then quit. This ensures the system is inherently secure and there's less probability for system failures.

