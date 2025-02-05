This illustrates a very minimal environment for executing pulley. Run the compile binary to generate `guest.cwasm`

Removing panic strips it down to 540kB:
`cargo +nightly build --release -Z build-std=std,panic_abort -Z build-std-features=panic_immediate_abort`
