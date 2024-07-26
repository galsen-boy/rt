build:
	cargo +nightly build --release

bench: build
	time cargo +nightly run --release -- -c config.ron

run: build
	cargo +nightly run --release -- -c config.ron > example.ppm
	ffmpeg -y -i example.ppm example.png
	python3 zoom_and_reset.py
clean:
	cargo clean
	rm example.ppm example.png example_zoomed.png example_reset.png

