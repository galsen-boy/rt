build:
	cargo +nightly build --release

bench: build
	time cargo +nightly run --release -- -c config.ron

run: build
	cargo +nightly run --release -- -c config.ron > example.ppm
	ffmpeg -y -i example.ppm example.png
clean:
	cargo clean
	rm example.ppm example.png

